pub trait Store<T: Clone> {
    fn get(&self) -> T;
    fn set(&mut self, bytes: T);
}
