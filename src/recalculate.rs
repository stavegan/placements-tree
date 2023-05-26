pub trait Recalculate<V, E> {
    fn recalculate(&self, vertex: &V, edge: &E) -> Self;
}
