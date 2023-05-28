use crate::fill::Fill;
use crate::recalc::Recalc;
use crate::Max;
use std::collections::LinkedList;
use std::ptr::NonNull;

pub struct Node<D> {
    parent: Option<NonNull<Node<D>>>,
    children: Vec<Box<Node<D>>>,
    key: usize,
    val: D,
}

impl<D> Node<D> {
    fn new(key: usize, val: D) -> Box<Self>
    where
        D: Max,
    {
        Box::new(Self {
            parent: None,
            children: Vec::new(),
            key,
            val,
        })
    }

    fn child(&self, key: usize) -> Box<Self>
    where
        D: Max,
    {
        Box::new(Self {
            parent: Some(NonNull::from(self)),
            children: Vec::new(),
            key,
            val: D::max(),
        })
    }

    pub fn root(n: usize, k: usize, key: usize, val: D) -> Box<Self>
    where
        D: Max,
    {
        assert!(key <= n);
        let k = k.min(n);
        let mut root = Self::new(key, val);
        for key in 0..key {
            root.insert(key, k);
        }
        for key in key + 1..=n {
            root.insert(key, k);
        }
        root.finish(key);
        root
    }

    fn insert(&mut self, key: usize, k: usize)
    where
        D: Max,
    {
        if k > 0 {
            let mut node = self.child(key);
            let mut index = self.children.len();
            for (i, child) in self.children.iter_mut().enumerate() {
                if key < child.key {
                    index = i;
                    break;
                }
                child.insert(key, k - 1);
                node.insert(child.key, k - 1);
            }
            if index < self.children.len() {
                for child in self.children.iter_mut() {
                    child.insert(key, k - 1);
                    node.insert(child.key, k - 1);
                }
                self.children.insert(index, node);
            } else {
                self.children.insert(index, node);
            }
        }
    }

    fn finish(&mut self, key: usize)
    where
        D: Max,
    {
        if self.children.is_empty() {
            self.children.insert(0, self.child(key));
        } else {
            for child in self.children.iter_mut() {
                child.finish(key);
            }
        }
    }

    pub unsafe fn recalc_children<V, E>(&mut self, vertices: &Vec<V>, edges: &Vec<Vec<E>>) -> &D
    where
        D: Recalc<V, E> + PartialOrd,
    {
        assert!(!self.children.is_empty());
        let mut children = self.children.iter_mut();
        let mut shortest = children
            .next()
            .map(|child| child.recalc(vertices, edges))
            .unwrap();
        for child in children {
            let recalced = child.recalc(vertices, edges);
            if recalced < shortest {
                shortest = recalced;
            }
        }
        shortest
    }

    pub unsafe fn recalc<V, E>(&mut self, vertices: &Vec<V>, edges: &Vec<Vec<E>>) -> &D
    where
        D: Recalc<V, E> + PartialOrd,
    {
        if let Some(parent) = self.parent {
            let parent_key = parent.as_ref().key;
            let parent_val = &parent.as_ref().val;
            let vertex = &vertices[parent_key];
            let edge = &edges[parent_key][self.key];
            self.val = parent_val.recalc(vertex, edge);
        }
        if self.children.is_empty() {
            &self.val
        } else {
            let mut children = self.children.iter_mut();
            let mut shortest = children
                .next()
                .map(|child| child.recalc(vertices, edges))
                .unwrap();
            for child in children {
                let recalced = child.recalc(vertices, edges);
                if recalced < shortest {
                    shortest = recalced;
                }
            }
            shortest
        }
    }

    #[cfg(test)]
    fn placements(&self) -> LinkedList<LinkedList<usize>> {
        if self.children.is_empty() {
            LinkedList::from([LinkedList::from([self.key])])
        } else {
            let mut placements = LinkedList::new();
            for child in self.children.iter() {
                placements.append(
                    &mut child
                        .placements()
                        .into_iter()
                        .map(|mut placements| {
                            let mut prepended = LinkedList::from([self.key]);
                            prepended.append(&mut placements);
                            prepended
                        })
                        .collect::<LinkedList<LinkedList<usize>>>(),
                );
            }
            placements
        }
    }
}

impl<D> Fill<Vec<LinkedList<NonNull<Node<D>>>>> for Node<D> {
    fn fill(&self, vertices: &mut Vec<LinkedList<NonNull<Node<D>>>>) {
        if !self.children.is_empty() {
            vertices[self.key].push_back(NonNull::from(self));
        }
        for child in self.children.iter() {
            child.fill(vertices);
        }
    }
}

