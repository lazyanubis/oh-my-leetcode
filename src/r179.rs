struct Solution;

// impl Solution {
//     pub fn largest_number(nums: Vec<i32>) -> String {
//         if !nums.iter().any(|n| *n != 0) {
//             return "0".to_string();
//         }

//         let mut nums = nums;
//         nums.sort_by(|a, b| {
//             let a = format!("{}{}", a, b);
//             let b = format!("{}{}", b, a);
//             b.cmp(&a)
//         });
//         let mut res = String::new();
//         for num in nums {
//             res.push_str(&num.to_string());
//         }
//         res
//     }
// }

impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        if !nums.iter().any(|n| *n != 0) {
            return "0".to_string();
        }

        let mut nums = nums.iter().map(|n| n.to_string()).collect::<Vec<String>>();
        nums.sort_by(|a, b| {
            let a = format!("{}{}", a, b);
            let b = format!("{}{}", b, a);
            b.cmp(&a)
        });
        let mut res = String::new();
        for num in nums {
            res.push_str(&num.to_string());
        }
        res
    }
}

// impl Solution {
//     pub fn largest_number(nums: Vec<i32>) -> String {
//         if !nums.iter().any(|n| *n != 0) {
//             return "0".to_string();
//         }

//         use std::cmp::Ordering;

//         fn compare(a: &Vec<u8>, b: &Vec<u8>, i: usize) -> Ordering {
//             if a.len() <= i {
//                 if b.len() <= i {
//                     return Ordering::Equal;
//                 } else {
//                     return Ordering::Less;
//                 }
//             } else {
//                 if b.len() <= i {
//                     return Ordering::Greater;
//                 } else {
//                     let a0 = a[i];
//                     let b0 = b[i];
//                     match a0.cmp(&b0) {
//                         Ordering::Greater => Ordering::Less,
//                         Ordering::Less => Ordering::Greater,
//                         Ordering::Equal => {
//                             if a.len() == b.len() {
//                                 return compare(a, b, i + 1);
//                             } else {
//                                 let mut aa = a.clone();
//                                 let mut aa2 = a.clone();
//                                 let mut bb = b.clone();
//                                 let mut bb2 = b.clone();
//                                 aa.append(&mut bb2);
//                                 bb.append(&mut aa2);
//                                 return compare(&aa, &bb, 0);
//                             }
//                         }
//                     }
//                 }
//             }
//         }

//         let mut nums: Vec<Vec<u8>> = nums.iter().map(|n| n.to_string().into_bytes()).collect();
//         nums.sort_by(|a, b| compare(a, b, 0));
//         let mut res = String::new();
//         for num in nums {
//             for n in num {
//                 res.push(char::from(n));
//             }
//         }
//         res
//     }
// }

fn main() {
    println!("{}", 179);

    // let hp = Solution::largest_number([1, 2, 9, 9, 91].to_vec());
    // let hp = Solution::largest_number([1, 2].to_vec());
    let hp = Solution::largest_number([3, 30, 34, 5, 9].to_vec());

    println!("{}", hp);
}
