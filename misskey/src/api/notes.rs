use crate::api::ApiRequest;
use crate::model::note::{Note, NoteId};

use serde::Serialize;

pub mod children;
pub mod conversation;
pub mod create;
pub mod delete;
pub mod favorites;
pub mod featured;
pub mod global_timeline;
pub mod hybrid_timeline;
pub mod local_timeline;
pub mod mentions;
pub mod polls;
pub mod reactions;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub local: bool,
    pub reply: bool,
    pub renote: bool,
    pub with_files: bool,
    pub poll: bool,
    /// 1 .. 100, default: 10
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_id: Option<NoteId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_id: Option<NoteId>,
}

impl ApiRequest for Request {
    type Response = Vec<Note>;
    const ENDPOINT: &'static str = "notes";
}