impl<D> Fill<Vec<Vec<LinkedList<NonNull<Node<D>>>>>> for Node<D> {
    fn fill(&self, edges: &mut Vec<Vec<LinkedList<NonNull<Node<D>>>>>) {
        if let Some(parent) = self.parent {
            unsafe {
                edges[parent.as_ref().key][self.key].push_back(NonNull::from(self));
            }
        }
        for child in self.children.iter() {
            child.fill(edges);
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
            if self.0 == i64::MAX {
                Self(i64::MAX)
            } else {
                Self(self.0 + vertex + edge)
            }
        }
    }

    #[test]
    #[should_panic(expected = "assertion failed: key <= n")]
    fn root_panicked_test() {
        Node::root(2, 2, 3, Dist(0));
    }

    #[test]
    fn permutations_test() {
        let root = Node::root(0, 0, 0, Dist(0));

        assert_eq!(
            root.placements()
                .into_iter()
                .map(|placement| placement.into_iter().collect::<Vec<_>>())
                .collect::<Vec<_>>(),
            [[0, 0]]
        );

        let root = Node::root(1, 1, 0, Dist(0));

        assert_eq!(
            root.placements()
                .into_iter()
                .map(|placement| placement.into_iter().collect::<Vec<_>>())
                .collect::<Vec<_>>(),
            [[0, 1, 0]]
        );

        let root = Node::root(2, 2, 0, Dist(0));

        assert_eq!(
            root.placements()
                .into_iter()
                .map(|placement| placement.into_iter().collect::<Vec<_>>())
                .collect::<Vec<_>>(),
            [[0, 1, 2, 0], [0, 2, 1, 0]]
        );

        let root = Node::root(3, 3, 0, Dist(0));

        assert_eq!(
            root.placements()
                .into_iter()
                .map(|placement| placement.into_iter().collect::<Vec<_>>())
                .collect::<Vec<_>>(),
            [
                [0, 1, 2, 3, 0],
                [0, 1, 3, 2, 0],
                [0, 2, 1, 3, 0],
                [0, 2, 3, 1, 0],
                [0, 3, 1, 2, 0],
                [0, 3, 2, 1, 0],
            ]
        );

        let root = Node::root(4, 4, 0, Dist(0));

        assert_eq!(
            root.placements()
                .into_iter()
                .map(|placement| placement.into_iter().collect::<Vec<_>>())
                .collect::<Vec<_>>(),
            [
                [0, 1, 2, 3, 4, 0],
                [0, 1, 2, 4, 3, 0],
                [0, 1, 3, 2, 4, 0],
                [0, 1, 3, 4, 2, 0],
                [0, 1, 4, 2, 3, 0],
                [0, 1, 4, 3, 2, 0],
                [0, 2, 1, 3, 4, 0],
                [0, 2, 1, 4, 3, 0],
                [0, 2, 3, 1, 4, 0],
                [0, 2, 3, 4, 1, 0],
                [0, 2, 4, 1, 3, 0],
                [0, 2, 4, 3, 1, 0],
                [0, 3, 1, 2, 4, 0],
                [0, 3, 1, 4, 2, 0],
                [0, 3, 2, 1, 4, 0],
                [0, 3, 2, 4, 1, 0],
                [0, 3, 4, 1, 2, 0],
                [0, 3, 4, 2, 1, 0],
                [0, 4, 1, 2, 3, 0],
                [0, 4, 1, 3, 2, 0],
                [0, 4, 2, 1, 3, 0],
                [0, 4, 2, 3, 1, 0],
                [0, 4, 3, 1, 2, 0],
                [0, 4, 3, 2, 1, 0],
            ]
        );
    }

    #[test]
    fn placements_test() {
        let root = Node::root(4, 0, 0, Dist(0));

        assert_eq!(
            root.placements()
                .into_iter()
                .map(|placement| placement.into_iter().collect::<Vec<_>>())
                .collect::<Vec<_>>(),
            [[0, 0]]
        );

        let root = Node::root(4, 1, 0, Dist(0));

        assert_eq!(
            root.placements()
                .into_iter()
                .map(|placement| placement.into_iter().collect::<Vec<_>>())
                .collect::<Vec<_>>(),
            [[0, 1, 0], [0, 2, 0], [0, 3, 0], [0, 4, 0]]
        );

        let root = Node::root(4, 2, 0, Dist(0));

        assert_eq!(
            root.placements()
                .into_iter()
                .map(|placement| placement.into_iter().collect::<Vec<_>>())
                .collect::<Vec<_>>(),
            [
                [0, 1, 2, 0],
                [0, 1, 3, 0],
                [0, 1, 4, 0],
                [0, 2, 1, 0],
                [0, 2, 3, 0],
                [0, 2, 4, 0],
                [0, 3, 1, 0],
                [0, 3, 2, 0],
                [0, 3, 4, 0],
                [0, 4, 1, 0],
                [0, 4, 2, 0],
                [0, 4, 3, 0],
            ]
        );

        let root = Node::root(4, 3, 0, Dist(0));

        assert_eq!(
            root.placements()
                .into_iter()
                .map(|placement| placement.into_iter().collect::<Vec<_>>())
                .collect::<Vec<_>>(),
            [
                [0, 1, 2, 3, 0],
                [0, 1, 2, 4, 0],
                [0, 1, 3, 2, 0],
                [0, 1, 3, 4, 0],
                [0, 1, 4, 2, 0],
                [0, 1, 4, 3, 0],
                [0, 2, 1, 3, 0],
                [0, 2, 1, 4, 0],
                [0, 2, 3, 1, 0],
                [0, 2, 3, 4, 0],
                [0, 2, 4, 1, 0],
                [0, 2, 4, 3, 0],
                [0, 3, 1, 2, 0],
                [0, 3, 1, 4, 0],
                [0, 3, 2, 1, 0],
                [0, 3, 2, 4, 0],
                [0, 3, 4, 1, 0],
                [0, 3, 4, 2, 0],
                [0, 4, 1, 2, 0],
                [0, 4, 1, 3, 0],
                [0, 4, 2, 1, 0],
                [0, 4, 2, 3, 0],
                [0, 4, 3, 1, 0],
                [0, 4, 3, 2, 0],
            ]
        );
    }

