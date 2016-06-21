//! Event types.

pub mod call;
pub mod presence;
pub mod receipt;
pub mod room;
pub mod tag;
pub mod typing;

use self::room::avatar::AvatarEventContent;
use self::room::canonical_alias::CanonicalAliasEventContent;
use self::room::join_rules::JoinRulesEventContent;
use self::room::name::NameEventContent;

/// The type of an event.
#[derive(Debug, Deserialize, Serialize)]
pub enum EventType {
    CallAnswer,
    CallCandidates,
    CallHangup,
    CallInvite,
    Presence,
    Receipt,
    RoomAliases,
    RoomAvatar,
    RoomCanonicalAlias,
    RoomCreate,
    RoomGuestAccess,
    RoomHistoryVisibility,
    RoomJoinRules,
    RoomMember,
    RoomMessage,
    RoomMessageFeedback,
    RoomName,
    RoomPowerLevels,
    RoomRedaction,
    RoomThirdPartyInvite,
    RoomTopic,
    Tag,
    Typing,
}

/// A stripped-down version of a state event that is included along with some other events.
#[derive(Debug, Deserialize, Serialize)]
pub enum StrippedState {
    RoomAvatar(StrippedRoomAvatar),
    RoomCanonicalAlias(StrippedRoomCanonicalAlias),
    RoomJoinRules(StrippedRoomJoinRules),
    RoomName(StrippedRoomName),
}

/// The general form of a `StrippedState`.
#[derive(Debug, Deserialize, Serialize)]
pub struct StrippedStateContent<T> {
    pub content: T,
    pub event_type: EventType,
    pub state_key: String,
}

pub type StrippedRoomAvatar = StrippedStateContent<AvatarEventContent>;
pub type StrippedRoomCanonicalAlias = StrippedStateContent<CanonicalAliasEventContent>;
pub type StrippedRoomJoinRules = StrippedStateContent<JoinRulesEventContent>;
pub type StrippedRoomName = StrippedStateContent<NameEventContent>;