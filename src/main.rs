#![allow(dead_code)]
struct Solution;

impl Solution {
    // pub fn rob(nums: Vec<i32>) -> i32 {
    //     let length = nums.len();
    //     if length == 0 {
    //         return 0;
    //     }
    //     if length == 1 {
    //         return nums[0];
    //     }

    //     fn inner(nums: &[i32], visited: &mut [bool], index: usize) -> i32 {
    //         if nums.len() - 1 == index {
    //             if visited[index + 1] {
    //                 return 0;
    //             } else {
    //                 if visited[index - 1] {
    //                     return 0;
    //                 } else {
    //                     return nums[index];
    //                 }
    //             }
    //         }

    //         let next = inner(nums, visited, index + 1);
    //         if index != 0 && visited[index - 1] {
    //             return next;
    //         } else {
    //             visited[index] = true;
    //             if index == 0 {
    //                 visited[nums.len()] = true;
    //             }
    //             let next2 = nums[index] + inner(nums, visited, index + 1);
    //             visited[index] = false;
    //             if index == 0 {
    //                 visited[nums.len()] = false;
    //             }
    //             if next < next2 {
    //                 return next2;
    //             } else {
    //                 return next;
    //             }
    //         }
    //     }

    //     let mut visited = vec![false; length + 1];

    //     inner(&nums, &mut visited, 0)
    // }

    // pub fn rob(nums: Vec<i32>) -> i32 {
    //     let n = nums.len();
    //     if n == 0 {
    //         return 0;
    //     }
    //     if n == 1 {
    //         return nums[0];
    //     }
    //     if n == 2 {
    //         return std::cmp::max(nums[0], nums[1]);
    //     }

    //     let s1 = {
    //         let mut dp = vec![0; n];
    //         dp[0] = nums[0];
    //         dp[1] = std::cmp::max(nums[0], nums[1]);
    //         for i in 2..n - 1 {
    //             dp[i] = std::cmp::max(dp[i - 1], dp[i - 2] + nums[i]);
    //         }
    //         dp[n - 2]
    //     };

    //     let s2 = {
    //         let mut dp = vec![0; n];
    //         dp[1] = nums[1];
    //         dp[2] = std::cmp::max(nums[1], nums[2]);
    //         for i in 3..n {
    //             dp[i] = std::cmp::max(dp[i - 1], dp[i - 2] + nums[i]);
    //         }
    //         dp[n - 1]
    //     };

    //     std::cmp::max(s1, s2)
    // }
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return nums[0];
        }
        if n == 2 {
            return std::cmp::max(nums[0], nums[1]);
        }

        let s1 = {
            let mut first = nums[0];
            let mut second = std::cmp::max(nums[0], nums[0 + 1]);
            for i in 2..n - 1 {
                let temp = second;
                second = std::cmp::max(first + nums[i], second);
                first = temp;
            }
            second
        };

        let s2 = {
            let mut first = nums[1];
            let mut second = std::cmp::max(nums[1], nums[2]);
            for i in 3..n {
                let temp = second;
                second = std::cmp::max(first + nums[i], second);
                first = temp;
            }
            second
        };

        std::cmp::max(s1, s2)
    }

    // pub fn rob(nums: Vec<i32>) -> i32 {
    //     let n = nums.len();
    //     if n == 0 {
    //         return 0;
    //     }
    //     if n == 1 {
    //         return nums[0];
    //     }
    //     if n == 2 {
    //         return std::cmp::max(nums[0], nums[1]);
    //     }
    //     if n == 3 {
    //         return std::cmp::max(nums[0], std::cmp::max(nums[1], nums[2]));
    //     }
    //     if n == 4 {
    //         return std::cmp::max(nums[0] + nums[2], nums[1] + nums[3]);
    //     }
    //     if n == 5 {
    //         return std::cmp::max(
    //             nums[0] + nums[2],
    //             std::cmp::max(nums[1] + nums[3], nums[2] + nums[4]),
    //         );
    //     }

    //     // 0 1 ... n-1
    //     //       2? n-2?
    //     let mut dp = vec![0; n];
    //     let v3 = nums[2] > nums[3];
    //     let vn2 = nums[n - 2] > nums[n - 3];
    //     for i in 2..n - 1 {
    //         dp[i] = std::cmp::max(dp[i - 1], dp[i - 2] + nums[i]);
    //     }
    //     match (v3, vn2) {
    //         (true, true) => {
    //             return dp[n - 2] + nums[0];
    //         }
    //         (true, false) => {
    //             return dp[n - 2] + std::cmp::max(nums[0], nums[n - 1]);
    //         }
    //         (false, true) => {
    //             return dp[n - 2] + std::cmp::max(nums[0], nums[1]);
    //         }
    //         (false, false) => {
    //             return dp[n - 2] + std::cmp::max(nums[0], nums[1] + nums[n - 1]);
    //         }
    //     }
    // }
}

fn main() {
    println!("No. {}", 213);

    let start = std::time::Instant::now();

    assert_eq!(Solution::rob(vec![2, 3, 2]), 3);

    let end = std::time::Instant::now();

    println!("spend: {:?}", end - start);
}
