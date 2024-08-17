#![allow(dead_code)]
struct Solution;
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
impl Solution {
    // pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    //     let mut next = match head {
    //         None => return None,
    //         Some(node) => node,
    //     };
    //     let mut head = None;
    //     loop {
    //         match next.next {
    //             None => {
    //                 next.next = head.take();
    //                 head = Some(next);
    //                 break;
    //             }
    //             Some(n) => {
    //                 next.next = head.take();
    //                 head = Some(next);
    //                 next = n;
    //             }
    //         }
    //     }
    //     head
    // }
    // pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    //     let mut next = match head {
    //         None => return None,
    //         Some(node) => node,
    //     };
    //     let mut head = None;
    //     loop {
    //         let n = std::mem::replace(&mut next.next, head.take());
    //         head = Some(next);
    //         match n {
    //             None => break,
    //             Some(n) => next = n,
    //         }
    //     }
    //     head
    // }
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn inner(
            head: Option<Box<ListNode>>,
            tail: Option<Box<ListNode>>,
        ) -> Option<Box<ListNode>> {
            match head {
                None => return head,
                Some(mut node) => match node.next.take() {
                    None => {
                        node.next = tail;
                        return Some(node);
                    }
                    Some(n) => {
                        node.next = tail;
                        inner(Some(n), Some(node))
                    }
                },
            }
        }
        inner(head, None)
    }
}

fn main() {
    println!("{}", 206);

    let start = std::time::Instant::now();

    // assert_eq!(
    //     Solution::is_isomorphic("egg".to_string(), "add".to_string()),
    //     true
    // );
    // assert_eq!(
    //     Solution::is_isomorphic("foo".to_string(), "bar".to_string()),
    //     false
    // );
    // assert_eq!(
    //     Solution::is_isomorphic("badc".to_string(), "baba".to_string()),
    //     false
    // );

    let end = std::time::Instant::now();

    println!("spend: {:?}", end - start);
}
