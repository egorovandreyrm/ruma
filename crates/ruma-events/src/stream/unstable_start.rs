//! Types for the `org.matrix.msc3381.stream.start` event, the unstable version of `m.stream.start`.

use std::ops::Deref;

use ruma_macros::EventContent;
use serde::{Deserialize, Serialize};

mod content_serde;

use ruma_common::OwnedEventId;
use ruma_common::room_version_rules::RedactionRules;
use super::unstable_end::UnstableStreamEndEventContent;
use crate::{
    relation::Replacement, room::message::RelationWithoutReplacement,
    MessageLikeEventContent, MessageLikeEventType, RedactContent, RedactedMessageLikeEventContent,
    StaticEventContent,
};

/// The payload for an unstable stream start event.
///
/// This is the event content that should be sent for room versions that don't support extensible
/// events. As of Matrix 1.7, none of the stable room versions (1 through 10) support extensible
/// events.
///
/// To send a stream start event for a room version that supports extensible events, use
/// [`StreamStartEventContent`].
///
/// [`StreamStartEventContent`]: super::start::StreamStartEventContent
#[derive(Clone, Debug, Serialize, EventContent)]
#[cfg_attr(not(ruma_unstable_exhaustive_types), non_exhaustive)]
#[ruma_event(type = "org.matrix.msc3381.stream.start", kind = MessageLike, custom_redacted)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum UnstableStreamStartEventContent {
    /// A new stream start event.
    New(NewUnstableStreamStartEventContent),

    /// A replacement stream start event.
    Replacement(ReplacementUnstableStreamStartEventContent),
}

impl UnstableStreamStartEventContent {
    /// Get the stream start content of this event content.
    pub fn stream_start(&self) -> &UnstableStreamStartContentBlock {
        match self {
            Self::New(c) => &c.stream_start,
            Self::Replacement(c) => &c.relates_to.new_content.stream_start,
        }
    }
}

impl RedactContent for UnstableStreamStartEventContent {
    type Redacted = RedactedUnstableStreamStartEventContent;

    fn redact(self, _rules: &RedactionRules) -> Self::Redacted {
        RedactedUnstableStreamStartEventContent::default()
    }
}

impl From<NewUnstableStreamStartEventContent> for UnstableStreamStartEventContent {
    fn from(value: NewUnstableStreamStartEventContent) -> Self {
        Self::New(value)
    }
}

impl From<ReplacementUnstableStreamStartEventContent> for UnstableStreamStartEventContent {
    fn from(value: ReplacementUnstableStreamStartEventContent) -> Self {
        Self::Replacement(value)
    }
}

impl OriginalSyncUnstableStreamStartEvent {
    /// Compile the results for this stream with the given response into an
    /// `UnstableStreamEndEventContent`.
    ///
    /// It generates a default text representation of the results in English.
    ///
    /// This uses [`compile_unstable_stream_results()`] internally.
    pub fn compile_results(&self, ) -> UnstableStreamEndEventContent {
        UnstableStreamEndEventContent::new(self.event_id.clone())
    }
}

/// A new unstable stream start event.
#[derive(Clone, Debug, Serialize)]
#[cfg_attr(not(ruma_unstable_exhaustive_types), non_exhaustive)]
pub struct NewUnstableStreamStartEventContent {
    /// The stream content of the message.
    #[serde(rename = "org.matrix.msc3381.stream.start")]
    pub stream_start: UnstableStreamStartContentBlock,

    /// Text representation of the message, for clients that don't support streams.
    #[serde(rename = "org.matrix.msc1767.text")]
    pub text: Option<String>,

    /// Information about related messages.
    #[serde(rename = "m.relates_to", skip_serializing_if = "Option::is_none")]
    pub relates_to: Option<RelationWithoutReplacement>,
}

impl NewUnstableStreamStartEventContent {
    /// Creates a `NewUnstableStreamStartEventContent` with the given stream content.
    pub fn new(stream_start: UnstableStreamStartContentBlock) -> Self {
        Self { stream_start, text: None, relates_to: None }
    }

    /// Creates a `NewUnstableStreamStartEventContent` with the given plain text fallback
    /// representation and stream content.
    pub fn plain_text(text: impl Into<String>, stream_start: UnstableStreamStartContentBlock) -> Self {
        Self { stream_start, text: Some(text.into()), relates_to: None }
    }
}

impl StaticEventContent for NewUnstableStreamStartEventContent {
    const TYPE: &'static str = "org.matrix.msc3381.stream.start";

    type IsPrefix = <UnstableStreamStartEventContent as StaticEventContent>::IsPrefix;
}

impl MessageLikeEventContent for NewUnstableStreamStartEventContent {
    fn event_type(&self) -> MessageLikeEventType {
        MessageLikeEventType::UnstableStreamStart
    }
}

/// Form of [`NewUnstableStreamStartEventContent`] without relation.
///
/// To construct this type, construct a [`NewUnstableStreamStartEventContent`] and then use one of its
/// `::from()` / `.into()` methods.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(not(ruma_unstable_exhaustive_types), non_exhaustive)]
pub struct NewUnstableStreamStartEventContentWithoutRelation {
    /// The stream content of the message.
    #[serde(rename = "org.matrix.msc3381.stream.start")]
    pub stream_start: UnstableStreamStartContentBlock,

