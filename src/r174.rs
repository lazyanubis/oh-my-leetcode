struct Solution;

// impl Solution {
//     pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
//         if dungeon.len() == 1 && dungeon[0].len() == 1 {
//             if dungeon[0][0] >= 0 {
//                 return 1;
//             } else {
//                 return -dungeon[0][0] + 1;
//             }
//         }

//         use std::collections::HashMap;

//         let mut health: HashMap<(usize, usize), (i32, i32)> = HashMap::new();

//         let alive = |x: i32| if x >= 0 { 1 } else { -x + 1 };

//         health.insert((0, 0), (dungeon[0][0], alive(dungeon[0][0])));

//         loop {
//             println!("{:?}", health);
//             println!();

//             let mut health2: HashMap<(usize, usize), (i32, i32)> = HashMap::new();

//             for ((r, c), vs) in &health {
//                 let mut count = |nr: usize, nc: usize| {
//                     let damage = dungeon[nr][nc];

//                     let (a, c) = vs;

//                     let a = a + damage;
//                     let new_alive = alive(a);
//                     let new_alive = if new_alive < *c { *c } else { new_alive };

//                     if health2.contains_key(&(nr, nc)) {
//                         let (_, c2) = health2[&(nr, nc)];
//                         if new_alive < c2 {
//                             health2.insert((nr, nc), (a, new_alive));
//                         }
//                     } else {
//                         health2.insert((nr, nc), (a, new_alive));
//                     }
//                 };

//                 if r + 1 < dungeon.len() {
//                     count(r + 1, *c);
//                 }

//                 if c + 1 < dungeon[0].len() {
//                     count(*r, c + 1);
//                 }
//             }

//             health = health2;

//             if health.contains_key(&(dungeon.len() - 1, dungeon[0].len() - 1)) {
//                 break;
//             }
//         }

//         println!("{:?}", dungeon);
//         println!("{:?}", health);

//         health[&(dungeon.len() - 1, dungeon[0].len() - 1)].1
//     }
// }

// impl Solution {
//     pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
//         if dungeon.len() == 1 && dungeon[0].len() == 1 {
//             if dungeon[0][0] >= 0 {
//                 return 1;
//             } else {
//                 return -dungeon[0][0] + 1;
//             }
//         }

//         let mut hp = i32::MAX;

//         fn count(dungeon: &Vec<Vec<i32>>, r: usize, c: usize, sum: i32, need: i32, hp: &mut i32) {
//             // println!("{},{} {} {}  {}", r, c, sum, need, hp);
//             let mut next = Vec::with_capacity(2);

//             if r < dungeon.len() - 1 {
//                 next.push((r + 1, c));
//             }

//             if c < dungeon[0].len() - 1 {
//                 next.push((r, c + 1));
//             }

//             if next.len() == 0 {
//                 if need < *hp {
//                     *hp = need;
//                 }
//                 // println!();
//                 return;
//             }

//             for (nr, nc) in next {
//                 let damage = dungeon[nr][nc];

//                 let sum = sum + damage;

//                 let need2 = if sum >= 0 { 1 } else { -sum + 1 };

//                 let need = if need2 > need { need2 } else { need };

//                 count(dungeon, nr, nc, sum, need, hp);
//             }
//         }

//         let alive = |x: i32| if x >= 0 { 1 } else { -x + 1 };

//         count(&dungeon, 0, 0, dungeon[0][0], alive(dungeon[0][0]), &mut hp);

//         hp
//     }
// }

// impl Solution {
//     pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
//         if dungeon.len() == 1 && dungeon[0].len() == 1 {
//             if dungeon[0][0] >= 0 {
//                 return 1;
//             } else {
//                 return -dungeon[0][0] + 1;
//             }
//         }

//         let mut cache = Vec::with_capacity(dungeon.len());
//         let mut temp = Vec::with_capacity(dungeon[0].len());
//         for _ in 0..dungeon[0].len() {
//             temp.push(0);
//         }
//         for _ in 0..dungeon.len() {
//             cache.push(temp.clone());
//         }

