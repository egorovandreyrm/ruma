//! Types for the `m.stream.end` event.

use std::{
    collections::{btree_map, BTreeMap},
    ops::Deref,
};

use js_int::UInt;
use ruma_common::OwnedEventId;
use ruma_macros::EventContent;
use serde::{Deserialize, Serialize};

use crate::{relation::Reference};

/// The payload for a stream end event.
///
/// This type can be generated from the stream start and stream response events with
/// [`OriginalSyncStreamStartEvent::compile_results()`].
///
/// This is the event content that should be sent for room versions that support extensible events.
/// As of Matrix 1.7, none of the stable room versions (1 through 10) support extensible events.
///
/// To send a stream end event for a room version that does not support extensible events, use
/// [`UnstableStreamEndEventContent`].
///
/// [`OriginalSyncStreamStartEvent::compile_results()`]: super::start::OriginalSyncStreamStartEvent::compile_results
/// [`UnstableStreamEndEventContent`]: super::unstable_end::UnstableStreamEndEventContent
#[derive(Clone, Debug, Serialize, Deserialize, EventContent)]
#[cfg_attr(not(ruma_unstable_exhaustive_types), non_exhaustive)]
#[ruma_event(type = "m.stream.end", kind = MessageLike)]
pub struct StreamEndEventContent {
    /// Whether this message is automated.
    #[cfg(feature = "unstable-msc3955")]
    #[serde(
        default,
        skip_serializing_if = "ruma_common::serde::is_default",
        rename = "org.matrix.msc1767.automated"
    )]
    pub automated: bool,

    /// Information about the stream start event this responds to.
    #[serde(rename = "m.relates_to")]
    pub relates_to: Reference,
}

impl StreamEndEventContent {
    /// Creates a new `StreamEndEventContent` with the given fallback representation and
    /// that responds to the given stream start event ID.
    pub fn new(stream_start_id: OwnedEventId) -> Self {
        Self {
            #[cfg(feature = "unstable-msc3955")]
            automated: false,
            relates_to: Reference::new(stream_start_id),
        }
    }

    /// Creates a new `StreamEndEventContent` with the given plain text fallback representation and
    /// that responds to the given stream start event ID.
    pub fn with_plain_text(stream_start_id: OwnedEventId) -> Self {
        Self {
            #[cfg(feature = "unstable-msc3955")]
            automated: false,
            relates_to: Reference::new(stream_start_id),
        }
    }
}