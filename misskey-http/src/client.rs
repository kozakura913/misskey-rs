use std::fmt::{self, Debug};

use crate::error::{Error, Result};

use common_multipart_rfc7578::client::multipart;
use futures_util::future::BoxFuture;
use futures_util::io::AsyncReadExt;
#[cfg(feature="isahc-client")]
use isahc::http;
#[cfg(feature = "inspect-contents")]
use log::debug;
use mime::Mime;
use misskey_core::model::ApiResult;
use misskey_core::{Client, Request, UploadFileClient, UploadFileRequest};
use serde::Serialize;
use serde_json::value::{self, Value};
use url::Url;

pub mod builder;

use builder::HttpClientBuilder;

/// Asynchronous HTTP-based client for Misskey.
///
/// [`HttpClient`] can be constructed using [`HttpClient::new`], [`HttpClient::with_token`] or
/// [`HttpClientBuilder`][`builder::HttpClientBuilder`].
pub struct HttpClient {
    url: Url,
    token: Option<String>,
	#[cfg(feature="reqwest-client")]
    client: reqwest::Client,
	#[cfg(feature="isahc-client")]
    client: isahc::HttpClient,
}

impl Debug for HttpClient {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("HttpClient")
            .field("url", &self.url)
            .finish()
    }
}

impl HttpClient {
    /// Creates a new HTTP-based client without a token.
    pub fn new(url: Url) -> Result<Self> {
        Ok(HttpClient {
            url,
            token: None,
			#[cfg(feature="reqwest-client")]
            client: reqwest::ClientBuilder::new().build()?,
			#[cfg(feature="isahc-client")]
            client: isahc::HttpClient::new()?,
        })
    }

    /// Creates a new HTTP-based client with a token.
    pub fn with_token(url: Url, token: impl Into<String>) -> Result<Self> {
        Ok(HttpClient {
            url,
            token: Some(token.into()),
			#[cfg(feature="reqwest-client")]
            client: reqwest::ClientBuilder::new().build()?,
			#[cfg(feature="isahc-client")]
            client: isahc::HttpClient::new()?,
        })
    }

    /// Creates a new builder instance with `url`.
    /// All configurations are set to default.
    ///
    /// This function is identical to [`HttpClientBuilder::new`].
    pub fn builder<T>(url: T) -> HttpClientBuilder
    where
        T: TryInto<Url>,
        T::Error: Into<Error>,
    {
        HttpClientBuilder::new(url)
    }

    fn set_api_key<R: Request>(
        &self,
        request: R,
    ) -> std::result::Result<impl Serialize, serde_json::Error> {
        #[derive(Serialize)]
        #[serde(untagged)]
        enum ValueOrRequest<R> {
            Value(Value),
            Request(R),
        }

        if let Some(token) = &self.token {
            let mut value = value::to_value(request)?;

            let obj = value.as_object_mut().expect("Request must be an object");
            if obj
                .insert("i".to_string(), Value::String(token.to_owned()))
                .is_some()
            {
                panic!("Request must not have 'i' key");
            }

            Ok(ValueOrRequest::Value(value))
        } else {
            Ok(ValueOrRequest::Request(request))
        }
    }

    fn make_url<R: Request>(&self) -> Result<Url> {
        let mut url = self.url.clone();
        if let Ok(mut segments) = url.path_segments_mut() {
            segments.pop_if_empty();
            for segment in R::ENDPOINT.split('/') {
                segments.push(segment);
            }
        } else {
            return self.url.join(R::ENDPOINT).map_err(Into::into);
        }
        Ok(url)
    }
}

impl Client for HttpClient {
    type Error = Error;

