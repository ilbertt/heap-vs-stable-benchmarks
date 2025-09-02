use std::borrow::Cow;

use ic_stable_structures::{storable::Bound, Storable};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct CborAsset {
    #[serde(with = "serde_bytes")]
    pub content: Vec<u8>,
    pub metadata_vec: Vec<String>,
    pub metadata_field: String,
}

impl CborAsset {
    pub fn new_with_content(content: Vec<u8>) -> Self {
        Self {
            content,
            metadata_vec: vec!["Test data 1".to_string(), "Test data 2".to_string()],
            metadata_field: "Test data 3".to_string(),
        }
    }
}

impl Storable for CborAsset {
    fn to_bytes(&self) -> Cow<'_, [u8]> {
        Cow::Owned(serde_cbor::to_vec(self).unwrap())
    }

    fn into_bytes(self) -> Vec<u8> {
        serde_cbor::to_vec(&self).unwrap()
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        serde_cbor::from_slice(bytes.as_ref()).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}
