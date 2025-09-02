pub trait Store<T: Clone> {
    fn get(&self, key: String) -> Option<T>;
    fn set(&mut self, key: String, asset: T);
}
