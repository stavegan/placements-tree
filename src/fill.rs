pub trait Fill<T> {
    fn fill(&self, container: &mut T);
}
