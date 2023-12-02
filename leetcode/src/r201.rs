#![allow(dead_code)]
struct Solution;

impl Solution {
    // pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
    //     let mut r = i32::MAX;
    //     for i in left..=right {
    //         r &= i;
    //     }
    //     r
    // }
    // pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
    //     if left == right {
    //         return left;
    //     }
    //     let mut last = 0;
    //     for i in (0..31).rev() {
    //         let mask = 1 << i;
    //         let l = left & mask;
    //         let r = right & mask;
    //         // println!("mask: {} {} {}", mask, l, r);
    //         match (l, r) {
    //             (0, 0) => continue,
    //             (_, 0) | (0, _) => return last,
    //             _ => {
    //                 last |= mask;
    //             }
    //         }
    //     }
    //     0
    // }
    pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
        if left == right {
            return left;
        }
        let r = (left ^ right) & left & right;
        // 0010
        // 0110
        // 1011
        // 0100
        // 0000
        // 0000

        // 0101
        // 0111
        // 0010
        // 1101
        // 0101

        println!("{left} {right} r {:b}", r);

        // 0111 1111 1111 1111
        // 0000 0000 0000 0001
        // 1000 0000 0000 0001
        // 0111 1111 1111 1110
        // 0111 1111 1111 1110
        let mut flag = false;
        for i in (0..31).rev() {
            let rr = r & (1 << i);
            if rr != 0 {
                flag = true;
            } else if flag {
                return r >> i << i;
            }
        }
        0
    }
}

fn main() {
    println!("{}", 201);

    let start = std::time::Instant::now();

    assert_eq!(Solution::range_bitwise_and(5, 7), 4);
    assert_eq!(Solution::range_bitwise_and(1, 2147483647), 0);
    assert_eq!(Solution::range_bitwise_and(3, 3), 3);
    assert_eq!(Solution::range_bitwise_and(6, 7), 6);
    assert_eq!(Solution::range_bitwise_and(2, 6), 0);
    // 110
    // 111

    // 111
    // 110

    // 0111 1111 1111 1111
    // 0000 0000 0000 0001

    let end = std::time::Instant::now();

    println!("spend: {:?}", end - start);
}