    fn request<R: Request>(&self, request: R) -> BoxFuture<Result<ApiResult<R::Response>>> {
        let url = self.make_url::<R>();

        // limit the use of `R` value to the outside of `async`
        // in order not to require `Send` on `R`
        let body = self
            .set_api_key(request)
            .and_then(|b| serde_json::to_vec(&b));

        Box::pin(async move {
            let url = url?;
            let body = body?;

            #[cfg(feature = "inspect-contents")]
            debug!(
                "sending request to {}: {}",
                url,
                String::from_utf8_lossy(&body)
            );
			#[cfg(feature="isahc-client")]
            use isahc::http::header::CONTENT_TYPE;
			#[cfg(feature="isahc-client")]
            let response = self
                .client
                .send_async(
                    // TODO: uncomfortable conversion from `Url` to `Uri`
                    http::Request::post(url.to_string())
                        .header(CONTENT_TYPE, "application/json")
                        .body(body)
                        .unwrap(),
                )
                .await?;
			#[cfg(feature="reqwest-client")]
			let response=self
				.client
				.post(url)
				.header(reqwest::header::CONTENT_TYPE,"application/json")
				.body(body)
				.send().await?;
            response_to_result::<R>(response).await
        })
    }
}

impl UploadFileClient for HttpClient {
    fn request_with_file<R, T>(
        &self,
        request: R,
        type_: Mime,
        file_name: String,
        read: T,
    ) -> BoxFuture<Result<ApiResult<R::Response>>>
    where
        R: UploadFileRequest,
        T: std::io::Read + Send + Sync + 'static,
    {
        let url = self.make_url::<R>();

        // limit the use of `R` value to the outside of `async`
        // in order not to require `Send` on `R`
        let value = self.set_api_key(request).and_then(value::to_value);

        Box::pin(async move {
            let url = url?;
            let value = value?;

            #[cfg(feature = "inspect-contents")]
            debug!(
                "sending request to {} with {} content: {}",
                url, type_, value
            );

            let mut form = multipart::Form::default();

            form.add_reader_file_with_mime("file", Box::new(read), file_name, type_);

            let obj = value.as_object().expect("Request must be an object");
            for (k, v) in obj {
                let v = v
                    .as_str()
                    .expect("UploadFileRequest must be an object that all values are string");
                form.add_text(k.to_owned(), v.to_owned());
            }

            let content_type = form.content_type();

			#[cfg(feature="isahc-client")]
            use futures_util::stream::TryStreamExt;
			#[cfg(feature="isahc-client")]
            let stream = multipart::Body::from(form).map_err(Into::into);
			#[cfg(feature="isahc-client")]
            let body =
                isahc::AsyncBody::from_reader(async_dup::Mutex::new(stream.into_async_read()));

			#[cfg(feature="reqwest-client")]
			let body=reqwest::Body::wrap_stream(multipart::Body::from(form));
			#[cfg(feature="isahc-client")]
            use isahc::http::header::CONTENT_TYPE;
			#[cfg(feature="isahc-client")]
            let response = self
                .client
                .send_async(
                    // TODO: uncomfortable conversion from `Url` to `Uri`
                    http::Request::post(String::from(url))
                        .header(CONTENT_TYPE, content_type)
                        .body(body)
                        .unwrap(),
                )
                .await?;
			#[cfg(feature="reqwest-client")]
			let response=self
				.client
				.post(url)
				.header(reqwest::header::CONTENT_TYPE,content_type)
				.body(body)
				.send().await?;

            response_to_result::<R>(response).await
        })
    }
}

#[cfg(feature="reqwest-client")]
async fn response_to_result<R: Request>(
    response: reqwest::Response,
) -> Result<ApiResult<R::Response>> {
    let status = response.status();
	let bytes=response.bytes().await?;
    let json_bytes = if bytes.is_empty() {
        b"null".as_ref()
    } else {
        bytes.as_ref()
    };

    if status.is_success() {
        // Limit response to `ApiResult::Ok` branch to get informative error message
        // when our model does not match the response.
        Ok(ApiResult::Ok(serde_json::from_slice(json_bytes)?))
    } else {
        Ok(serde_json::from_slice(json_bytes)?)
    }
}

