//! Modules for events in the `m.poll` namespace ([MSC3381]).
//!
//! This module also contains types shared by events in its child namespaces.
//!
//! [MSC3381]: https://github.com/matrix-org/matrix-spec-proposals/pull/3381

use std::ops::Deref;

pub mod end;
pub mod start;
pub mod unstable_end;
pub mod unstable_start;

/// Generate the fallback text representation of a poll end event.
///
/// This is a sentence that lists the top answers for the given results, in english. It is used to
/// generate a valid poll end event when using
/// `OriginalSync(Unstable)PollStartEvent::compile_results()`.
///
/// `answers` is an iterator of `(answer ID, answer plain text representation)` and `results` is an
/// iterator of `(answer ID, count)` ordered in descending order.
fn generate_stream_end_fallback_text<'a>() -> String {
    "stream ended".to_owned()
}
