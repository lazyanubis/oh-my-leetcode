#![allow(dead_code)]
struct Solution;

// impl Solution {
//     pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
//         let mut sum = vec![0; nums.len()];
//         for i in 1..=nums.len() {
//             for j in 0..=(nums.len() - i) {
//                 sum[j] += nums[j + i - 1];
//                 if sum[j] >= target {
//                     println!("{:?}", j);
//                     return i as i32;
//                 }
//             }
//         }
//         0
//     }
// }

// xxxx
// impl Solution {
//     pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
//         let sum: i32 = nums.iter().sum();

//         if sum < target {
//             return 0;
//         }

//         if sum == target {
//             return nums.len() as i32;
//         }

//         fn min(target: i32, sum: i32, nums: &[i32]) -> i32 {
//             let l = nums[0];
//             let r = nums[nums.len() - 1];

//             let ll = {
//                 let sum = sum - l;
//                 if sum < target {
//                     nums.len() as i32
//                 } else {
//                     min(target, sum, &nums[1..])
//                 }
//             };
//             let rr = {
//                 let sum = sum - r;
//                 if sum < target {
//                     nums.len() as i32
//                 } else {
//                     min(target, sum, &nums[..nums.len() - 1])
//                 }
//             };
//             println!("{:?}", nums);
//             if ll < rr {
//                 ll
//             } else {
//                 rr
//             }

//             // if l < r {
//             //     let sum = sum - l;
//             //     if sum < target {
//             //         return nums.len() as i32;
//             //     }
//             //     return min(target, sum, &nums[1..]);
//             // } else if l > r {
//             //     let sum = sum - r;
//             //     if sum < target {
//             //         return nums.len() as i32;
//             //     }
//             //     return min(target, sum, &nums[..nums.len() - 1]);
//             // } else {
//             //     let sum = sum - l;
//             //     if sum < target {
//             //         return nums.len() as i32;
//             //     }
//             //     let ll = min(target, sum, &nums[1..]);
//             //     let rr = min(target, sum, &nums[..nums.len() - 1]);
//             //     if ll < rr {
//             //         ll
//             //     } else {
//             //         rr
//             //     }
//             // }
//         }

//         min(target, sum, &nums)
//     }
// }

// impl Solution {
//     pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
//         let sum: i32 = nums.iter().sum();

//         if sum < target {
//             return 0;
//         }

//         if sum == target {
//             return nums.len() as i32;
//         }

//         use std::collections::HashMap;
//         let mut cached: HashMap<usize, HashMap<usize, i32>> = Default::default();
//         fn min(
//             target: i32,
//             sum: i32,
//             nums: &[i32],
//             start: usize,
//             end: usize,
//             cached: &mut HashMap<usize, HashMap<usize, i32>>,
//         ) -> i32 {
//             let l = nums[start];
//             let r = nums[end - 1];
//             if let Some(x) = cached.get(&start).and_then(|c| c.get(&end)) {
//                 return *x;
//             }

//             let ll = {
//                 let sum = sum - l;
//                 if sum < target {
//                     (end - start) as i32
//                 } else {
//                     min(target, sum, nums, start + 1, end, cached)
//                 }
//             };
//             let rr = {
//                 let sum = sum - r;
//                 if sum < target {
//                     (end - start) as i32
//                 } else {
//                     min(target, sum, nums, start, end - 1, cached)
//                 }
//             };
//             let value = if ll < rr { ll } else { rr };

//             cached
//                 .entry(start)
//                 .or_insert_with(|| Default::default())
//                 .insert(end, value);
//             value
//         }

//         min(target, sum, &nums, 0, nums.len(), &mut cached)
//     }
// }

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut sum = vec![0; nums.len() + 1];
        for i in 0..nums.len() {
            sum[i + 1] = sum[i] + nums[i];
        }

        let total = sum[nums.len()];

        if total < target {
            return 0;
        }

        if total == target {
            return nums.len() as i32;
        }

        #[inline]
        fn find_sum(sum: &[i32], start: usize, n: usize) -> i32 {
            if n == 0 {
                return 0;
            }
            sum[start + n] - sum[start]
        }

        fn fix_left(target: i32, sum: &[i32], fix: usize, min: usize, max: usize) -> usize {
            match max - min {
                0 => return min,
                1 => return max,
                _ => {
                    let mid = (max + min) / 2;
                    let s = find_sum(sum, fix, mid);
                    if s < target {
                        return fix_left(target, sum, fix, mid, max);
                    } else if s == target {
                        return mid;
                    } else {
                        return fix_left(target, sum, fix, min, mid);
                    }
                }
            }
        }
        fn fix_right(target: i32, sum: &[i32], fix: usize, min: usize, max: usize) -> usize {
            match max - min {
                0 => return min,
                1 => return max,
                _ => {
                    let mid = (max + min) / 2;
                    let s = find_sum(sum, fix - mid, mid);
                    if s < target {
                        return fix_right(target, sum, fix, mid, max);
                    } else if s == target {
                        return mid;
                    } else {
                        return fix_right(target, sum, fix, min, mid);
                    }
                }
            }
        }

        fn min(
            target: i32,
            nums: &[i32],
            sum: &[i32],
            start: usize,
            end: usize,
            count: i32,
        ) -> i32 {
            let l = nums[start];
            let r = nums[end - 1];

            if count - l >= target && count - r >= target {
                let left = fix_left(target, sum, start, 0, end - start);
                let right = fix_right(target, sum, end, 0, end - start);
                if left < right {
                    return min(target, nums, sum, start, end - 1, count - r);
                } else {
                    return min(target, nums, sum, start + 1, end, count - l);
                }
            }

            let ll = {
                let count = count - l;
                if count < target {
                    (end - start) as i32
                } else {
                    min(target, nums, sum, start + 1, end, count)
                }
            };
            let rr = {
                let count = count - r;
                if count < target {
                    (end - start) as i32
                } else {
                    min(target, nums, sum, start, end - 1, count)
                }
            };
            if ll < rr {
                ll
            } else {
                rr
            }
        }

        min(target, &nums, &sum, 0, nums.len(), total)
    }
}

