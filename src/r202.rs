#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        #[inline(always)]
        fn m(c: char) -> i32 {
            match c {
                '1' => 1,
                '2' => 4,
                '3' => 9,
                '4' => 16,
                '5' => 25,
                '6' => 36,
                '7' => 49,
                '8' => 64,
                '9' => 81,
                _ => 0,
            }
        }
        let mut saw = std::collections::HashSet::new();
        let mut n = n;
        loop {
            if saw.contains(&n) {
                return false;
            }
            let mut nn = 0;
            for c in n.to_string().chars() {
                nn += m(c);
            }
            if nn == 1 {
                return true;
            }
            saw.insert(n);
            n = nn;
        }
    }
}

fn main() {
    println!("{}", 202);

    let start = std::time::Instant::now();

    assert_eq!(Solution::is_happy(19), true);
    assert_eq!(Solution::is_happy(2), false);

    let end = std::time::Instant::now();

    println!("spend: {:?}", end - start);
}
