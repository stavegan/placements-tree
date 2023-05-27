mod apply;
mod fill;
mod node;
mod recalculate;

pub use crate::apply::Apply;
use crate::fill::Fill;
use crate::node::Node;
pub use crate::recalculate::Recalculate;
use std::collections::LinkedList;
use std::ptr::NonNull;

pub struct PlacementsTree<V, E, D> {
    _root: Box<Node<D>>,
    vertices: Vec<V>,
    vertices_idx: Vec<LinkedList<NonNull<Node<D>>>>,
    edges: Vec<Vec<E>>,
    edges_idx: Vec<Vec<LinkedList<NonNull<Node<D>>>>>,
    n: usize,
}

impl<V, E, D> PlacementsTree<V, E, D> {
    pub fn new(n: usize, k: usize, key: usize) -> Self
    where
        V: Default + Clone,
        E: Default + Clone,
        D: Default,
    {
        assert!(key <= n);
        let k = k.min(n);
        let root = Node::root(n, k, key);
        let vertices = vec![V::default(); n + 1];
        let mut vertices_idx = vec![LinkedList::new(); n + 1];
        root.fill(&mut vertices_idx);
        let edges = vec![vec![E::default(); n + 1]; n + 1];
        let mut edges_idx = vec![vec![LinkedList::new(); n + 1]; n + 1];
        root.fill(&mut edges_idx);
        Self {
            _root: root,
            vertices,
            vertices_idx,
            edges,
            edges_idx,
            n,
        }
    }

    pub fn update_vertex<Diff>(&mut self, v: usize, diff: Diff) -> Option<&D>
    where
        V: Apply<Diff>,
        D: Recalculate<V, E> + PartialOrd,
    {
        assert!(v <= self.n);
        self.vertices[v].apply(diff);
        let mut min = None;
        for vertex in self.vertices_idx[v].iter_mut() {
            if let Some(recalculated) = unsafe {
                vertex
                    .as_mut()
                    .recalculate_children(&self.vertices, &self.edges)
            } {
                min = min.filter(|min| *min < recalculated).or(Some(recalculated));
            }
        }
        min
    }

    pub fn update_edge<Diff>(&mut self, v: usize, u: usize, diff: Diff) -> Option<&D>
    where
        E: Apply<Diff>,
        D: Recalculate<V, E> + PartialOrd,
    {
        assert!(v <= self.n);
        assert!(u <= self.n);
        assert!(v != u);
        self.edges[v][u].apply(diff);
        let mut min = None;
        for vertex in self.edges_idx[v][u].iter_mut() {
            let recalculated = unsafe { vertex.as_mut().recalculate(&self.vertices, &self.edges) };
            min = min.filter(|min| *min < recalculated).or(Some(recalculated));
        }
        min
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default, PartialEq, Eq, PartialOrd, Debug)]
    struct Distance(i64);

    impl Recalculate<i64, i64> for Distance {
        fn recalculate(&self, vertex: &i64, edge: &i64) -> Self {
            Self(self.0 + *vertex + *edge)
        }
    }

    #[test]
    #[should_panic(expected = "assertion failed: key <= n")]
    fn new_panicked_test() {
        PlacementsTree::<i64, i64, Distance>::new(2, 2, 3);
    }

    #[test]
    fn update_test() {
        let mut ptree = PlacementsTree::<i64, i64, Distance>::new(2, 2, 0);
        assert_eq!(*ptree.update_vertex(1, 1).unwrap(), Distance(1));
        assert_eq!(*ptree.update_vertex(2, 1).unwrap(), Distance(2));
        assert_eq!(*ptree.update_edge(0, 1, 1).unwrap(), Distance(3));
        assert_eq!(*ptree.update_edge(0, 2, 2).unwrap(), Distance(4));
        assert_eq!(*ptree.update_edge(1, 0, 3).unwrap(), Distance(7));
        assert_eq!(*ptree.update_edge(1, 2, 4).unwrap(), Distance(7));
        assert_eq!(*ptree.update_edge(2, 0, 5).unwrap(), Distance(12));
        assert_eq!(*ptree.update_edge(2, 1, 6).unwrap(), Distance(13));
        assert_eq!(*ptree.update_vertex(0, 1).unwrap(), Distance(13));
    }

    #[test]
    #[should_panic(expected = "assertion failed: v <= self.n")]
    fn update_vertex_panicked_test() {
        let mut ptree = PlacementsTree::<i64, i64, Distance>::new(2, 2, 0);
        ptree.update_vertex(3, 0);
    }

    #[test]
    #[should_panic(expected = "assertion failed: v <= self.n")]
    fn update_edge_panicked_1_test() {
        let mut ptree = PlacementsTree::<i64, i64, Distance>::new(2, 2, 0);
        ptree.update_edge(3, 0, 0);
    }

    #[test]
    #[should_panic(expected = "assertion failed: u <= self.n")]
    fn update_edge_panicked_2_test() {
        let mut ptree = PlacementsTree::<i64, i64, Distance>::new(2, 2, 0);
        ptree.update_edge(0, 3, 0);
    }

    #[test]
    #[should_panic(expected = "assertion failed: v != u")]
    fn update_edge_panicked_3_test() {
        let mut ptree = PlacementsTree::<i64, i64, Distance>::new(2, 2, 0);
        ptree.update_edge(0, 0, 0);
    }
}
