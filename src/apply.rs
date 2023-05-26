pub trait Apply<D> {
    fn apply(&mut self, diff: D);
}

impl<T> Apply<T> for T
where
    T: Clone,
{
    fn apply(&mut self, diff: T) {
        *self = diff;
    }
}
