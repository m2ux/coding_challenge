# Coding Challenge: Binary Search Tree

## Problem Statement

Implement a Binary Search Tree (BST) data structure in Rust that supports node insertion. Your implementation should maintain the BST property: for any given node, all nodes in the left subtree must have values less than the node's value, and all nodes in the right subtree must have values greater than the node's value.

## Requirements

1. Create a BST structure that can store comparable values
2. Implement the following operations:
   - Create a new empty BST
   - Insert a value into the BST while maintaining the BST property
3. Include test cases demonstrating the correctness of your implementation

## Expected Interface

Your solution should support at least these operations:

```rust
// Create a new BST
let mut tree = BinarySearchTree::new();

// Insert values
tree.insert(value);
```

## Constraints

- The implementation must handle any type that can be ordered
- Duplicate values should be handled (you may choose to either ignore them or handle them in a specified way)
- You must use safe Rust (no unsafe blocks)

## Example Usage

```rust
let mut tree = BinarySearchTree::new();
tree.insert(5);
tree.insert(3);
tree.insert(7);
```

Should create a tree structure like:

```
    5
   / \
  3   7
```

## Bonus Points (Optional Extensions)

1. Implement a method to print the tree structure
2. Add tree traversal methods (in-order, pre-order, or post-order)
3. Implement a search operation
4. Add deletion functionality
5. Include performance analysis of your implementation

## Time Allocation

| Task | Time |
|:-----|:-----|
| Basic Implementation | 20-30 minutes |
| With All Tests | 30-40 minutes |
| With Bonus Features | 40-60 minutes |
---
