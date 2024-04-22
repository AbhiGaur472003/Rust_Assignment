use std::io;

// Define a simple binary tree node
#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode { val, left: None, right: None }
    }
}

fn main() {
    println!("Enter the elements of the binary tree in preorder traversal (use -1 to denote null nodes), separated by spaces:");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let elements: Vec<i32> = input.trim().split_whitespace()
        .map(|s| s.parse().expect("Invalid number"))
        .collect();

    let root = build_tree(&elements);
    
    let max_depth = max_depth(&root);
    println!("The maximum depth of the binary tree is: {}", max_depth);
}

fn build_tree(elements: &[i32]) -> Option<Box<TreeNode>> {
    let mut index = 0;
    build_tree_helper(elements, &mut index)
}

fn build_tree_helper(elements: &[i32], index: &mut usize) -> Option<Box<TreeNode>> {
    if *index >= elements.len() || elements[*index] == -1 {
        *index += 1;
        return None;
    }
    
    let mut node = TreeNode::new(elements[*index]);
    *index += 1;
    node.left = build_tree_helper(elements, index);
    node.right = build_tree_helper(elements, index);
    Some(Box::new(node))
}

fn max_depth(root: &Option<Box<TreeNode>>) -> i32 {
    match root {
        Some(node) => {
            let left_depth = max_depth(&node.left);
            let right_depth = max_depth(&node.right);
            1 + left_depth.max(right_depth)
        }
        None => 0,
    }
}
