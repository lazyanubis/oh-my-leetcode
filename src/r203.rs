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
    // pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
    //     let mut head: Option<Box<ListNode>> = head;
    //     let mut header: Option<Box<ListNode>>;
    //     let mut current: Option<Box<ListNode>>;
    //     loop {
    //         match head {
    //             None => return None,
    //             Some(node) if node.val != val => {
    //                 current = node.next;
    //                 header = Some(Box::new(ListNode {
    //                     val: node.val,
    //                     next: None,
    //                 }));
    //                 break;
    //             }
    //             Some(node) => {
    //                 head = node.next;
    //             }
    //         }
    //     }

    //     let mut last = header.as_mut().unwrap();

    //     while let Some(mut node) = current {
    //         if node.val == val {
    //             current = node.next;
    //         } else {
    //             current = std::mem::take(&mut node.next);
    //             last.next = Some(node);
    //             last = last.next.as_mut().unwrap();
    //         }
    //     }

    //     header
    // }
    // pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
    //     let mut header: Box<ListNode> = Box::new(ListNode { val: 0, next: head });
    //     let mut current: &mut Box<ListNode> = &mut header;

    //     'outer: loop {
    //         let value = match &current.next {
    //             None => break 'outer,
    //             Some(n) => n.val,
    //         };
    //         if value == val {
    //             current.next = current.next.as_mut().unwrap().next.take();
    //         } else {
    //             current = current.next.as_mut().unwrap();
    //         }
    //     }

    //     header.next
    // }
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        match &mut head {
            Some(node) => {
                if node.val == val {
                    Self::remove_elements(std::mem::take(&mut node.next), val)
                } else {
                    node.next = Self::remove_elements(std::mem::take(&mut node.next), val);
                    head
                }
            }
            None => None,
        }
    }
}

fn main() {
    println!("{}", 203);

    let start = std::time::Instant::now();

    // assert_eq!(Solution::is_happy(19), true);
    // assert_eq!(Solution::is_happy(2), false);

    let end = std::time::Instant::now();

    println!("spend: {:?}", end - start);
}
