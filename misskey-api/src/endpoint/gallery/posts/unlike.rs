use crate::model::{gallery::GalleryPost, id::Id};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub post_id: Id<GalleryPost>,
}

impl misskey_core::Request for Request {
    type Response = ();
    const ENDPOINT: &'static str = "gallery/posts/unlike";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        let url = client.avatar_url().await;
        let file = client.upload_from_url(url).await;

        let post = client
            .user
            .test(crate::endpoint::gallery::posts::create::Request {
                title: "gallery post".to_string(),
                description: None,
                file_ids: vec![file.id],
                is_sensitive: None,
            })
            .await;

        client
            .admin
            .test(crate::endpoint::gallery::posts::like::Request { post_id: post.id })
            .await;

        client.admin.test(Request { post_id: post.id }).await;
    }
}
