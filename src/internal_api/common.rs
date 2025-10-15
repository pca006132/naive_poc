use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use rustc_stable_hash::{FromStableHash, SipHasher128Hash, StableSipHasher128};
use ustr::Ustr;
use std::{collections::HashMap, hash::Hash};

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserId(pub usize);

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LocalId(pub Ustr);

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LocationId(pub Ustr);

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FileId(pub usize);

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

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct Hash128(pub u128);

impl FromStableHash for Hash128 {
    type Hash = SipHasher128Hash;

    fn from(hash: SipHasher128Hash) -> Hash128 {
        let upper: u128 = hash.0[0] as u128;
        let lower: u128 = hash.0[1] as u128;

        Hash128((upper << 64) | lower)
    }
}

pub fn get_hash<T: Hash>(v: &T) -> Hash128 {
    let mut hasher = StableSipHasher128::new();
    v.hash(&mut hasher);
    hasher.finish()
}
