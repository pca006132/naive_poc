use super::hashes::*;
use macros::DiffFields;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;
use ustr::Ustr;

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ArtistId(pub usize);

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ReleaseId(pub usize);

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TagId(pub usize);

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EventId(pub usize);

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserId(pub usize);

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LocalId(pub Ustr);

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LocationId(pub Ustr);

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FileId(pub usize);

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
    Other(Ustr),
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

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StringWithLocal {
    pub local: LocalId,
    pub content: String,
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DatePrecision {
    Year,
    Month,
    Day,
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DateWithPrecision {
    pub year: u16,
    pub month: u16,
    pub day: u16,
    pub precision: DatePrecision,
}

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Birthday {
    pub month: u16,
    pub day: u16,
}

pub type LocalizedDocuments = HashMap<LocalId, FileId>;
pub type LocalizedStrings = HashMap<LocalId, String>;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Image {
    pub id: FileId,
    pub descriptions: LocalizedStrings,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Url {
    pub url: String,
    pub archived: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialOrd, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ArtistMembership {
    pub group_id: ArtistId,
    pub role: ArtistRole,
    pub start_date: Option<DateWithPrecision>,
    pub end_date: Option<DateWithPrecision>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DiffFields, Default)]
pub struct ArtistMetaData {
    pub name: String,
    pub aliases: Vec<StringWithLocal>,
    pub kind: Option<ArtistKind>,
    pub start_loc: Option<LocationId>,
    pub current_loc: Option<LocationId>,
    pub start_date: Option<DateWithPrecision>,
    pub end_date: Option<DateWithPrecision>,
    pub birthday: Option<Birthday>,
    pub birthyear: Option<u16>,
    pub urls: Vec<Url>,

    #[skip_diff]
    pub seq_id: Hash128,
    #[skip_diff]
    pub profile_image: Option<Image>,
    #[skip_diff]
    pub memberships: Vec<ArtistMembership>,
    #[skip_diff]
    pub tags: Vec<TagId>,
    #[skip_diff]
    pub descriptions: LocalizedDocuments,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DiffFields)]
pub struct Song {
    pub title: String,
    pub artists: Vec<ArtistId>,
    pub credits: Vec<(ArtistId, ArtistRole)>,
    pub language: Vec<LocalId>,
    pub originals: Vec<(TrackRef, SongRelationKind)>,
    pub duration_s: Option<u32>,

    #[skip_diff]
    pub tags: Vec<TagId>,
    #[skip_diff]
    pub localized_titles: LocalizedStrings,
    #[skip_diff]
    pub lyrics: LocalizedDocuments,
}

// for query, also return artist -> name mapping, and simple song metadata
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DiffFields, Default)]
pub struct Release {
    pub title: String,
    pub release_kind: Option<ReleaseKind>,
    pub catalog_num: Option<String>,
    pub album_artists: Vec<ArtistId>,
    pub cover_art: Option<Image>,
    pub credits: Vec<(ArtistId, ArtistRole)>,
    pub disc_names: Vec<String>,
    pub event: Option<EventId>,
    pub release_date: Option<DateWithPrecision>,
    pub urls: Vec<Url>,

    #[skip_diff]
    pub seq_id: Hash128,
    #[skip_diff]
    pub localized_titles: LocalizedStrings,
    #[skip_diff]
    pub tracks: HashMap<TrackNum, Song>,
    #[skip_diff]
    pub tags: Vec<TagId>,
    #[skip_diff]
    pub images: Vec<Image>,
    #[skip_diff]
    pub descriptions: LocalizedDocuments,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DiffFields, Default)]
pub struct Event {
    pub name: String,
    pub location: Option<LocationId>,
    pub address: String,
    pub start_date: Option<DateWithPrecision>,
    pub end_date: Option<DateWithPrecision>,
    pub urls: Vec<Url>,

    #[skip_diff]
    pub seq_id: Hash128,
    #[skip_diff]
    pub localized_names: LocalizedStrings,
    #[skip_diff]
    pub descriptions: LocalizedDocuments,
}
