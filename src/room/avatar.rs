//! Types for the *m.room.avatar* event.

use StateEvent;
use super::ImageInfo;

/// A picture that is associated with the room.
///
/// This can be displayed alongside the room information.
pub type AvatarEvent = StateEvent<AvatarEventContent, ()>;

/// The payload of an `AvatarEvent`.
#[derive(Debug, Deserialize, Serialize)]
pub struct AvatarEventContent {
    /// Information about the avatar image.
    pub info: ImageInfo,
    /// Information about the avatar thumbnail image.
    pub thumbnail_info: ImageInfo,
    /// URL of the avatar thumbnail image.
    pub thumbnail_url: String,
    /// URL of the avatar image.
    pub url: String,
}