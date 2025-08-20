pub struct BinarySearchTree<V: PartialOrd> {
    branches: Vec<TreeBranch<V>>,
}

impl<V: PartialOrd> BinarySearchTree<V> {
    pub fn new() -> Self {
        BinarySearchTree {
            branches: Vec::new(),
        }
    }

    fn find_first_available_branch(&self, branch: Option<&Box<TreeBranch<V>>>)
        -> Option<&mut TreeBranch<V>> {
        for branch in &self.branches {
            let left_none = branch.left.is_none();
            let right_none = branch.right.is_none();
            if left_none && right_none {
                let left_available = self.find_first_available_branch(branch.left.as_ref());
                let right_available = self.find_first_available_branch(branch.right.as_ref());
                if let Some(found) = left_available {
                    return Some(found);
                }
                if let Some(found) = right_available {
                    return Some(found);
                }
            }
        }

        None
    }

    pub fn insert(&mut self, value: V) {
        if let Some(branch) = self.find_first_available_branch(None) {
            if let Some(left_branch) = &branch.left {
                
                if let Some(left_value) = left_branch.value {
                    if value < left_value {
                        let mut left_side = branch.left.unwrap().as_mut();
                        left_side.left = Some(Box::new(TreeBranch::new(value)));
                        return;
                    }
                }

                if value > left_branch.value && branch.right.is_none() {
                    let mut right_side = branch.right.unwrap().as_mut();
                    right_side.value = value;
                }
            }
        }
    }
}

pub struct TreeBranch<V: PartialOrd> {
    pub left: Option<Box<TreeBranch<V>>>,
    pub right: Option<Box<TreeBranch<V>>>,
    pub value: Option<V>,
}

impl<V: PartialOrd> TreeBranch<V> {
    pub fn new(value: Option<V>) -> Self {
        TreeBranch {
            left: None,
            right: None,
            value,
        }
    }

    pub fn empty() -> Self {
        TreeBranch {
            left: None,
            right: None,
            value: None,
        }

    }
}

fn main() {

}