fn main() {
    println!("No. {}", 209);

    let start = std::time::Instant::now();

    assert_eq!(Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
    assert_eq!(Solution::min_sub_array_len(4, vec![1, 4, 4]), 1);
    assert_eq!(
        Solution::min_sub_array_len(11, vec![1, 1, 1, 1, 1, 1, 1, 1]),
        0
    );
    assert_eq!(
        Solution::min_sub_array_len(
            697439,
            // ,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,
            vec![
                5334, 6299, 4199, 9663, 8945, 3566, 9509, 3124, 6026, 6250, 7475, 5420, 9201, 9501,
                38, 5897, 4411, 6638, 9845, 161, 9563, 8854, 3731, 5564, 5331, 4294, 3275, 1972,
                1521, 2377, 3701, 6462, 6778, 187, 9778, 758, 550, 7510, 6225, 8691, 3666, 4622,
                9722, 8011, 7247, 575, 5431, 4777, 4032, 8682, 5888, 8047, 3562, 9462, 6501, 7855,
                505, 4675, 6973, 493, 1374, 3227, 1244, 7364, 2298, 3244, 8627, 5102, 6375, 8653,
                1820, 3857, 7195, 7830, 4461, 7821, 5037, 2918, 4279, 2791, 1500, 9858, 6915, 5156,
                970, 1471, 5296, 1688, 578, 7266, 4182, 1430, 4985, 5730, 7941, 3880, 607,
                8776, //
                1348, 2974, 1094, 6733, 5177, 4975, 5421, 8190, 8255, 9112, 8651, 2797, 335, 8677,
                3754, 893, 1818, 8479, 5875, 1695, 8295, 7993, 7037, 8546, 7906, 4102, 7279, 1407,
                2462, 4425, 2148, 2925, 3903, 5447, 5893, 3534, 3663, 8307, 8679, 8474, 1202, 3474,
                2961, 1149, 7451, 4279, 7875, 5692, 6186, 8109, 7763, 7798, 2250, 2969, 7974, 9781,
                7741, 4914, 5446, 1861, 8914, 2544, 5683, 8952, 6745, 4870, 1848, 7887, 6448, 7873,
                128, 3281, 794, 1965, 7036, 8094, 1211, 9450, 6981, 4244, 2418, 8610, 8681, 2402,
                2904, 7712, 3252, 5029, 3004, 5526, 6965, 8866, 2764, 600, 631, 9075, 2631, 3411,
                2737, 2328, 652, 494, 6556, 9391, 4517, 8934, 8892, 4561, 9331, 1386, 4636, 9627,
                5435, 9272, 110, 413, 9706, 5470, 5008, 1706, 7045, 9648, 7505, 6968, 7509, 3120,
                7869, 6776, 6434, 7994, 5441, 288, 492, 1617, 3274, 7019, 5575, 6664, 6056, 7069,
                1996, 9581, 3103, 9266, 2554, 7471, 4251, 4320, 4749, 649, 2617, 3018, 4332, 415,
                2243, 1924, 69, 5902, 3602, 2925, 6542, 345, 4657, 9034, 8977, 6799, 8397, 1187,
                3678, 4921, 6518, 851, 6941, 6920, 259, 4503, 2637, 7438, 3893, 5042, 8552, 6661,
                5043, 9555, 9095, 4123, 142, 1446, 8047, 6234, 1199, 8848, 5656, 1910, 3430, 2843,
                8043, 9156, 7838, 2332, 9634, 2410, 2958, 3431, 4270, 1420, 4227, 7712, 6648, 1607,
                1575, 3741, 1493, 7770, 3018, 5398, 6215, 8601, 6244, 7551, 2587, 2254, 3607, 1147,
                5184, 9173, 8680, 8610, 1597, 1763, 7914, 3441, 7006, 1318, 7044, 7267, 8206, 9684,
                4814, 9748, 4497, 2239
            ]
        ),
        132
    );

    let end = std::time::Instant::now();

    println!("spend: {:?}", end - start);
}
