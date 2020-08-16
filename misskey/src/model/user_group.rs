use crate::model::user::UserId;

use chrono::{DateTime, Utc};
use derivative::Derivative;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Derivative)]
#[derivative(Debug = "transparent")]
pub struct UserGroupId(pub String);

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", tag = "type")]
pub struct UserGroup {
    pub id: UserGroupId,
    pub created_at: DateTime<Utc>,
    pub name: String,
    pub owner_id: UserId,
    pub user_ids: Vec<UserId>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Derivative)]
#[derivative(Debug = "transparent")]
pub struct UserGroupInvitationId(pub String);

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase", tag = "type")]
pub struct UserGroupInvitation {
    pub id: UserGroupInvitationId,
    pub group: UserGroup,
}