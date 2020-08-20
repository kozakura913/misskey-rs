use crate::model::user::{User, UserId};

use misskey_core::ApiRequest;
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub user_id: UserId,
}

impl ApiRequest for Request {
    type Response = User;
    const ENDPOINT: &'static str = "blocking/delete";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let mut client = TestClient::new();
        let (user, _) = client.admin.create_user().await;
        client
            .user
            .test(crate::api::blocking::create::Request {
                user_id: user.id.clone(),
            })
            .await;

        client.user.test(Request { user_id: user.id }).await;
    }
}