//         fn min_hp(dungeon: &Vec<Vec<i32>>, r: usize, c: usize, cache: &mut Vec<Vec<i32>>) -> i32 {
//             if 0 != cache[r][c] {
//                 return cache[r][c];
//             }

//             let mut next = Vec::with_capacity(2);

//             if r < dungeon.len() - 1 {
//                 next.push((r + 1, c));
//             }

//             if c < dungeon[0].len() - 1 {
//                 next.push((r, c + 1));
//             }

//             let damage = dungeon[r][c];

//             if next.len() == 0 {
//                 let hp = if damage < 0 { -damage + 1 } else { 1 };
//                 // println!("{},{} -> {}", r, c, hp);
//                 cache[r][c] = hp;
//                 return hp;
//             }

//             let min = next
//                 .iter()
//                 .map(|&(nr, nc)| min_hp(dungeon, nr, nc, cache))
//                 .min();

//             let min = match min {
//                 Some(v) => v,
//                 None => 0,
//             };

//             let hp = if damage > 0 {
//                 if damage >= min {
//                     1
//                 } else {
//                     min - damage
//                 }
//             } else {
//                 -damage + min
//             };

//             // println!("{},{} -> {}", r, c, hp);
//             cache[r][c] = hp;
//             return hp;
//         }

//         min_hp(&dungeon, 0, 0, &mut cache)
//     }
// }

impl Solution {
    pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
        if dungeon.len() == 1 && dungeon[0].len() == 1 {
            if dungeon[0][0] >= 0 {
                return 1;
            } else {
                return -dungeon[0][0] + 1;
            }
        }

        let mut dungeon = dungeon.clone();

        const MAX: i32 = 100000000;

        // println!("{:?} {}", dungeon, MAX);

        fn min_hp(dungeon: &mut Vec<Vec<i32>>, r: usize, c: usize) -> i32 {
            if MAX < dungeon[r][c] {
                return dungeon[r][c] - MAX;
            }

            let mut next = Vec::with_capacity(2);

            if r < dungeon.len() - 1 {
                next.push((r + 1, c));
            }

            if c < dungeon[0].len() - 1 {
                next.push((r, c + 1));
            }

            let damage = dungeon[r][c];

            if next.len() == 0 {
                let hp = if damage < 0 { -damage + 1 } else { 1 };
                // println!("{},{} -> {}", r, c, hp);
                dungeon[r][c] = hp + MAX;
                return hp;
            }

            let min = next.iter().map(|&(nr, nc)| min_hp(dungeon, nr, nc)).min();

            let min = match min {
                Some(v) => v,
                None => 0,
            };

            let hp = if damage > 0 {
                if damage >= min {
                    1
                } else {
                    min - damage
                }
            } else {
                -damage + min
            };

            // println!("{},{} -> {}", r, c, hp);
            dungeon[r][c] = hp + MAX;
            return hp;
        }

        min_hp(&mut dungeon, 0, 0)
    }
}

fn main() {
    println!("{}", 174);

    // let dungeon = vec![vec![0, 0]];
    // let dungeon = vec![vec![2, 1]];
    // let dungeon = vec![vec![2, 0, -1]];
    // let dungeon = vec![vec![-2, -3, 3], vec![-5, -10, 1], vec![10, 30, -5]];
    // let dungeon = vec![vec![1, -3, 3], vec![0, -2, 0], vec![-3, -3, -3]];
    // [[1,-3,3],
    // [0,-2,0],
    // [-3,-3,-3]]
    // let dungeon = vec![vec![0, 5], vec![-2, -3]];
    let dungeon = vec![vec![1, -2, 3], vec![2, -2, -2]];
    // 1 -2 3
    // 2 -2 -2

    let hp = Solution::calculate_minimum_hp(dungeon);

    println!("{}", hp);
}
