struct Solution;

impl Solution {
    // pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    //     use std::collections::HashSet;
    //     let mut lands: Vec<HashSet<usize>> = vec![];

    //     let rl = grid[0].len();

    //     for (r, row) in grid.iter().enumerate() {
    //         for (c, column) in row.iter().enumerate() {
    //             if *column == '0' {
    //                 continue;
    //             }
    //             let mut linked: Vec<_> = vec![];
    //             for (i, land) in lands.iter().enumerate() {
    //                 for l in land.iter() {
    //                     let x = l / rl;
    //                     let y = l % rl;
    //                     if (x + 1 == r && y == c) || (x == r && y + 1 == c) {
    //                         linked.push(i);
    //                         break;
    //                     }
    //                 }
    //             }
    //             let length = linked.len();
    //             if length >= 2 {
    //                 let mut land = HashSet::new();
    //                 for i in linked.iter().rev() {
    //                     land.extend(lands.swap_remove(*i));
    //                 }
    //                 land.insert(r * rl + c);
    //                 lands.push(land);
    //             } else if length == 1 {
    //                 lands[linked[0]].insert(r * rl + c);
    //             } else {
    //                 lands.push(HashSet::from([r * rl + c]));
    //             }
    //         }
    //     }

    //     // println!("lands: {:?}", lands);

    //     lands.len() as i32
    // }
    // pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    //     use std::collections::HashSet;
    //     let mut lands: HashSet<isize> = HashSet::new();
    //     let mut count = 0;
    //     let mut last: Vec<isize> = vec![];

    //     if grid[0][0] == '1' {
    //         lands.insert(count);
    //         last.push(count);
    //         count += 1;
    //     } else {
    //         last.push(-1);
    //     }

    //     for c in 1..grid[0].len() {
    //         let column = grid[0][c];
    //         if column == '0' {
    //             last.push(-1);
    //             continue;
    //         }
    //         let left = grid[0][c - 1];
    //         if left == '1' {
    //             last.push(count - 1);
    //         } else {
    //             lands.insert(count);
    //             last.push(count);
    //             count += 1;
    //         }
    //     }

    //     for r in 1..grid.len() {
    //         let row = &grid[r];
    //         for (c, column) in row.iter().enumerate() {
    //             if *column == '0' {
    //                 last[c] = -1;
    //                 continue;
    //             }
    //             let up = last[c];
    //             if c == 0 {
    //                 if 0 <= up {
    //                 } else {
    //                     lands.insert(count);
    //                     last[c] = count;
    //                     count += 1;
    //                 }
    //                 continue;
    //             }
    //             let left = last[c - 1];
    //             if 0 <= up {
    //                 if 0 <= left {
    //                     if up == left {
    //                     } else {
    //                         lands.remove(&up);
    //                         lands.remove(&left);
    //                         lands.insert(count);
    //                         for l in last.iter_mut() {
    //                             if *l == up || *l == left {
    //                                 *l = count;
    //                             }
    //                         }
    //                         count += 1;
    //                     }
    //                 } else {
    //                 }
    //             } else {
    //                 if 0 <= left {
    //                     last[c] = left;
    //                 } else {
    //                     lands.insert(count);
    //                     last[c] = count;
    //                     count += 1;
    //                 }
    //             }
    //         }
    //     }

    //     // println!("lands: {:?}", lands);

    //     lands.len() as i32
    // }
    // pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    //     let mut lands: Vec<isize> = Vec::new();
    //     let mut count = 0;
    //     let mut last: Vec<isize> = vec![];

    //     if grid[0][0] == '1' {
    //         lands.push(count);
    //         last.push(count);
    //         count += 1;
    //     } else {
    //         last.push(-1);
    //     }

    //     for c in 1..grid[0].len() {
    //         let column = grid[0][c];
    //         if column == '0' {
    //             last.push(-1);
    //             continue;
    //         }
    //         let left = grid[0][c - 1];
    //         if left == '1' {
    //             last.push(count - 1);
    //         } else {
    //             lands.push(count);
    //             last.push(count);
    //             count += 1;
    //         }
    //     }

