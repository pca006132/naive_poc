use linear_map::LinearMap;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LocalId(pub usize);

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LocationId(pub usize);

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FileId(pub usize);

#[derive(Clone, Debug, PartialOrd, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

pub type LocalizedDocuments = LinearMap<LocalId, FileId>;
pub type LocalizedStrings = LinearMap<LocalId, String>;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Image {
    pub id: FileId,
    pub descriptions: LocalizedStrings,
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialOrd, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Url {
    pub url: String,
    pub archived: Option<String>,
}

pub const LOCALS: [&'static str; 5] = ["Original", "Japanese", "Chinese", "English", "German"];
