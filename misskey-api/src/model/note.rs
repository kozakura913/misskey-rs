use std::collections::HashMap;

use crate::model::{
    channel::ChannelId,
    drive::{DriveFile, DriveFileId},
    user::{User, UserId},
};

use chrono::{DateTime, Utc};
use derive_more::{Display, Error, FromStr};
use misskey_core::streaming::SubNoteId;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, FromStr, Debug, Display)]
#[serde(transparent)]
pub struct NoteId(pub String);

/// in order to use as ID in [`streaming::note`](crate::streaming::note)
impl From<NoteId> for SubNoteId {
    fn from(id: NoteId) -> SubNoteId {
        SubNoteId(id.0)
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, FromStr, Debug)]
#[serde(transparent)]
pub struct Tag(pub String);

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, FromStr, Debug)]
#[serde(transparent)]
pub struct Reaction(pub String);

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Copy)]
#[serde(rename_all = "camelCase")]
pub enum Visibility {
    Public,
    Home,
    Followers,
    Specified,
}

#[derive(Debug, Display, Error, Clone)]
#[display(fmt = "invalid note visibility")]
pub struct ParseVisibilityError;

impl std::str::FromStr for Visibility {
    type Err = ParseVisibilityError;

    fn from_str(s: &str) -> Result<Visibility, Self::Err> {
        match s {
            "public" | "Public" => Ok(Visibility::Public),
            "home" | "Home" => Ok(Visibility::Home),
            "followers" | "Followers" => Ok(Visibility::Followers),
            "specified" | "Specified" => Ok(Visibility::Specified),
            _ => Err(ParseVisibilityError),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PollChoice {
    pub is_voted: bool,
    pub text: String,
    pub votes: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Poll {
    pub choices: Vec<PollChoice>,
    pub multiple: bool,
    pub expires_at: Option<DateTime<Utc>>,
    // pub expired_after: Option<DateTime<Utc>>,
}

// packed `Emoji` for `Note`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NoteEmoji {
    pub name: String,
    pub url: Url,
}

// packed `Channel` for `Note`
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NoteChannel {
    pub id: ChannelId,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    pub id: NoteId,
    pub created_at: DateTime<Utc>,
    pub text: Option<String>,
    #[serde(default)]
    pub cw: Option<String>,
    pub user_id: UserId,
    pub user: User,
    #[serde(default)]
    pub reply_id: Option<NoteId>,
    #[serde(default)]
    pub renote_id: Option<NoteId>,
    #[serde(default)]
    pub reply: Option<Box<Note>>,
    #[serde(default)]
    pub renote: Option<Box<Note>>,
    #[serde(default = "default_false")]
    pub via_mobile: bool,
    #[serde(default = "default_false")]
    pub is_hidden: bool,
    pub visibility: Visibility,
    #[serde(default)]
    pub mentions: Vec<UserId>,
    #[serde(default)]
    pub visible_user_ids: Vec<UserId>,
    #[serde(default)]
    pub file_ids: Vec<DriveFileId>,
    #[serde(default)]
    pub files: Vec<DriveFile>,
    #[serde(default)]
    pub tags: Vec<Tag>,
    #[serde(default)]
    pub poll: Option<Poll>,
    #[serde(default)]
    pub reactions: HashMap<Reaction, u64>,
    #[serde(default)]
    pub emojis: Vec<NoteEmoji>,
    pub renote_count: u64,
    pub replies_count: u64,
    #[cfg(feature = "12-47-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-47-0")))]
    pub channel_id: Option<ChannelId>,
    #[cfg(feature = "12-47-0")]
    #[cfg_attr(docsrs, doc(cfg(feature = "12-47-0")))]
    pub channel: Option<NoteChannel>,
}

fn default_false() -> bool {
    false
}
