//! Types for the `m.stream.start` event.

use ruma_macros::EventContent;
use serde::{Deserialize, Serialize};

use super::end::StreamEndEventContent;
use crate::{message::TextContentBlock, room::message::Relation};

/// The payload for a stream start event.
///
/// This is the event content that should be sent for room versions that support extensible events.
/// As of Matrix 1.7, none of the stable room versions (1 through 10) support extensible events.
///
/// To send a stream start event for a room version that does not support extensible events, use
/// [`UnstableStreamStartEventContent`].
///
/// [`UnstableStreamStartEventContent`]: super::unstable_start::UnstableStreamStartEventContent
#[derive(Clone, Debug, Serialize, Deserialize, EventContent)]
#[cfg_attr(not(ruma_unstable_exhaustive_types), non_exhaustive)]
#[ruma_event(type = "m.stream.start", kind = MessageLike, without_relation)]
pub struct StreamStartEventContent {
    /// The stream content of the message.
    #[serde(rename = "m.stream")]
    pub stream: StreamContentBlock,

    /// Text representation of the message, for clients that don't support streams.
    #[serde(rename = "m.text")]
    pub text: TextContentBlock,

    /// Information about related messages.
    #[serde(
        flatten,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::room::message::relation_serde::deserialize_relation"
    )]
    pub relates_to: Option<Relation<StreamStartEventContentWithoutRelation>>,

    /// Whether this message is automated.
    #[cfg(feature = "unstable-msc3955")]
    #[serde(
        default,
        skip_serializing_if = "ruma_common::serde::is_default",
        rename = "org.matrix.msc1767.automated"
    )]
    pub automated: bool,
}

impl StreamStartEventContent {
    /// Creates a new `StreamStartEventContent` with the given fallback representation and stream
    /// content.
    pub fn new(text: TextContentBlock, stream: StreamContentBlock) -> Self {
        Self {
            stream,
            text,
            relates_to: None,
            #[cfg(feature = "unstable-msc3955")]
            automated: false,
        }
    }

    /// Creates a new `StreamStartEventContent` with the given plain text fallback
    /// representation and stream content.
    pub fn with_plain_text(plain_text: impl Into<String>, stream: StreamContentBlock) -> Self {
        Self::new(TextContentBlock::plain(plain_text), stream)
    }
}

impl OriginalSyncStreamStartEvent {
    /// Compile the results for this stream with the given response into a `StreamEndEventContent`.
    ///
    /// It generates a default text representation of the results in English.
    ///
    /// This uses [`compile_stream_results()`] internally.
    pub fn compile_results(&self) -> StreamEndEventContent {
        StreamEndEventContent::with_plain_text(self.event_id.clone())
    }
}

/// A block for stream content.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(not(ruma_unstable_exhaustive_types), non_exhaustive)]
pub struct StreamContentBlock {
    /// The question of the stream.
    pub description: StreamDescription,
    pub stream_app: StreamApp,
    pub stream_id: StreamId,
    pub third_party: bool
}

impl StreamContentBlock {
    /// Creates a new `StreamStartContent` with the given question and answers.
    pub fn new(
        description: TextContentBlock,
        stream_app: TextContentBlock,
        stream_id: TextContentBlock,
        third_party: bool
    ) -> Self {
        Self {
            description: description.into(),
            stream_app: stream_app.into(),
            stream_id: stream_id.into(),
            third_party
        }
    }
}

/// The question of a stream.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(not(ruma_unstable_exhaustive_types), non_exhaustive)]
pub struct StreamDescription {
    /// The text representation of the question.
    #[serde(rename = "m.text")]
    pub text: TextContentBlock,
}

impl From<TextContentBlock> for StreamDescription {
    fn from(text: TextContentBlock) -> Self {
        Self { text }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(not(ruma_unstable_exhaustive_types), non_exhaustive)]
pub struct StreamApp {
    /// The text representation of the question.
    #[serde(rename = "m.text")]
    pub text: TextContentBlock,
}

impl From<TextContentBlock> for StreamApp {
    fn from(text: TextContentBlock) -> Self {
        Self { text }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(not(ruma_unstable_exhaustive_types), non_exhaustive)]
pub struct StreamId {
    /// The text representation of the question.
    #[serde(rename = "m.text")]
    pub text: TextContentBlock,
}

impl From<TextContentBlock> for StreamId {
    fn from(text: TextContentBlock) -> Self {
        Self { text }
    }
}