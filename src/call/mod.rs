//! Modules for events in the *m.call* namespace.
//!
//! This module also contains types shared by events in its child namespaces.

use std::fmt::{Display, Formatter, Error as FmtError};
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use {ParseError, Visitor};

pub mod answer;
pub mod candidates;
pub mod hangup;
pub mod invite;

/// A VoIP session description.
#[derive(Debug, Deserialize, Serialize)]
pub struct SessionDescription {
    /// The type of session description.
    pub session_type: SessionDescriptionType,
    /// The SDP text of the session description.
    pub sdp: String,
}

/// The type of VoIP session description.
#[derive(Debug, PartialEq)]
pub enum SessionDescriptionType {
    /// An answer.
    Answer,
    /// An offer.
    Offer,
}

impl Display for SessionDescriptionType {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        let session_description_type_str = match *self {
            SessionDescriptionType::Answer => "answer",
            SessionDescriptionType::Offer => "offer",
        };

        write!(f, "{}", session_description_type_str)
    }
}

impl FromStr for SessionDescriptionType {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "answer" => Ok(SessionDescriptionType::Answer),
            "offer" => Ok(SessionDescriptionType::Offer),
            _ => Err(ParseError),
        }
    }
}

impl Serialize for SessionDescriptionType {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where S: Serializer {
        serializer.serialize_str(&self.to_string())
    }
}

impl Deserialize for SessionDescriptionType {
    fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error> where D: Deserializer {
        deserializer.deserialize_str(Visitor::new())
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{from_str, to_string};

    use super::SessionDescriptionType;

    #[test]
    fn session_description_types_serialize_to_display_form() {
        assert_eq!(
            to_string(&SessionDescriptionType::Answer).unwrap(),
            r#""answer""#
        );
    }

    #[test]
    fn session_description_types_deserialize_from_display_form() {
        assert_eq!(
            from_str::<SessionDescriptionType>(r#""answer""#).unwrap(),
            SessionDescriptionType::Answer
        );
    }

    #[test]
    fn invalid_session_description_types_fail_deserialization() {
        assert!(from_str::<SessionDescriptionType>(r#""bad""#).is_err());
    }
}