pub trait Apply<D> {
    fn apply(&mut self, diff: D);
}

impl<T> Apply<T> for T {
    fn apply(&mut self, diff: T) {
        *self = diff;
    }
}

impl<T> Apply<T> for Option<T> {
    fn apply(&mut self, diff: T) {
        *self = Some(diff);
    }
}
