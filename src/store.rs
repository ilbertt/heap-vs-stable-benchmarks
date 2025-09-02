use crate::asset::Asset;

pub trait Store {
    fn get(&self, key: String) -> Option<Asset>;
    fn set(&mut self, key: String, asset: Asset);
}
