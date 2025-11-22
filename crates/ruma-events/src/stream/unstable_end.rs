//! Types for the `org.matrix.msc3381.stream.end` event, the unstable version of `m.stream.end`.

use ruma_common::OwnedEventId;
use ruma_macros::EventContent;
use serde::{Deserialize, Serialize};

use crate::relation::Reference;

/// The payload for an unstable stream end event.
///
/// This type can be generated from the unstable stream start and stream response events with
/// [`OriginalSyncUnstableStreamStartEvent::compile_results()`].
///
/// This is the event content that should be sent for room versions that don't support extensible
/// events. As of Matrix 1.7, none of the stable room versions (1 through 10) support extensible
/// events.
///
/// To send a stream end event for a room version that supports extensible events, use
/// [`StreamEndEventContent`].
///
/// [`OriginalSyncUnstableStreamStartEvent::compile_results()`]: super::unstable_start::OriginalSyncUnstableStreamStartEvent::compile_results
/// [`StreamEndEventContent`]: super::end::StreamEndEventContent
#[derive(Clone, Debug, Serialize, Deserialize, EventContent)]
#[cfg_attr(not(ruma_unstable_exhaustive_types), non_exhaustive)]
#[ruma_event(type = "org.matrix.msc3381.stream.end", kind = MessageLike)]
pub struct UnstableStreamEndEventContent {
    /// The stream end content.
    #[serde(default, rename = "org.matrix.msc3381.stream.end")]
    pub stream_end: UnstableStreamEndContentBlock,

    /// Information about the stream start event this responds to.
    #[serde(rename = "m.relates_to")]
    pub relates_to: Reference,
}

impl UnstableStreamEndEventContent {
    /// Creates a new `StreamEndEventContent` with the given fallback representation and
    /// that responds to the given stream start event ID.
    pub fn new(stream_start_id: OwnedEventId) -> Self {
        Self {
            stream_end: UnstableStreamEndContentBlock {},
            relates_to: Reference::new(stream_start_id),
        }
    }
}

/// A block for the results of a stream.
///
/// This is currently an empty struct.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[cfg_attr(not(ruma_unstable_exhaustive_types), non_exhaustive)]
pub struct UnstableStreamEndContentBlock {}
