//! API endpoints.
//!
//! Each endpoint is implemented under modules named by replacing `/` with `::` and `-` with `_` in the endpoint name.
//! For example, `notes/local-timeline` is implemented under [`notes::local_timeline`] and
//! `drive/files/create` is implemented under [`drive::files::create`].
//!
//! All request types implements [`Request`][`misskey_core::Request`].
//! We dispatch it actually and get the [response][`misskey_core::Request::Response`]
//! using [`Client::request`][`misskey_core::Client::request`].

macro_rules! impl_pagination {
    ($name:ident, $item:ty) => {
        impl ::misskey_core::PaginationRequest for $name {
            type Item = $item;
            fn set_since(&mut self, item: &$item) {
                self.since_id
                    .replace(::misskey_core::model::Entity::id(item));
            }
            fn set_until(&mut self, item: &$item) {
                self.until_id
                    .replace(::misskey_core::model::Entity::id(item));
            }
        }
    };
}

macro_rules! impl_offset_pagination {
    ($name:ident, $item:ty) => {
        impl ::misskey_core::OffsetPaginationRequest for $name {
            type Item = $item;
            fn set_offset(&mut self, offset: u64) {
                self.offset.replace(offset);
            }
        }
    };
}

pub mod admin;
pub mod announcements;
pub mod antennas;
pub mod blocking;
pub mod charts;
pub mod clips;
pub mod drive;
#[allow(clippy::module_inception)]
pub mod endpoint;
pub mod endpoints;
pub mod following;
pub mod i;
pub mod messaging;
pub mod meta;
pub mod mute;
pub mod notes;
pub mod notifications;
pub mod pinned_users;
pub mod stats;
pub mod username;
pub mod users;

#[cfg(feature = "12-47-0")]
#[cfg_attr(docsrs, doc(cfg(feature = "12-47-0")))]
pub mod channels;
