// Internal API structs
//
// We try to keep a single source of truth:
// Info should be stored once, and we refer to the ID of that entity.
//
// These are not the final user-facing API, it will reference many internal IDs that we want to
// resolve for the user. We try to factor out those that do not require resolving, so we can use
// them directly in the user-facing APIs.

use macros::DiffFields;
use super::common::*;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::vec::Vec;
use std::collections::BTreeMap;
use ustr::Ustr;

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ArtistId(usize);

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ReleaseId(usize);

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TagId(usize);

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EventId(usize);

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TrackNum {
    // 0 if there is no disc, otherwise starts from 1
    pub disc_num: u16,
    // also starts from 1...
    pub track_num: u16,
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TrackRef {
    #[serde(flatten)]
    pub track_num: TrackNum,
    pub release_id: ReleaseId,
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ArtistKind {
    Solo,
    Group,
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ArtistRole {
    Arranger,
    Vocal,
    Lyricist,
    Other,
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
    Other
}

#[skip_serializing_none]
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ArtistMembership {
    pub group_id: ArtistId,
    pub role: ArtistRole,
    pub start_date: Option<DateWithPrecision>,
    pub end_date: Option<DateWithPrecision>,
}

// proc macro to generate per field update enum, but allow exclusion
// for documents, we implement update manually (diff it)

#[skip_serializing_none]
#[derive(Clone, Debug, PartialOrd, PartialEq, Eq, Hash, Serialize, Deserialize, DiffFields)]
pub struct ArtistMetaData {
    pub name: Ustr,
    pub aliases: Vec<LocalizedString>,
    pub kind: Option<ArtistKind>,
    pub start_loc: Option<LocationId>,
    pub current_loc: Option<LocationId>,
    pub start_date: Option<DateWithPrecision>,
    pub end_date: Option<DateWithPrecision>,
    pub birthday: Birthday,
    pub birthyear: u16,
    pub urls: Vec<Url>,
    pub profile_image: Option<Image>,

    #[skip_diff]
    pub memberships: Vec<ArtistMembership>,
    #[skip_diff]
    pub tags: Vec<TagId>,
    #[skip_diff]
    pub descriptions: Vec<LocalizedDocument>,
}

// for query, also return artist -> name mapping
#[derive(Clone, Debug, PartialOrd, PartialEq, Eq, Hash, Serialize, Deserialize, DiffFields)]
pub struct Song {
    #[serde(flatten)]
    pub track_num: TrackNum,
    pub title: Ustr,
    pub localized_titles: Vec<LocalizedString>,
    pub artists: Vec<ArtistId>,
    pub credits: Vec<(ArtistId, ArtistRole)>,
    pub language: LocalId,
    pub lyrics: Vec<LocalizedDocument>,
    pub originals: Vec<(TrackRef, SongRelationKind)>,
    pub duration_s: Option<u32>,
    pub tags: Vec<TagId>,
}

// for query, also return artist -> name mapping, and simple song metadata
#[derive(Clone, Debug, PartialOrd, PartialEq, Eq, Hash, Serialize, Deserialize, DiffFields)]
pub struct Release {
    pub album_artists: Vec<ArtistId>,
    pub release_kind: ReleaseKind,
    pub title: Ustr,
    pub catalog_num: Option<Ustr>,
    pub cover_art: Option<Image>,
    pub credits: Vec<(ArtistId, ArtistRole)>,
    pub disc_names: Vec<Ustr>,
    pub event: Option<EventId>,
    pub localized_titles: Vec<LocalizedString>,
    pub release_date: Option<DateWithPrecision>,
    pub urls: Vec<Url>,
    pub tags: Vec<TagId>,
    pub images: Vec<Image>,
    pub descriptions: Vec<LocalizedDocument>
}

pub struct States {
    pub artists: Vec<ArtistMetaData>,
    pub releases: Vec<Release>,
    pub release_tracks: Vec<Vec<Song>>,
    pub group_members: BTreeMap<ArtistId, Vec<ArtistId>>,
    pub song_derived: BTreeMap<TrackRef, Vec<(TrackRef, SongRelationKind)>>,
}

impl States {
    pub fn artist_metadata_update(&mut self, id: ArtistId, diff: ArtistMetaDataDiff) -> Result<(), ()> {
        if id.0 >= self.artists.len() {
            return Err(())
        }
        apply_artist_meta_data_diff(&mut self.artists[id.0], diff);
        // further updates
        Ok(())
    }
}