#[cfg(feature="isahc-client")]
async fn response_to_result<R: Request>(
    response: http::Response<isahc::AsyncBody>,
) -> Result<ApiResult<R::Response>> {
    let status = response.status();
    let mut bytes = Vec::new();
    response.into_body().read_to_end(&mut bytes).await?;

    #[cfg(feature = "inspect-contents")]
    debug!(
        "got response ({}) from {}: {}",
        status,
        R::ENDPOINT,
        String::from_utf8_lossy(&bytes)
    );

    let json_bytes = if bytes.is_empty() {
        b"null".as_ref()
    } else {
        bytes.as_ref()
    };

    if status.is_success() {
        // Limit response to `ApiResult::Ok` branch to get informative error message
        // when our model does not match the response.
        Ok(ApiResult::Ok(serde_json::from_slice(json_bytes)?))
    } else {
        Ok(serde_json::from_slice(json_bytes)?)
    }
}

#[cfg(test)]
mod tests {
    use super::HttpClient;

    use misskey_core::{Client, UploadFileClient};
    use misskey_test::{self, env};
    use uuid::Uuid;

    fn test_client() -> HttpClient {
        misskey_test::init_logger();
        HttpClient::with_token(env::api_url(), env::token()).unwrap()
    }

    #[test]
    fn test_send() {
        fn assert_send<T: Send>() {}
        assert_send::<HttpClient>();
    }

    #[test]
    fn test_sync() {
        fn assert_send<T: Sync>() {}
        assert_send::<HttpClient>();
    }

    #[tokio::test]
    async fn test_url_without_trailing_slash() {
        let mut url = env::api_url().to_string();
        assert_eq!(url.pop(), Some('/'));
        let client = HttpClient::with_token(url.parse().unwrap(), env::token()).unwrap();
        client
            .request(
                misskey_api::endpoint::notes::create::Request::builder()
                    .text("hi")
                    .build(),
            )
            .await
            .unwrap()
            .unwrap();
    }

    #[tokio::test]
    async fn tokio_request() {
        let client = test_client();
        client
            .request(
                misskey_api::endpoint::notes::create::Request::builder()
                    .text("hi")
                    .build(),
            )
            .await
            .unwrap()
            .unwrap();
    }

    #[async_std::test]
    async fn async_std_request() {
        let client = test_client();
        client
            .request(
                misskey_api::endpoint::notes::create::Request::builder()
                    .text("hi")
                    .build(),
            )
            .await
            .unwrap()
            .unwrap();
    }

    fn write_to_temp_file(data: impl AsRef<[u8]>) -> std::path::PathBuf {
        let tmp_name = Uuid::new_v4().simple().to_string();
        let path = std::env::temp_dir().join(tmp_name);
        {
            use std::{fs::File, io::Write};
            let mut file = File::create(&path).unwrap();
            file.write_all(data.as_ref()).unwrap();
            file.sync_all().unwrap();
        }
        path
    }

    #[tokio::test]
    async fn tokio_request_with_file() {
        let client = test_client();
        let path = write_to_temp_file("test");
        let file = std::fs::File::open(path).unwrap();

        client
            .request_with_file(
                misskey_api::endpoint::drive::files::create::Request::default(),
                mime::TEXT_PLAIN,
                "test.txt".to_string(),
                file,
            )
            .await
            .unwrap()
            .unwrap();
    }

    #[async_std::test]
    async fn async_std_request_with_file() {
        let client = test_client();
        let path = write_to_temp_file("test");
        let file = std::fs::File::open(path).unwrap();

        client
            .request_with_file(
                misskey_api::endpoint::drive::files::create::Request::default(),
                mime::TEXT_PLAIN,
                "test.txt".to_string(),
                file,
            )
            .await
            .unwrap()
            .unwrap();
    }
}
