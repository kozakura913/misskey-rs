use crate::model::{
    user::UserId,
    user_group::{UserGroup, UserGroupId},
};

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub group_id: UserGroupId,
    pub user_id: UserId,
}

impl misskey_core::Request for Request {
    type Response = UserGroup;
    const ENDPOINT: &'static str = "users/groups/transfer";
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request() {
        let mut client = TestClient::new();
        let (new_user, mut new_user_client) = client.admin.create_user().await;
        let group = client
            .test(crate::api::users::groups::create::Request {
                name: "test".to_string(),
            })
            .await;
        client
            .test(crate::api::users::groups::invite::Request {
                group_id: group.id.clone(),
                user_id: new_user.id.clone(),
            })
            .await;
        let invitation = new_user_client
            .test(crate::api::i::user_group_invites::Request {
                limit: None,
                since_id: None,
                until_id: None,
            })
            .await
            .pop()
            .unwrap();
        new_user_client
            .test(crate::api::users::groups::invitations::accept::Request {
                invitation_id: invitation.id,
            })
            .await;

        client
            .test(Request {
                group_id: group.id,
                user_id: new_user.id,
            })
            .await;
    }
}
