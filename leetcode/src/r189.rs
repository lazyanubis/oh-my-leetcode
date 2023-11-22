struct Solution;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let length = nums.len();
        let k = k as usize % length;
        if k == 0 {
            return;
        }

        let mut a = length;
        let mut b = k;
        while a != b {
            if a > b {
                a = a - b;
            } else {
                b = b - a;
            }
        }

        for i in 0..a {
            // println!("i ========= {:?} {}", i, k);
            let mut ii = i;
            let mut next = (ii + k) % length;
            let vv = nums[ii];
            let mut vv = std::mem::replace(&mut nums[next], vv);
            ii = next;
            // println!("nums: {:?}", nums);
            loop {
                next = (ii + k) % length;
                if next == i {
                    nums[i] = vv;
                    break;
                }
                vv = std::mem::replace(&mut nums[next], vv);
                ii = next;
                // println!("nums: {:?}", nums);
            }
        }
    }
}

fn main() {
    println!("{}", 189);

    let start = std::time::Instant::now();

    let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
    Solution::rotate(&mut nums, 3);
    assert_eq!(nums, vec![5, 6, 7, 1, 2, 3, 4]);

    nums = vec![-1, -100, 3, 99];
    Solution::rotate(&mut nums, 2);
    assert_eq!(nums, vec![3, 99, -1, -100]);

    nums = vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 27,
    ];
    Solution::rotate(&mut nums, 38);
    assert_eq!(
        nums,
        vec![
            17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13,
            14, 15, 16
        ]
    );

    let end = std::time::Instant::now();

    println!("spend: {:?}", end - start);
}
