use std::collections::LinkedList;
use std::ptr::NonNull;

pub struct Node<V> {
    parent: Option<NonNull<Node<V>>>,
    children: Vec<NonNull<Node<V>>>,
    key: usize,
    val: V,
}

impl<V> Node<V> {
    fn new(key: usize) -> Box<Self>
    where
        V: Default,
    {
        Box::new(Self {
            parent: None,
            children: Vec::new(),
            key,
            val: V::default(),
        })
    }

    fn child(&self, key: usize) -> Box<Self>
    where
        V: Default,
    {
        Box::new(Self {
            parent: Some(NonNull::from(self)),
            children: Vec::new(),
            key,
            val: V::default(),
        })
    }

    pub unsafe fn root(n: usize, k: usize, key: usize) -> NonNull<Self>
    where
        V: Default,
    {
        assert!(key <= n);
        let k = k.min(n);
        let mut root = Self::new(key);
        for key in 0..key {
            root.insert(key, k);
        }
        for key in key + 1..=n {
            root.insert(key, k);
        }
        root.finish(key);
        NonNull::from(Box::leak(root))
    }

    unsafe fn insert(&mut self, key: usize, k: usize)
    where
        V: Default,
    {
        if k > 0 {
            let mut node = self.child(key);
            let mut index = self.children.len();
            for (i, child) in self.children.iter_mut().enumerate() {
                if key < child.as_ref().key {
                    index = i;
                    break;
                }
                child.as_mut().insert(key, k - 1);
                node.as_mut().insert(child.as_ref().key, k - 1);
            }
            if index < self.children.len() {
                for child in self.children.iter_mut() {
                    child.as_mut().insert(key, k - 1);
                    node.as_mut().insert(child.as_ref().key, k - 1);
                }
                self.children.insert(index, NonNull::from(Box::leak(node)));
            } else {
                self.children.insert(index, NonNull::from(Box::leak(node)));
            }
        }
    }

    unsafe fn finish(&mut self, key: usize)
    where
        V: Default,
    {
        if self.children.is_empty() {
            self.children
                .insert(0, NonNull::from(Box::leak(self.child(key))));
        } else {
            for child in self.children.iter_mut() {
                child.as_mut().finish(key);
            }
        }
    }

    unsafe fn placements(&self) -> Vec<Vec<usize>> {
        let mut placements = Vec::new();
        if self.children.is_empty() {
            placements.insert(0, Vec::from([self.key]));
        } else {
            for child in self.children.iter() {
                placements.append(
                    &mut child
                        .as_ref()
                        .placements()
                        .into_iter()
                        .map(|mut placements| {
                            let mut prepended = Vec::from([self.key]);
                            prepended.append(&mut placements);
                            prepended
                        })
                        .collect::<Vec<Vec<usize>>>(),
                );
            }
        }
        placements
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn permutations_test() {
        let root = unsafe { Node::<()>::root(0, 0, 0) };

        unsafe { assert_eq!(root.as_ref().placements(), [[0, 0]]) };

        let root = unsafe { Node::<()>::root(1, 1, 0) };

        unsafe { assert_eq!(root.as_ref().placements(), [[0, 1, 0]]) };

        let root = unsafe { Node::<()>::root(2, 2, 0) };

        unsafe { assert_eq!(root.as_ref().placements(), [[0, 1, 2, 0], [0, 2, 1, 0]]) };

        let root = unsafe { Node::<()>::root(3, 3, 0) };

        unsafe {
            assert_eq!(
                root.as_ref().placements(),
                [
                    [0, 1, 2, 3, 0],
                    [0, 1, 3, 2, 0],
                    [0, 2, 1, 3, 0],
                    [0, 2, 3, 1, 0],
                    [0, 3, 1, 2, 0],
                    [0, 3, 2, 1, 0],
                ]
            )
        };

        let root = unsafe { Node::<()>::root(4, 4, 0) };

        unsafe {
            assert_eq!(
                root.as_ref().placements(),
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
            )
        };
    }

    #[test]
    fn placements_test() {
        let root = unsafe { Node::<()>::root(4, 0, 0) };

        unsafe { assert_eq!(root.as_ref().placements(), [[0, 0]]) };

        let root = unsafe { Node::<()>::root(4, 1, 0) };

        unsafe {
            assert_eq!(
                root.as_ref().placements(),
                [[0, 1, 0], [0, 2, 0], [0, 3, 0], [0, 4, 0]]
            )
        };

        let root = unsafe { Node::<()>::root(4, 2, 0) };

        unsafe {
            assert_eq!(
                root.as_ref().placements(),
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
            )
        };

        let root = unsafe { Node::<()>::root(4, 3, 0) };

        unsafe {
            assert_eq!(
                root.as_ref().placements(),
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
            )
        };
    }

    #[test]
    fn placements_keyed_test() {
        let root: NonNull<Node<()>> = unsafe { Node::root(2, 2, 1) };

        unsafe { assert_eq!(root.as_ref().placements(), [[1, 0, 2, 1], [1, 2, 0, 1]]) };

        let root: NonNull<Node<()>> = unsafe { Node::root(2, 2, 2) };

        unsafe { assert_eq!(root.as_ref().placements(), [[2, 0, 1, 2], [2, 1, 0, 2]]) };
    }

    #[test]
    #[should_panic(expected = "assertion failed: key <= n")]
    fn placements_keyed_panicked_test() {
        unsafe { Node::<()>::root(2, 2, 3) };
    }
}
