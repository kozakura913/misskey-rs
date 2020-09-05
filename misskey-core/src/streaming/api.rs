use derive_more::{Display, FromStr};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, FromStr, Debug, Display)]
#[serde(transparent)]
pub struct SubNoteId(pub String);

pub trait ConnectChannelRequest: Serialize {
    type Incoming: DeserializeOwned;
    type Outgoing: Serialize;

    const NAME: &'static str;
}

pub trait SubNoteEvent: DeserializeOwned {}

pub trait BroadcastEvent: DeserializeOwned {
    const TYPE: &'static str;
}
