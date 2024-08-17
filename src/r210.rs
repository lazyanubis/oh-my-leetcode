#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        // if prerequisites.len() == 0 {
        //     if num_courses == 1 {
        //         return vec![0];
        //     } else {
        //         return vec![];
        //     }
        // }

        use std::collections::HashMap;
        use std::collections::HashSet;
        let mut courses: HashMap<i32, HashSet<i32>> = HashMap::new();

        for c in 0..num_courses {
            courses.insert(c, HashSet::new());
        }

        for p in prerequisites {
            let pre = p[1];
            let cur = p[0];
            courses.entry(cur).or_insert_with(HashSet::new).insert(pre);
            courses.entry(pre).or_insert_with(HashSet::new);
        }

        // println!("courses: {:?}", courses);

        if num_courses as usize > courses.len() {
            return vec![];
        }

        let mut results = vec![];
        fn dfs(
            num_courses: usize,                   // 目标课程数量，满足可提前退出
            courses: &HashMap<i32, HashSet<i32>>, // 每个课程的前置课程
            stack: &mut Vec<bool>,                // 后面的课程
            results: &mut Vec<i32>,               // 记录结果
            _results: &mut Vec<bool>,
            list: &HashSet<i32>, // 前置课程
        ) -> Result<(), bool> {
            if results.len() >= num_courses {
                return Ok(());
            }
            for course in list {
                if _results[*course as usize] {
                    continue;
                }
                if stack[*course as usize] {
                    return Err(false);
                }
                match courses.get(&course) {
                    Some(cs) => {
                        stack[*course as usize] = true;
                        dfs(num_courses, courses, stack, results, _results, cs)?;
                        results.push(*course);
                        _results[*course as usize] = true;
                        stack[*course as usize] = false;
                    }
                    None => {
                        results.push(*course);
                        _results[*course as usize] = true;
                    }
                };
                // println!("results: {:?}", results);
            }
            Ok(())
        }

        let _ = dfs(
            num_courses as usize,
            &courses,
            &mut vec![false; num_courses as usize],
            &mut results,
            &mut vec![false; num_courses as usize],
            &courses.keys().into_iter().cloned().collect::<HashSet<_>>(),
        );

        if results.len() < num_courses as usize {
            return vec![];
        }

        results
    }
}

fn main() {
    println!("No. {}", 210);

    let start = std::time::Instant::now();

    // assert_eq!(Solution::find_order(2, vec![vec![1, 0]]), vec![0, 1]);
    assert_eq!(Solution::find_order(2, vec![]), vec![0, 1]);
    // assert_eq!(
    //     Solution::find_order(4, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]]),
    //     vec![0, 1, 2, 3]
    // );

    let end = std::time::Instant::now();

    println!("spend: {:?}", end - start);
}