    /// Text representation of the message, for clients that don't support streams.
    #[serde(rename = "org.matrix.msc1767.text")]
    pub text: Option<String>,
}

impl From<NewUnstableStreamStartEventContent> for NewUnstableStreamStartEventContentWithoutRelation {
    fn from(value: NewUnstableStreamStartEventContent) -> Self {
        let NewUnstableStreamStartEventContent { stream_start, text, .. } = value;
        Self { stream_start, text }
    }
}

/// A replacement unstable stream start event.
#[derive(Clone, Debug)]
#[cfg_attr(not(ruma_unstable_exhaustive_types), non_exhaustive)]
pub struct ReplacementUnstableStreamStartEventContent {
    /// The stream content of the message.
    pub stream_start: Option<UnstableStreamStartContentBlock>,

    /// Text representation of the message, for clients that don't support streams.
    pub text: Option<String>,

    /// Information about related messages.
    pub relates_to: Replacement<NewUnstableStreamStartEventContentWithoutRelation>,
}

impl ReplacementUnstableStreamStartEventContent {
    /// Creates a `ReplacementUnstableStreamStartEventContent` with the given stream content that
    /// replaces the event with the given ID.
    ///
    /// The constructed content does not have a fallback by default.
    pub fn new(stream_start: UnstableStreamStartContentBlock, replaces: OwnedEventId) -> Self {
        Self {
            stream_start: None,
            text: None,
            relates_to: Replacement {
                event_id: replaces,
                new_content: NewUnstableStreamStartEventContent::new(stream_start).into(),
            },
        }
    }

    /// Creates a `ReplacementUnstableStreamStartEventContent` with the given plain text fallback
    /// representation and stream content that replaces the event with the given ID.
    ///
    /// The constructed content does not have a fallback by default.
    pub fn plain_text(
        text: impl Into<String>,
        stream_start: UnstableStreamStartContentBlock,
        replaces: OwnedEventId,
    ) -> Self {
        Self {
            stream_start: None,
            text: None,
            relates_to: Replacement {
                event_id: replaces,
                new_content: NewUnstableStreamStartEventContent::plain_text(text, stream_start).into(),
            },
        }
    }
}

impl StaticEventContent for ReplacementUnstableStreamStartEventContent {
    const TYPE: &'static str = "org.matrix.msc3381.stream.start";

    type IsPrefix = <UnstableStreamStartEventContent as StaticEventContent>::IsPrefix;
}

impl MessageLikeEventContent for ReplacementUnstableStreamStartEventContent {
    fn event_type(&self) -> MessageLikeEventType {
        MessageLikeEventType::UnstableStreamStart
    }
}

/// Redacted form of UnstableStreamStartEventContent
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[cfg_attr(not(ruma_unstable_exhaustive_types), non_exhaustive)]
pub struct RedactedUnstableStreamStartEventContent {}

impl RedactedUnstableStreamStartEventContent {
    /// Creates an empty RedactedUnstableЫtreamStartEventContent.
    pub fn new() -> RedactedUnstableStreamStartEventContent {
        Self::default()
    }
}

impl StaticEventContent for RedactedUnstableStreamStartEventContent {
    const TYPE: &'static str = "org.matrix.msc3381.stream.start";

    type IsPrefix = <UnstableStreamStartEventContent as StaticEventContent>::IsPrefix;
}

impl RedactedMessageLikeEventContent for RedactedUnstableStreamStartEventContent {
    fn event_type(&self) -> MessageLikeEventType {
        MessageLikeEventType::UnstableStreamStart
    }
}

/// An unstable block for stream start content.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(not(ruma_unstable_exhaustive_types), non_exhaustive)]
pub struct UnstableStreamStartContentBlock {
    /// The question of the stream.
    pub description: UnstableStreamDescription,
    pub stream_app: UnstableStreamApp,
    pub stream_id: UnstableStreamId,
    pub third_party: bool
}

impl UnstableStreamStartContentBlock {
    /// Creates a new `StreamStartContent` with the given question and answers.
    pub fn new(
        description: impl Into<String>,
        stream_app: impl Into<String>,
        stream_id: impl Into<String>,
        third_party: bool
    ) -> Self {
        Self {
            description: UnstableStreamDescription::new(description),
            stream_app: UnstableStreamApp::new(stream_app),
            stream_id: UnstableStreamId::new(stream_id),
            third_party
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(not(ruma_unstable_exhaustive_types), non_exhaustive)]
pub struct UnstableStreamDescription {
    /// The text representation of the question.
    #[serde(rename = "org.matrix.msc1767.text")]
    pub text: String,
}

impl UnstableStreamDescription {
    /// Creates a new `UnstableStreamQuestion` with the given plain text.
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(not(ruma_unstable_exhaustive_types), non_exhaustive)]
pub struct UnstableStreamApp {
    /// The text representation of the question.
    #[serde(rename = "org.matrix.msc1767.text")]
    pub text: String,
}

impl UnstableStreamApp {
    /// Creates a new `UnstableStreamQuestion` with the given plain text.
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(not(ruma_unstable_exhaustive_types), non_exhaustive)]
pub struct UnstableStreamId {
    /// The text representation of the question.
    #[serde(rename = "org.matrix.msc1767.text")]
    pub text: String,
}

impl UnstableStreamId {
    /// Creates a new `UnstableStreamQuestion` with the given plain text.
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}