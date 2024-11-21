use std::cmp::Ordering;
use std::iter::Iterator;

#[derive(Debug)]
struct BinarySearchTree<T>(Vec<T>);

impl<T> BinarySearchTree<T> {
    pub fn new() -> Self {
        Self(Default::default())
    }

    pub fn insert(&mut self, new_elem: T)
    where
        T: Ord,
    {
        let maybe_exists = self
            .0
            .iter()
            .position(|collection_elem| new_elem <= *collection_elem);

        match maybe_exists {
            Some(index) => {
                let existing_elem = &self.0[index];

                if existing_elem != &new_elem {
                    self.0.insert(index, new_elem);
                }
            }
            None => {
                self.0.push(new_elem);
            }
        }
    }

    pub fn into_inner(self) -> Vec<T> {
        self.0
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.0.iter()
    }
}

fn main() {}

#[test]
fn insert_left() {
    let mut tree = BinarySearchTree::new();

    tree.insert(7);
    tree.insert(3);

    assert_eq!(tree.into_inner(), vec![3, 7]);
}

#[test]
fn insert_right() {
    let mut tree = BinarySearchTree::new();

    tree.insert(7);
    tree.insert(8);

    assert_eq!(tree.into_inner(), vec![7, 8]);
}

#[test]
fn handle_duplicated() {
    let mut tree = BinarySearchTree::new();

    tree.insert(7);
    tree.insert(3);
    tree.insert(3);

    assert_eq!(tree.into_inner(), vec![3, 7]);
}
