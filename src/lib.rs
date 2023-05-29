mod apply;
mod fill;
mod max;
mod node;
mod recalc;

pub use crate::apply::Apply;
use crate::fill::Fill;
pub use crate::max::Max;
use crate::node::Node;
pub use crate::recalc::Recalc;
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
    pub fn new(n: usize, k: usize, key: usize, val: D) -> Self
    where
        V: Default + Clone,
        E: Default + Clone,
        D: Max,
    {
        assert!(key <= n);
        let k = k.min(n);
        let root = Node::root(n, k, key, val);
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
        D: Recalc<V, E> + PartialOrd,
    {
        assert!(v <= self.n);
        self.vertices[v].apply(diff);
        if self.vertices_idx[v].is_empty() {
            None
        } else {
            unsafe {
                let mut vertices = self.vertices_idx[v].iter_mut();
                let mut shortest = vertices
                    .next()
                    .map(|vertex| vertex.as_mut().recalc_children(&self.vertices, &self.edges))
                    .unwrap();
                for vertex in vertices {
                    let recalced = vertex.as_mut().recalc_children(&self.vertices, &self.edges);
                    if recalced < shortest {
                        shortest = recalced;
                    }
                }
                Some(shortest)
            }
        }
    }

    pub fn update_edge<Diff>(&mut self, v: usize, u: usize, diff: Diff) -> Option<&D>
    where
        E: Apply<Diff>,
        D: Recalc<V, E> + PartialOrd,
    {
        assert!(v <= self.n);
        assert!(u <= self.n);
        assert!(v != u);
        self.edges[v][u].apply(diff);
        if self.edges_idx[v][u].is_empty() {
            None
        } else {
            unsafe {
                let mut edges = self.edges_idx[v][u].iter_mut();
                let mut shortest = edges
                    .next()
                    .map(|edge| edge.as_mut().recalc(&self.vertices, &self.edges))
                    .unwrap();
                for edge in edges {
                    let recalced = edge.as_mut().recalc(&self.vertices, &self.edges);
                    if recalced < shortest {
                        shortest = recalced;
                    }
                }
                Some(shortest)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(PartialEq, Eq, PartialOrd, Debug)]
    struct Dist(i64);

    impl Max for Dist {
        fn max() -> Self {
            Dist(i64::MAX)
        }
    }

    impl Recalc<i64, i64> for Dist {
        fn recalc(&self, vertex: &i64, edge: &i64) -> Self {
            if *self == Self::max() {
                Self::max()
            } else {
                Self(self.0 + vertex + edge)
            }
        }
    }

    #[test]
    #[should_panic(expected = "assertion failed: key <= n")]
    fn new_panicked_test() {
        PlacementsTree::<i64, i64, Dist>::new(2, 2, 3, Dist(0));
    }

    #[test]
    fn update_test() {
        let mut ptree: PlacementsTree<i64, i64, Dist> = PlacementsTree::new(2, 2, 0, Dist(0));
        assert_eq!(*ptree.update_vertex(1, 1).unwrap(), Dist::max());
        assert_eq!(*ptree.update_vertex(2, 1).unwrap(), Dist::max());
        assert_eq!(*ptree.update_edge(0, 1, 1).unwrap(), Dist(3));
        assert_eq!(*ptree.update_edge(0, 2, 2).unwrap(), Dist(4));
        assert_eq!(*ptree.update_edge(1, 0, 3).unwrap(), Dist(7));
        assert_eq!(*ptree.update_edge(1, 2, 4).unwrap(), Dist(7));
        assert_eq!(*ptree.update_edge(2, 0, 5).unwrap(), Dist(12));
        assert_eq!(*ptree.update_edge(2, 1, 6).unwrap(), Dist(13));
        assert_eq!(*ptree.update_vertex(0, 1).unwrap(), Dist(13));
    }

    #[test]
    #[should_panic(expected = "assertion failed: v <= self.n")]
    fn update_vertex_panicked_test() {
        let mut ptree: PlacementsTree<i64, i64, Dist> = PlacementsTree::new(2, 2, 0, Dist(0));
        ptree.update_vertex(3, 0);
    }

    #[test]
    #[should_panic(expected = "assertion failed: v <= self.n")]
    fn update_edge_panicked_1_test() {
        let mut ptree: PlacementsTree<i64, i64, Dist> = PlacementsTree::new(2, 2, 0, Dist(0));
        ptree.update_edge(3, 0, 0);
    }

    #[test]
    #[should_panic(expected = "assertion failed: u <= self.n")]
    fn update_edge_panicked_2_test() {
        let mut ptree: PlacementsTree<i64, i64, Dist> = PlacementsTree::new(2, 2, 0, Dist(0));
        ptree.update_edge(0, 3, 0);
    }

    #[test]
    #[should_panic(expected = "assertion failed: v != u")]
    fn update_edge_panicked_3_test() {
        let mut ptree: PlacementsTree<i64, i64, Dist> = PlacementsTree::new(2, 2, 0, Dist(0));
        ptree.update_edge(0, 0, 0);
    }
}
