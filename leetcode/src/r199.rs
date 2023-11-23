// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
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
}
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

impl Solution {
    // pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    //     let mut view = vec![];
    //     #[inline]
    //     fn insert(view: &mut Vec<i32>, layer: usize, value: i32) {
    //         if layer < view.len() {
    //             view[layer] = value;
    //         } else {
    //             view.push(value);
    //         }
    //     }
    //     fn right_view(root: &Option<Rc<RefCell<TreeNode>>>, layer: usize, view: &mut Vec<i32>) {
    //         if let None = root {
    //             return;
    //         }
    //         let root = root.as_ref().unwrap().borrow();
    //         insert(view, layer, root.val);
    //         right_view(&root.left, layer + 1, view);
    //         right_view(&root.right, layer + 1, view);
    //     }
    //     right_view(&root, 0, &mut view);
    //     view
    // }
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut view = vec![];

        #[inline]
        fn insert(view: &mut Vec<i32>, layer: usize, value: i32) {
            if layer < view.len() {
                view[layer] = value;
            } else {
                view.push(value);
            }
        }

        let mut list = vec![];
        list.push((root, 0));
        while let Some((root, layer)) = list.pop() {
            if let Some(root) = root {
                let root = root.borrow();
                insert(&mut view, layer, root.val);
                list.push((root.right.as_ref().and_then(|n| Some(n.clone())), layer + 1));
                list.push((root.left.as_ref().and_then(|n| Some(n.clone())), layer + 1));
            }
        }

        view
    }
}

fn main() {
    println!("{}", 199);

    let start = std::time::Instant::now();

    assert_eq!(Solution::right_side_view(None), vec![]);

    let end = std::time::Instant::now();

    println!("spend: {:?}", end - start);
}
