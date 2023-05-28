pub trait Recalc<V, E> {
    fn recalc(&self, vertex: &V, edge: &E) -> Self;
}
