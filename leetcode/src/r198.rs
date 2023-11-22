#[allow(dead_code)]
struct Solution;

impl Solution {
    // #[allow(dead_code)]
    // pub fn rob(nums: Vec<i32>) -> i32 {
    //     use std::collections::HashMap;
    //     fn inner(
    //         nums: &[i32],
    //         start: usize,
    //         end: usize,
    //         cached: &mut HashMap<(usize, usize), i32>,
    //     ) -> i32 {
    //         let key = (start, end);
    //         if let Some(v) = cached.get(&key) {
    //             return *v;
    //         }
    //         let mut max = i32::MIN;
    //         for i in start..end {
    //             let left = if i == 0 || i - 1 <= start {
    //                 0
    //             } else if 2 <= i && start == i - 2 {
    //                 nums[start]
    //             } else {
    //                 inner(nums, start, i - 1, cached)
    //             };
    //             let right = if end <= i + 2 {
    //                 0
    //             } else if i + 2 == end - 1 {
    //                 nums[i + 2]
    //             } else {
    //                 inner(nums, i + 2, end, cached)
    //             };
    //             let current = left + nums[i] + right;
    //             if max < current {
    //                 max = current;
    //             }
    //         }
    //         cached.insert(key, max);
    //         max
    //     }
    //     let mut cached: HashMap<(usize, usize), i32> = HashMap::new();
    //     inner(&nums[..], 0, nums.len(), &mut cached)
    // }
    // #[allow(dead_code)]
    // pub fn rob(nums: Vec<i32>) -> i32 {
    //     if nums.len() == 0 {
    //         return 0;
    //     }
    //     if nums.len() == 1 {
    //         return nums[0];
    //     }
    //     let mut nums = nums;
    //     if nums[1] < nums[0] {
    //         nums[1] = nums[0];
    //     }
    //     for i in 2..nums.len() {
    //         let sum = nums[i - 2] + nums[i];
    //         nums[i] = if nums[i - 1] < sum { sum } else { nums[i - 1] };
    //     }

    //     *nums.last().unwrap()
    // }
    #[allow(dead_code)]
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        if nums.len() == 1 {
            return nums[0];
        }
        let mut f0 = nums[0];
        let mut f1 = if nums[1] < nums[0] { nums[0] } else { nums[1] };

        for i in 2..nums.len() {
            let sum = f0 + nums[i];
            let n = if f1 < sum { sum } else { f1 };
            f0 = f1;
            f1 = n;
        }

        f1
    }
}

fn main() {
    println!("{}", 198);

    let start = std::time::Instant::now();

    assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
    assert_eq!(Solution::rob(vec![2, 7, 9, 3, 1]), 12);
    assert_eq!(Solution::rob(vec![2, 7, 9, 7, 1]), 14);

    let end = std::time::Instant::now();

    println!("spend: {:?}", end - start);
}
