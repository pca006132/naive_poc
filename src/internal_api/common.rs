use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::vec::Vec;
use ustr::Ustr;

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LocalId(pub usize);

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LocationId(pub usize);

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FileId(pub usize);

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LocalizedString {
    pub local: LocalId,
    pub content: Ustr,
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

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LocalizedDocument {
    pub local: LocalId,
    pub id: FileId,
}

#[derive(Clone, Debug, PartialOrd, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Image {
    pub id: FileId,
    pub descriptions: Vec<LocalizedString>,
}

#[skip_serializing_none]
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Url {
    pub url: Ustr,
    pub archived: Option<Ustr>,
}
