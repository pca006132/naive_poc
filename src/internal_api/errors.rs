use super::defs::{ArtistId, EventId, LocalId, ReleaseId, TagId, TrackRef};
use serde::{Deserialize, Serialize};
use std::sync::PoisonError;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum InternalErr {
    InvalidArtistId(ArtistId),
    InvalidEventId(EventId),
    InvalidLocalId(LocalId),
    InvalidTagId(TagId),
    InvalidReleaseId(ReleaseId),
    InvalidTrackRef(TrackRef),
    Poisoned,
    OutdatedUpdate,
    InvalidRelation,
    Other(String),
}

impl<T> From<PoisonError<T>> for InternalErr {
    fn from(_: PoisonError<T>) -> InternalErr {
        InternalErr::Poisoned
    }
}

impl From<String> for InternalErr {
    fn from(s: String) -> InternalErr {
        InternalErr::Other(s)
    }
}
