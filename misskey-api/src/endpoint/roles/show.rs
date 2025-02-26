use crate::model::{id::Id, role::Role};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub role_id: Id<Role>,
}

impl misskey_core::Request for Request {
    type Response = Role;
    const ENDPOINT: &'static str = "roles/show";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let client = TestClient::new();
        let role = client
            .admin
            .test(
                crate::endpoint::admin::roles::create::Request::builder()
                    .is_public(true)
                    .build(),
            )
            .await;

        client.test(Request { role_id: role.id }).await;
    }
}
