use candid::{CandidType, Deserialize};

#[derive(Clone, CandidType, Deserialize)]
pub struct Asset {
    pub content: Vec<u8>,
    pub metadata_vec: Vec<String>,
    pub metadata_field: String,
}

impl Asset {
    pub fn new_with_content(content: Vec<u8>) -> Self {
        Self {
            content,
            metadata_vec: vec!["Test data 1".to_string(), "Test data 2".to_string()],
            metadata_field: "Test data 3".to_string(),
        }
    }
}
