use std::borrow::Cow;

use candid::{CandidType, Decode, Deserialize, Encode};
use ic_stable_structures::{storable::Bound, Storable};

#[derive(Clone, CandidType, Deserialize)]
pub struct CandidAsset {
    #[serde(with = "serde_bytes")]
    pub content: Vec<u8>,
    pub metadata_vec: Vec<String>,
    pub metadata_field: String,
}

impl CandidAsset {
    pub fn new_with_content(content: Vec<u8>) -> Self {
        Self {
            content,
            metadata_vec: vec!["Test data 1".to_string(), "Test data 2".to_string()],
            metadata_field: "Test data 3".to_string(),
        }
    }
}

impl Storable for CandidAsset {
    fn to_bytes(&self) -> Cow<'_, [u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn into_bytes(self) -> Vec<u8> {
        Encode!(&self).unwrap()
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}
