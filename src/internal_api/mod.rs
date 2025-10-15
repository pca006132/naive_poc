pub mod common;

// Internal API structs
//
// We try to keep a single source of truth:
// Info should be stored once, and we refer to the ID of that entity.
//
// These are not the final user-facing API, it will reference many internal IDs that we want to
// resolve for the user. We try to factor out those that do not require resolving, so we can use
// them directly in the user-facing APIs.

use common::*;
use linear_map::{set::LinearSet, LinearMap};
use macros::DiffFields;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::BTreeMap;
use std::sync::{PoisonError, RwLock};
use std::vec::Vec;
use ustr::Ustr;

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ArtistId(usize);

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ReleaseId(usize);

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TagId(usize);

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EventId(usize);

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TrackNum {
    // 0 if there is no disc, otherwise starts from 1
    pub disc_num: u16,
    // also starts from 1...
    pub track_num: u16,
}

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TrackRef {
    pub release_id: ReleaseId,
    #[serde(flatten)]
    pub track_num: TrackNum,
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ArtistKind {
    Solo,
    Group,
}

#[derive(Clone, Debug, PartialOrd, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ArtistRole {
    Arranger,
    Vocal,
    Lyricist,
    Other(String),
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SongRelationKind {
    Cover,
    Rearrangement,
    Remix,
    ReRelease,
    Other(Ustr),
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ReleaseKind {
    Album,
    Ep,
    Single,
    Compilation,
    Demo,
    Other,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialOrd, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ArtistMembership {
    pub group_id: ArtistId,
    pub role: ArtistRole,
    pub start_date: Option<DateWithPrecision>,
    pub end_date: Option<DateWithPrecision>,
}

// proc macro to generate per field update enum, but allow exclusion
// for documents, we implement update manually (diff it)

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DiffFields)]
pub struct ArtistMetaData {
    pub name: String,
    pub aliases: LinearSet<StringWithLocal>,
    pub kind: Option<ArtistKind>,
    pub start_loc: Option<LocationId>,
    pub current_loc: Option<LocationId>,
    pub start_date: Option<DateWithPrecision>,
    pub end_date: Option<DateWithPrecision>,
    pub birthday: Option<Birthday>,
    pub birthyear: Option<u16>,
    pub urls: LinearSet<Url>,

    #[skip_diff]
    pub profile_image: Option<Image>,
    #[skip_diff]
    pub memberships: LinearSet<ArtistMembership>,
    #[skip_diff]
    pub tags: LinearSet<TagId>,
    #[skip_diff]
    pub descriptions: LocalizedDocuments,
}

// for query, also return artist -> name mapping
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DiffFields)]
pub struct Song {
    pub title: String,
    pub artists: LinearSet<ArtistId>,
    pub credits: LinearSet<(ArtistId, ArtistRole)>,
    pub language: LinearSet<LocalId>,
    pub originals: LinearMap<TrackRef, SongRelationKind>,
    pub duration_s: Option<u32>,

    #[skip_diff]
    pub tags: LinearSet<TagId>,
    #[skip_diff]
    pub localized_titles: LocalizedStrings,
    #[skip_diff]
    pub lyrics: LocalizedDocuments,
}

// for query, also return artist -> name mapping, and simple song metadata
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DiffFields)]
pub struct Release {
    pub album_artists: LinearSet<ArtistId>,
    pub release_kind: ReleaseKind,
    pub title: String,
    pub catalog_num: Option<String>,
    pub cover_art: Option<Image>,
    pub credits: LinearSet<(ArtistId, ArtistRole)>,
    pub disc_names: Vec<String>,
    pub event: Option<EventId>,
    pub release_date: Option<DateWithPrecision>,
    pub urls: LinearSet<Url>,

    #[skip_diff]
    pub localized_titles: LocalizedStrings,
    #[skip_diff]
    pub tracks: LinearMap<TrackNum, Song>,
    #[skip_diff]
    pub tags: LinearSet<TagId>,
    #[skip_diff]
    pub images: LinearSet<Image>,
    #[skip_diff]
    pub descriptions: LocalizedDocuments,
}

pub struct States {
    pub artists: RwLock<Vec<RwLock<ArtistMetaData>>>,
    pub releases: RwLock<Vec<RwLock<Release>>>,

    // derived
    pub group_members: RwLock<BTreeMap<ArtistId, Vec<ArtistId>>>,
    pub artist_discography: RwLock<BTreeMap<ArtistId, Vec<TrackRef>>>,
    pub derived_songs: RwLock<BTreeMap<TrackRef, Vec<(TrackRef, SongRelationKind)>>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum InternalErr {
    InvalidArtistId(ArtistId),
    InvalidEventId(EventId),
    InvalidLocalId(LocalId),
    InvalidTagId(TagId),
    InvalidReleaseId(ReleaseId),
    InvalidTrackRef(TrackRef),
    Poisoned,
    Other(String),
}

impl<T> From<PoisonError<T>> for InternalErr {
    fn from(_: PoisonError<T>) -> InternalErr {
        InternalErr::Poisoned
    }
}

impl States {
    pub fn artist_add(&self, metadata: ArtistMetaData) -> Result<ArtistId, InternalErr> {
        // TODO: validation
        let mut artists = self.artists.write()?;
        artists.push(RwLock::new(metadata));
        Ok(ArtistId(artists.len() - 1))
    }

    pub fn artist_metadata_update(
        &self,
        id: ArtistId,
        diff: ArtistMetaDataDiff,
    ) -> Result<(), InternalErr> {
        let artists = self.artists.read()?;
        if id.0 >= artists.len() {
            return Err(InternalErr::InvalidArtistId(id));
        }
        let mut artist = artists[id.0].write()?;
        apply_artist_meta_data_diff(&mut artist, diff);
        // TODO: further updates
        Ok(())
    }
}
