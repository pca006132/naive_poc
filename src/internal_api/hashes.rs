use rustc_stable_hash::{FromStableHash, SipHasher128Hash, StableSipHasher128};
use serde::{Deserialize, Serialize};
use std::hash::Hash;

#[derive(
    Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize, Default,
)]
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
