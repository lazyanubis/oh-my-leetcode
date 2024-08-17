#![allow(dead_code)]

struct Solution;

impl Solution {
    // pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    //     let mut record: Vec<u8> = vec![0; num_courses as usize];
    //     let mut checked_prerequisites = vec![false; prerequisites.len()];

    //     fn check(
    //         course: usize,
    //         record: &mut Vec<u8>,
    //         checked_prerequisites: &mut Vec<bool>,
    //         prerequisites: &Vec<Vec<i32>>,
    //     ) -> bool {
    //         let r = record[course];
    //         match r {
    //             0 => {
    //                 record[course] = 1;
    //                 for i in 0..prerequisites.len() {
    //                     if checked_prerequisites[i] {
    //                         continue;
    //                     } else {
    //                         let p = &prerequisites[i];
    //                         if p[0] as usize == course {
    //                             for c in p[1..].iter() {
    //                                 if !check(
    //                                     *c as usize,
    //                                     record,
    //                                     checked_prerequisites,
    //                                     prerequisites,
    //                                 ) {
    //                                     return false;
    //                                 }
    //                             }
    //                             checked_prerequisites[i] = true;
    //                         }
    //                     }
    //                 }
    //                 record[course] = 2;
    //             }
    //             1 => return false,
    //             2 => return true,
    //             _ => panic!(),
    //         }
    //         true
    //     }

    //     for i in 0..num_courses as usize {
    //         if !check(i, &mut record, &mut checked_prerequisites, &prerequisites) {
    //             return false;
    //         }
    //     }

    //     true
    // }
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut record = std::collections::HashMap::new();
        for i in 0..num_courses as usize {
            record.insert(i, vec![]);
        }
        for mut p in prerequisites {
            let index = p[0] as usize;
            p.remove(0);
            record.get_mut(&index).unwrap().push(p);
        }
        loop {
            let mut changed = false;

            let removed = record
                .iter()
                .filter(|(_, ps)| !ps.iter().any(|p| !p.is_empty()))
                .map(|(i, _)| *i)
                .collect::<std::collections::HashSet<_>>();
            if removed.is_empty() {
                break;
            }

            record.retain(|_, ps| ps.iter().any(|p| !p.is_empty()));
            record.values_mut().for_each(|ps| {
                for p in ps.iter_mut() {
                    if p.is_empty() {
                        continue;
                    }
                    let pp = p[p.len() - 1] as usize;
                    if removed.contains(&pp) {
                        p.remove(p.len() - 1);
                        changed = true;
                    }
                }
            });

            if !changed || record.is_empty() {
                break;
            }
        }

        record.is_empty()
    }
}

fn main() {
    println!("{}", 207);

    let start = std::time::Instant::now();

    assert_eq!(Solution::can_finish(2, vec![vec![1, 0]]), true);
    assert_eq!(Solution::can_finish(2, vec![vec![1, 0], vec![0, 1]]), false);
    assert_eq!(
        Solution::can_finish(3, vec![vec![1, 0], vec![1, 2], vec![0, 1]]),
        false
    );

    let end = std::time::Instant::now();

    println!("spend: {:?}", end - start);
}
