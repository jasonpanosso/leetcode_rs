pub struct Solution;

use crate::data_structures::tree_node::TreeNode;
use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;

enum Direction {
    LEFT,
    RIGHT,
}

impl Solution {
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, count: i32, last_direction: Direction) -> i32 {
        if let Some(node) = node {
            let node = node.borrow();

            match last_direction {
                Direction::LEFT => {
                    let left_longest = Self::dfs(&node.left, 1, Direction::LEFT);
                    let right_longest = Self::dfs(&node.right, count + 1, Direction::RIGHT);

                    max(left_longest, right_longest)
                }
                Direction::RIGHT => {
                    let left_longest = Self::dfs(&node.left, count + 1, Direction::LEFT);
                    let right_longest = Self::dfs(&node.right, 1, Direction::RIGHT);

                    max(left_longest, right_longest)
                }
            }
        } else {
            count - 1
        }
    }
    pub fn longest_zig_zag(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let left_longest = Self::dfs(&node.borrow().left, 1, Direction::LEFT);
            let right_longest = Self::dfs(&node.borrow().right, 1, Direction::RIGHT);
            max(left_longest, right_longest)
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let tree = vec![
            Some(1),
            None,
            Some(1),
            Some(1),
            Some(1),
            None,
            None,
            Some(1),
            Some(1),
            None,
            Some(1),
            None,
            None,
            None,
            Some(1),
        ];

        let root = TreeNode::from_vec(tree);
        let expected = 3;

        assert_eq!(Solution::longest_zig_zag(root), expected);
    }
    #[test]
    fn test_example_2() {
        let tree = vec![
            Some(1),
            Some(1),
            Some(1),
            None,
            Some(1),
            None,
            None,
            Some(1),
            Some(1),
            None,
            Some(1),
        ];

        let root = TreeNode::from_vec(tree);
        let expected = 4;

        assert_eq!(Solution::longest_zig_zag(root), expected);
    }
}