    //     for r in 1..grid.len() {
    //         let row = &grid[r];
    //         for (c, column) in row.iter().enumerate() {
    //             if *column == '0' {
    //                 last[c] = -1;
    //                 continue;
    //             }
    //             let up = last[c];
    //             if c == 0 {
    //                 if 0 <= up {
    //                 } else {
    //                     lands.push(count);
    //                     last[c] = count;
    //                     count += 1;
    //                 }
    //                 continue;
    //             }
    //             let left = last[c - 1];
    //             if 0 <= up {
    //                 if 0 <= left {
    //                     if up == left {
    //                     } else {
    //                         lands.retain(|v| *v != up && *v != left);
    //                         lands.push(count);
    //                         for l in last.iter_mut() {
    //                             if *l == up || *l == left {
    //                                 *l = count;
    //                             }
    //                         }
    //                         count += 1;
    //                     }
    //                 } else {
    //                 }
    //             } else {
    //                 if 0 <= left {
    //                     last[c] = left;
    //                 } else {
    //                     lands.push(count);
    //                     last[c] = count;
    //                     count += 1;
    //                 }
    //             }
    //         }
    //     }

    //     println!("lands: {:?}", lands);

    //     lands.len() as i32
    // }
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut lands: usize = 0;
        let mut count = 0;
        let mut last: Vec<isize> = Vec::with_capacity(grid[0].len());

        if grid[0][0] == '1' {
            lands += 1;
            last.push(count);
            count += 1;
        } else {
            last.push(-1);
        }

        for c in 1..grid[0].len() {
            let column = grid[0][c];
            if column == '0' {
                last.push(-1);
                continue;
            }
            let left = grid[0][c - 1];
            if left == '1' {
                last.push(count - 1);
            } else {
                lands += 1;
                last.push(count);
                count += 1;
            }
        }

        for r in 1..grid.len() {
            let row = &grid[r];
            for (c, column) in row.iter().enumerate() {
                if *column == '0' {
                    last[c] = -1;
                    continue;
                }
                let up = last[c];
                if c == 0 {
                    if 0 <= up {
                    } else {
                        lands += 1;
                        last[c] = count;
                        count += 1;
                    }
                    continue;
                }
                let left = last[c - 1];
                if 0 <= up {
                    if 0 <= left {
                        if up == left {
                        } else {
                            lands -= 1;
                            for l in last.iter_mut() {
                                if *l == up || *l == left {
                                    *l = count;
                                }
                            }
                            count += 1;
                        }
                    } else {
                    }
                } else {
                    if 0 <= left {
                        last[c] = left;
                    } else {
                        lands += 1;
                        last[c] = count;
                        count += 1;
                    }
                }
            }
        }

        // println!("lands: {:?}", lands);

        lands as i32
    }
}

fn main() {
    println!("{}", 200);

    let start = std::time::Instant::now();

    // assert_eq!(
    //     Solution::num_islands(vec![
    //         vec!['1', '1', '1', '1', '0'],
    //         vec!['1', '1', '0', '1', '0'],
    //         vec!['1', '1', '0', '0', '0'],
    //         vec!['0', '0', '0', '0', '0']
    //     ]),
    //     1
    // );
    // assert_eq!(
    //     Solution::num_islands(vec![
    //         vec!['1', '1', '0', '0', '0'],
    //         vec!['1', '1', '0', '0', '0'],
    //         vec!['0', '0', '1', '0', '0'],
    //         vec!['0', '0', '0', '1', '1']
    //     ]),
    //     3
    // );
    // assert_eq!(
    //     Solution::num_islands(vec![
    //         vec!['1', '1', '1'],
    //         vec!['0', '1', '0'],
    //         vec!['1', '1', '1']
    //     ]),
    //     1
    // );
    assert_eq!(
        Solution::num_islands(vec![
            vec!['1', '1', '1', '1', '1', '0', '1', '1', '1', '1'],
            vec!['1', '0', '1', '0', '1', '1', '1', '1', '1', '1'],
            vec!['0', '1', '1', '1', '0', '1', '1', '1', '1', '1'],
            vec!['1', '1', '0', '1', '1', '0', '0', '0', '0', '1'],
            vec!['1', '0', '1', '0', '1', '0', '0', '1', '0', '1'],
            vec!['1', '0', '0', '1', '1', '1', '0', '1', '0', '0'],
            vec!['0', '0', '1', '0', '0', '1', '1', '1', '1', '0'],
            vec!['1', '0', '1', '1', '1', '0', '0', '1', '1', '1'],
            vec!['1', '1', '1', '1', '1', '1', '1', '1', '0', '1'],
            vec!['1', '0', '1', '1', '1', '1', '1', '1', '1', '0']
        ]),
        2
    );
    assert_eq!(
        Solution::num_islands(vec![vec!['1', '0', '1', '1', '0', '1', '1']]),
        3
    );

    let end = std::time::Instant::now();

    println!("spend: {:?}", end - start);
}
