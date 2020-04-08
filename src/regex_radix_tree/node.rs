pub trait Node<T> {
    fn find(&self, value: &str) -> Vec<&T>;
}