    #[test]
    fn placements_keyed_test() {
        let root = Node::root(2, 2, 1, Dist(0));

        assert_eq!(
            root.placements()
                .into_iter()
                .map(|placement| placement.into_iter().collect::<Vec<_>>())
                .collect::<Vec<_>>(),
            [[1, 0, 2, 1], [1, 2, 0, 1]]
        );

        let root = Node::root(2, 2, 2, Dist(0));

        assert_eq!(
            root.placements()
                .into_iter()
                .map(|placement| placement.into_iter().collect::<Vec<_>>())
                .collect::<Vec<_>>(),
            [[2, 0, 1, 2], [2, 1, 0, 2]]
        );
    }

    #[test]
    fn fill_vertices_test() {
        let root = Node::root(4, 2, 0, Dist(0));

        let mut vertices = vec![LinkedList::new(); 5];
        root.fill(&mut vertices);

        assert_eq!(vertices[0].len(), 1);
        assert_eq!(vertices[1].len(), 4);
        assert_eq!(vertices[2].len(), 4);
        assert_eq!(vertices[3].len(), 4);
        assert_eq!(vertices[4].len(), 4);

        for key in 0..vertices.len() {
            for vertex in vertices[key].iter() {
                unsafe {
                    assert_eq!(vertex.as_ref().key, key);
                }
            }
        }
    }

    #[test]
    fn fill_edges_test() {
        let root = Node::root(4, 2, 0, Dist(0));

        let mut edges = vec![vec![LinkedList::new(); 5]; 5];
        root.fill(&mut edges);

        assert_eq!(edges[0][0].len(), 0);
        assert_eq!(edges[0][1].len(), 1);
        assert_eq!(edges[0][2].len(), 1);
        assert_eq!(edges[0][3].len(), 1);
        assert_eq!(edges[0][4].len(), 1);
        assert_eq!(edges[1][0].len(), 3);
        assert_eq!(edges[1][1].len(), 0);
        assert_eq!(edges[1][2].len(), 1);
        assert_eq!(edges[1][3].len(), 1);
        assert_eq!(edges[1][4].len(), 1);
        assert_eq!(edges[2][0].len(), 3);
        assert_eq!(edges[2][1].len(), 1);
        assert_eq!(edges[2][2].len(), 0);
        assert_eq!(edges[2][3].len(), 1);
        assert_eq!(edges[2][4].len(), 1);
        assert_eq!(edges[3][0].len(), 3);
        assert_eq!(edges[3][1].len(), 1);
        assert_eq!(edges[3][2].len(), 1);
        assert_eq!(edges[3][3].len(), 0);
        assert_eq!(edges[3][4].len(), 1);
        assert_eq!(edges[4][0].len(), 3);
        assert_eq!(edges[4][1].len(), 1);
        assert_eq!(edges[4][2].len(), 1);
        assert_eq!(edges[4][3].len(), 1);
        assert_eq!(edges[4][4].len(), 0);

        for i in 0..edges.len() {
            for key in 0..edges[i].len() {
                for edge in edges[i][key].iter() {
                    unsafe {
                        assert_eq!(edge.as_ref().key, key);
                    }
                }
            }
        }
    }

    #[test]
    fn recalc_test() {
        let mut root = Node::root(2, 2, 0, Dist(0));

        let vertices = vec![0, 0, 0];

        let edges = vec![vec![0, 1, 2], vec![3, 0, 4], vec![5, 6, 0]];

        unsafe {
            assert_eq!(*root.recalc(&vertices, &edges), Dist(10));
        }
    }
}
