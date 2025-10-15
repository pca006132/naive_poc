pub mod common;
pub mod wal;

// Internal API structs
//
// We try to keep a single source of truth:
// Info should be stored once, and we refer to the ID of that entity.
//
// These are not the final user-facing API, it will reference many internal IDs that we want to
// resolve for the user. We try to factor out those that do not require resolving, so we can use
// them directly in the user-facing APIs.

use common::*;
use wal::LogStore;
use macros::DiffFields;
use safe_mix::triplet_mix;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;
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

pub struct States<'a, L: LogStore> {
    wal: &'a L,

    artists: RwLock<Vec<RwLock<ArtistMetaData>>>,
    releases: RwLock<Vec<RwLock<Release>>>,
    events: RwLock<Vec<RwLock<Event>>>,

    // derived
    group_members: RwLock<HashMap<ArtistId, Vec<ArtistId>>>,
    artist_discography: RwLock<HashMap<ArtistId, Vec<TrackRef>>>,
    derived_songs: RwLock<HashMap<TrackRef, Vec<(TrackRef, SongRelationKind)>>>,
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

impl<'a, L: LogStore> States<'a, L> {
    pub fn artist_add(&self, user: UserId, name: String) -> Result<ArtistId, InternalErr> {
        let artist = ArtistMetaData {
            name,
            seq_id: Hash128(0),
            ..Default::default()
        };
        let mut artists = self.artists.write()?;
        self.wal.record(user, "artist_add", &artist)?;
        artists.push(RwLock::new(artist));
        Ok(ArtistId(artists.len() - 1))
    }

    pub fn release_add(&self, user: UserId, title: String) -> Result<ReleaseId, InternalErr> {
        let release = Release {
            title,
            seq_id: Hash128(0),
            ..Default::default()
        };
        let mut releases = self.releases.write()?;
        self.wal.record(user, "release_add", &release)?;
        releases.push(RwLock::new(release));
        Ok(ReleaseId(releases.len() - 1))
    }

    pub fn event_add(&self, user: UserId, name: String) -> Result<EventId, InternalErr> {
        let event = Event {
            name,
            seq_id: Hash128(0),
            ..Default::default()
        };
        let mut events = self.events.write()?;
        self.wal.record(user, "event_add", &event)?;
        events.push(RwLock::new(event));
        Ok(EventId(events.len() - 1))
    }

    pub fn artist_metadata_update(
        &self,
        user: UserId,
        id: ArtistId,
        diff: ArtistMetaDataDiff,
        mut seq_id: Hash128,
        update_seq_id: bool,
    ) -> Result<Hash128, InternalErr> {
        let hash = get_hash(&diff);
        let artists = self.artists.read()?;
        if id.0 >= artists.len() {
            return Err(InternalErr::InvalidArtistId(id));
        }
        let mut artist = artists[id.0].write()?;

        // enforce sequential update for each artist metadata
        if artist.seq_id != seq_id {
            return Err(InternalErr::OutdatedUpdate);
        }
        if update_seq_id {
            seq_id = Hash128(triplet_mix(&[seq_id.0, hash.0]).unwrap());
            artist.seq_id = seq_id;
        }
        self.wal.record(user, "artist_metadata_update", &diff)?;

        apply_artist_meta_data_diff(&mut artist, diff);
        Ok(seq_id)
    }
}
