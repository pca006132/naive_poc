pub mod defs;
pub mod errors;
pub mod hashes;
pub mod wal;

// Internal API structs
//
// We try to keep a single source of truth:
// Info should be stored once, and we refer to the ID of that entity.
//
// These are not the final user-facing API, it will reference many internal IDs that we want to
// resolve for the user. We try to factor out those that do not require resolving, so we can use
// them directly in the user-facing APIs.

use defs::*;
use errors::InternalErr;
use hashes::*;
use safe_mix::triplet_mix;
use std::collections::HashMap;
use std::sync::RwLock;
use std::vec::Vec;
use wal::LogStore;

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
