use crate::model::following::FollowingWithFollower;
use crate::model::user::UserId;

use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum Request {
    #[serde(rename_all = "camelCase")]
    WithUserId {
        user_id: UserId,
        /// 1 .. 100
        #[serde(skip_serializing_if = "Option::is_none")]
        limit: Option<u8>,
        #[serde(skip_serializing_if = "Option::is_none")]
        since_id: Option<UserId>,
        #[serde(skip_serializing_if = "Option::is_none")]
        until_id: Option<UserId>,
    },
    #[serde(rename_all = "camelCase")]
    WithUsername {
        username: String,
        host: Option<String>,
        /// 1 .. 100
        #[serde(skip_serializing_if = "Option::is_none")]
        limit: Option<u8>,
        #[serde(skip_serializing_if = "Option::is_none")]
        since_id: Option<UserId>,
        #[serde(skip_serializing_if = "Option::is_none")]
        until_id: Option<UserId>,
    },
}

impl misskey_core::Request for Request {
    type Response = Vec<FollowingWithFollower>;
    const ENDPOINT: &'static str = "users/followers";
}

impl misskey_core::PaginationRequest for Request {
    type Item = FollowingWithFollower;

    fn set_since(&mut self, item: &FollowingWithFollower) {
        let id = item.follower_id.clone();
        match self {
            Request::WithUserId { since_id, .. } => since_id.replace(id),
            Request::WithUsername { since_id, .. } => since_id.replace(id),
        };
    }

    fn set_until(&mut self, item: &FollowingWithFollower) {
        let id = item.follower_id.clone();
        match self {
            Request::WithUserId { until_id, .. } => until_id.replace(id),
            Request::WithUsername { until_id, .. } => until_id.replace(id),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::Request;
    use crate::test::{ClientExt, TestClient};

    #[tokio::test]
    async fn request_with_id() {
        let client = TestClient::new();
        let user = client.me().await;

        client
            .test(Request::WithUserId {
                user_id: user.id,
                limit: None,
                since_id: None,
                until_id: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_with_username() {
        let client = TestClient::new();
        let user = client.me().await;

        client
            .test(Request::WithUsername {
                username: user.username,
                host: None,
                limit: None,
                since_id: None,
                until_id: None,
            })
            .await;
    }

    // TODO: request_with_username_and_host

    #[tokio::test]
    async fn request_with_limit() {
        let client = TestClient::new();
        let user = client.me().await;
        client
            .test(Request::WithUserId {
                user_id: user.id,
                limit: Some(100),
                since_id: None,
                until_id: None,
            })
            .await;
    }

    #[tokio::test]
    async fn request_paginate() {
        let client = TestClient::new();
        let user = client.user.me().await;
        let (new_user, new_user_client) = client.admin.create_user().await;
        new_user_client
            .test(crate::endpoint::following::create::Request {
                user_id: user.id.clone(),
            })
            .await;

        client
            .user
            .test(Request::WithUserId {
                user_id: user.id,
                limit: None,
                since_id: Some(new_user.id.clone()),
                until_id: Some(new_user.id.clone()),
            })
            .await;
    }
}
