use std::{cell::RefCell, collections::VecDeque, fmt, rc::Rc};

#[derive(PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl fmt::Debug for TreeNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.val)?;
        if let Some(ref left) = self.left {
            write!(f, " Left [{:?}]", left.borrow())?;
        }
        if let Some(ref right) = self.right {
            write!(f, " Right [{:?}]", right.borrow())?;
        }
        Ok(())
    }
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn from_vec(vec: Vec<Option<i32>>) -> Option<Rc<RefCell<Self>>> {
        if vec.is_empty() || vec[0].is_none() {
            return None;
        }

        let root = Rc::new(RefCell::new(TreeNode::new(vec[0].unwrap())));
        let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        queue.push_back(root.clone());

        let mut i = 1;
        while let Some(current) = queue.pop_front() {
            if i < vec.len() && vec[i].is_some() {
                let left_child = Rc::new(RefCell::new(TreeNode::new(vec[i].unwrap())));
                current.borrow_mut().left = Some(left_child.clone());
                queue.push_back(left_child);
            }
            i += 1;

            if i < vec.len() && vec[i].is_some() {
                let right_child = Rc::new(RefCell::new(TreeNode::new(vec[i].unwrap())));
                current.borrow_mut().right = Some(right_child.clone());
                queue.push_back(right_child);
            }
            i += 1;
        }

        Some(root)
    }
}
