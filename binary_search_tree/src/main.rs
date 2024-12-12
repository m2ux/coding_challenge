use std::path::Iter;

#[derive(Debug, Default, Clone)]
pub struct BinarySearchTree<T>
where
    T: PartialEq + Eq + Ord + PartialOrd,
{
    inner: Option<Tree<T>>,
}

#[derive(Debug, Default, Clone)]
struct Tree<T>
where
    T: PartialEq + Eq + Ord + PartialOrd,
{
    value: T,
    left: Option<Box<Tree<T>>>,
    right: Option<Box<Tree<T>>>,
}

#[inline(always)]
fn insert_at<T>(leaf: &mut Tree<T>, new_value: T) -> bool
where
    T: PartialEq + Eq + Ord + PartialOrd,
{
    if leaf.value == new_value {
        return false;
    } else if leaf.value < new_value {
        if let Some(leaf) = leaf.right.as_mut() {
            insert_at(leaf, new_value)
        } else {
            leaf.right = Some(Box::new(Tree {
                value: new_value,
                left: None,
                right: None,
            }));
            true
        }
    } else {
        if let Some(leaf) = leaf.left.as_mut() {
            insert_at(leaf, new_value)
        } else {
            leaf.left = Some(Box::new(Tree {
                value: new_value,
                left: None,
                right: None,
            }));
            true
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: PartialEq + Eq + Ord + PartialOrd,
{
    pub fn insert(&mut self, new_value: T) -> bool {
        let root = if let Some(root) = self.inner.as_mut() {
            root
        } else {
            self.inner = Some(Tree {
                value: new_value,
                left: None,
                right: None,
            });
            return true;
        };

        insert_at(root, new_value)
    }
}

impl<T> Iterator for BinarySearchTree<T>
where
    T: PartialEq + Eq + Ord + PartialOrd,
{
    type Item = T;

    fn next() -> Option<Self::Item> {}
}

#[cfg(test)]
mod test {
    use crate::BinarySearchTree;

    #[test]
    fn empty_tree() {
        let mut tree = BinarySearchTree::default();
        assert!(tree.insert(5));
        assert!(tree.insert(1));
        assert!(tree.insert(3));
        assert!(!tree.insert(3));
    }
}

fn main() {}
