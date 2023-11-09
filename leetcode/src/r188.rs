struct Solution;

// impl Solution {
//     pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
//         if k == 0 || prices.len() < 2 {
//             return 0;
//         }

//         #[derive(Debug)]
//         struct Phase(i32, i32);

//         let mut phases: Vec<Phase> = vec![];

//         let mut start = prices[0];
//         let mut end = prices[0];

//         for i in 1..prices.len() {
//             // println!("{}: {} - {}", i, start, end);
//             let price = prices[i];
//             if end <= price {
//                 end = price;
//                 if i == prices.len() - 1 {
//                     if start < end {
//                         phases.push(Phase(start, end));
//                     }
//                 } else {
//                     continue;
//                 }
//             } else {
//                 if start < end {
//                     phases.push(Phase(start, end));
//                 }
//                 start = price;
//                 end = price;
//             }
//         }

//         println!("{:?}", phases);

//         fn sum(ps: &Vec<Phase>) -> i32 {
//             return ps.iter().map(|p| p.1 - p.0).filter(|r| *r > 0).sum();
//         }

//         fn get_max(k: i32, ps: &Vec<Phase>) -> i32 {
//             // println!("0 {:?}", ps);
//             if k >= ps.len() as i32 {
//                 // println!("sum {:#?}", ps);
//                 return sum(ps);
//             }
//             let mut max = 0;
//             // 以合并的方式缩短
//             for i in 1..ps.len() {
//                 let mut phases2: Vec<Phase> = vec![];
//                 for j in 0..i - 1 {
//                     phases2.push(Phase(ps[j].0, ps[j].1));
//                 }
//                 phases2.push(Phase(ps[i - 1].0, ps[i].1));
//                 for j in i + 1..ps.len() {
//                     phases2.push(Phase(ps[j].0, ps[j].1));
//                 }

//                 // println!("1 {:?} merge {} {}", phases2, i - 1, i);
//                 let m = get_max(k, &phases2);
//                 if m > max {
//                     max = m;
//                 }
//             }
//             // 以丢弃的方式缩短
//             for i in 0..ps.len() {
//                 let mut phases2: Vec<Phase> = vec![];

//                 for j in 0..ps.len() {
//                     if i != j {
//                         phases2.push(Phase(ps[j].0, ps[j].1));
//                     }
//                 }

//                 // println!("2 {:?} remove {}", phases2, i);
//                 let m = get_max(k, &phases2);
//                 if m > max {
//                     max = m;
//                 }
//             }
//             max
//         }

//         get_max(k, &phases)
//     }
// }

// impl Solution {
//     pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
//         if k == 0 || prices.len() < 2 {
//             return 0;
//         }
//         let k = k as usize;
//         use std::collections::HashMap;
//         #[derive(Debug)]
//         struct Phase(i32, i32);

//         let mut phases: Vec<Phase> = vec![];

//         let mut start = prices[0];
//         let mut end = prices[0];

//         for i in 1..prices.len() {
//             // println!("{}: {} - {}", i, start, end);
//             let price = prices[i];
//             if end <= price {
//                 end = price;
//                 if i == prices.len() - 1 {
//                     if start < end {
//                         phases.push(Phase(start, end));
//                     }
//                 } else {
//                     continue;
//                 }
//             } else {
//                 if start < end {
//                     phases.push(Phase(start, end));
//                 }
//                 start = price;
//                 end = price;
//             }
//         }

//         println!(
//             "{} {:?} [{}]",
//             phases.len(),
//             1,
//             phases
//                 .iter()
//                 .map(|p| format!("{},{}", p.0, p.1))
//                 .collect::<Vec<String>>()
//                 .join(",")
//         );

//         fn trim(phases: &mut Vec<Phase>, k: usize) {
//             if phases.len() <= 6 {
//                 return;
//             }
//             let mut changed = true;
//             while changed {
//                 if phases.len() <= k {
//                     break;
//                 }
//                 changed = false;

//                 let mut k_map = HashMap::new();
//                 let mut k_min = 0;
//                 for (i, p) in phases.iter().enumerate() {
//                     let gap = p.1 - p.0;
//                     if gap > k_min || k_map.len() < k {
//                         k_map.insert(i, gap);
//                         let mut list: Vec<(usize, i32)> =
//                             k_map.iter().map(|(index, g)| (*index, *g)).collect();
//                         list.sort_by(|a, b| b.1.cmp(&a.1));
//                         while k_map.len() > k {
//                             let index = list.pop().unwrap().0;
//                             k_map.remove(&index);
//                         }
//                         k_min = list.last().unwrap().1;
//                     }
//                 }
//                 println!("k_map: {:?}", k_map);

//                 // for i in (1..phases.len()).rev() {
//                 //     let p1 = &phases[i - 1];
//                 //     let p2 = &phases[i];
//                 //     if p1.0 < p2.0 && p2.1 < p1.1 {
//                 //         phases.remove(i);
//                 //         changed = true;
//                 //     }
//                 // }
//                 // for i in (0..phases.len() - 1).rev() {
//                 //     let p1 = &phases[i];
//                 //     let p2 = &phases[i + 1];
//                 //     if p1.0 > p2.0 && p2.1 > p1.1 {
//                 //         phases.remove(i);
//                 //     }
//                 // }
//                 // for i in (1..phases.len() - 1).rev() {
//                 //     if k_map.contains_key(&i) {
//                 //         continue;
//                 //     }
//                 //     let p1 = &phases[i - 1];
//                 //     let p2 = &phases[i];
//                 //     let p3 = &phases[i + 1];
//                 //     if p1.0 <= p2.0 && p2.1 <= p1.1 && p3.0 <= p2.0 && p2.1 <= p3.1 {
//                 //         phases.remove(i);
//                 //         changed = true;
//                 //     }
//                 // }
//                 for i in (1..phases.len() - 2).rev() {
//                     if k_map.contains_key(&i) || k_map.contains_key(&(i + 1)) {
//                         continue;
//                     }
//                     let p1 = &phases[i - 1];
//                     let p2 = &phases[i];
//                     let p3 = &phases[i + 1];
//                     let p4 = &phases[i + 2];
//                     if p1.0 <= p2.0
//                         && p2.1 <= p1.1
//                         && p4.0 <= p2.0
//                         && p2.1 <= p4.1
//                         && p1.0 <= p3.0
//                         && p3.1 <= p1.1
//                         && p4.0 <= p3.0
//                         && p3.1 <= p4.1
//                     {
//                         phases.remove(i + 1);
//                         phases.remove(i);
//                         changed = true;
//                     }
//                 }
//                 for i in (1..phases.len() - 3).rev() {
//                     if k_map.contains_key(&i) || k_map.contains_key(&(i + 1)) {
//                         continue;
//                     }
//                     let p1 = &phases[i - 1];
//                     let p2 = &phases[i];
//                     let p3 = &phases[i + 1];
//                     let p4 = &phases[i + 2];
//                     let p5 = &phases[i + 3];
//                     if p1.0 <= p2.0
//                         && p2.1 <= p1.1
//                         && p5.0 <= p2.0
//                         && p2.1 <= p5.1
//                         && p1.0 <= p3.0
//                         && p3.1 <= p1.1
//                         && p5.0 <= p3.0
//                         && p3.1 <= p5.1
//                         && p1.0 <= p4.0
//                         && p4.1 <= p1.1
//                         && p5.0 <= p4.0
//                         && p4.1 <= p5.1
//                     {
//                         phases.remove(i + 1);
//                         phases.remove(i);
//                         changed = true;
//                     }
//                 }
//             }
//         }

//         trim(&mut phases, k);

//         println!(
//             "{} {:?} [{}]",
//             phases.len(),
//             1,
//             phases
//                 .iter()
//                 .map(|p| format!("{},{}", p.0, p.1))
//                 .collect::<Vec<String>>()
//                 .join(",")
//         );

//         let mut changed = true;
//         'outer: while changed {
//             trim(&mut phases, k);
//             println!(
//                 "{} {:?} [{}]",
//                 phases.len(),
//                 1,
//                 phases
//                     .iter()
//                     .map(|p| format!("{},{}", p.0, p.1))
//                     .collect::<Vec<String>>()
//                     .join(",")
//             );

//             changed = false;
//             // 1. 统计一下前 k 个
//             let mut k_map = HashMap::new();
//             let mut k_min = 0;
//             for (i, p) in phases.iter().enumerate() {
//                 let gap = p.1 - p.0;
//                 if gap > k_min || k_map.len() < k {
//                     k_map.insert(i, gap);
//                     let mut list: Vec<(usize, i32)> =
//                         k_map.iter().map(|(index, g)| (*index, *g)).collect();
//                     list.sort_by(|a, b| b.1.cmp(&a.1));
//                     while k_map.len() > k {
//                         let index = list.pop().unwrap().0;
//                         k_map.remove(&index);
//                     }
//                     k_min = list.last().unwrap().1;
//                 }
//             }
//             println!("k_map: {:?}", k_map);
//             println!("k_min: {}", k_min);
//             // 2. 从后往前遍历，记录每一个位置可以向后合并的最大位置
//             let mut k_map2 = HashMap::new();
//             let mut end = phases.len();
//             for ii in (0..phases.len()).rev() {
//                 if k_map.contains_key(&ii) {
//                     let mut iii = ii;
//                     while iii < end {
//                         k_map2.insert(iii, end);
//                         iii += 1;
//                     }
//                     end = ii;
//                 }
//             }
//             let mut iii = 0;
//             while iii < end {
//                 k_map2.insert(iii, end);
//                 iii += 1;
//             }
//             println!("k_map2: {:?}", k_map2.len());
//             // 3. 从后往前遍历，检查是否可以合并
//             let mut k_map3: HashMap<usize, (i32, i32)> = HashMap::new();
//             for n in 1..phases.len() {
//                 // 3. 先比较第一个
//                 let length = phases.len();
//                 for i in (0..length).rev() {
//                     if !k_map.contains_key(&i) {
//                         let next = i + n;
//                         if next < k_map2[&i] {
//                             let c = &phases[i];
//                             let p = &phases[next];
//                             // 先比较是否要插入
//                             let status = k_map3.get(&i);
//                             let status = if let Some(status) = status {
//                                 if p.0 < status.0 {
//                                     (p.0, p.1)
//                                 } else {
//                                     (status.0, p.1)
//                                 }
//                             } else {
//                                 (p.0, p.1)
//                             };
//                             if c.1 < status.1 {
//                                 let gap = status.1 - status.0;
//                                 if gap < k_min * 2 / 4 {
//                                     // 可以合并
//                                     changed = true;
//                                     let np = Phase(c.0, status.1);
//                                     for d in (i + 1..=next).rev() {
//                                         phases.remove(d);
//                                     }
//                                     phases[i] = np;
//                                     println!("{} {:?}", phases.len(), 1);
//                                     continue 'outer;
//                                 } else {
//                                     k_map2.insert(i, next); // 禁止到这个位置
//                                 }
//                             }
//                             k_map3.insert(i, status);
//                         }
//                     }
//                 }
//             }
//         }

//         println!(
//             "{} [{}]",
//             phases.len(),
//             phases
//                 .iter()
//                 .map(|p| format!("{},{}", p.0, p.1))
//                 .collect::<Vec<String>>()
//                 .join(",")
//         );

//         fn sum(phases: &Vec<Phase>, start: usize, end: usize) -> i32 {
//             let mut max = 0;
//             for i in start..end {
//                 max += phases[i].1 - phases[i].0;
//             }
//             max
//         }

//         fn get_max(
//             phases: &Vec<Phase>,
//             start: usize,
//             end: usize,
//             k: usize,
//             cache: &mut HashMap<(usize, usize, usize), i32>,
//         ) -> i32 {
//             if k == 0 {
//                 return 0;
//             }
//             // if start + 1 == end {
//             //     return phases[start].1 - phases[start].0;
//             // }
//             let key = (start, end, k);
//             if let Some(max) = cache.get(&key) {
//                 return *max;
//             }
//             let max = if k >= end - start {
//                 // println!("sum {:#?}", ps);
//                 sum(phases, start, end)
//             } else if k == 1 {
//                 let mut max = 0;
//                 for i in start..end {
//                     for j in i..end {
//                         let gap = phases[j].1 - phases[i].0;
//                         if gap > max {
//                             max = gap;
//                         }
//                     }
//                 }
//                 max
//             } else {
//                 let mut max = 0;
//                 // 以合并的方式缩短
//                 for i in start..end - 1 {
//                     for kk in 0..=k {
//                         let left = get_max(phases, start, i + 1, kk, cache);
//                         let right = get_max(phases, i + 1, end, k - kk, cache);
//                         let m = left + right;
//                         if max < m {
//                             max = m;
//                         }
//                     }
//                 }
//                 max
//             };
//             cache.insert(key, max);
//             max
//         }

//         let mut cache: HashMap<_, _> = HashMap::new();
//         get_max(&phases, 0, phases.len(), k as usize, &mut cache)
//     }
// }

impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        if k == 0 || prices.len() < 2 {
            return 0;
        }
        let k = k as usize;
        use std::collections::HashMap;
        #[derive(Debug)]
        struct Up(i32, i32);

        let mut ups: Vec<Up> = vec![];

        let mut start = prices[0];
        let mut end = prices[0];

        for i in 1..prices.len() {
            // println!("{}: {} - {}", i, start, end);
            let price = prices[i];
            if end <= price {
                end = price;
                if i == prices.len() - 1 {
                    if start < end {
                        ups.push(Up(start, end));
                    }
                } else {
                    continue;
                }
            } else {
                if start < end {
                    ups.push(Up(start, end));
                }
                start = price;
                end = price;
            }
        }

        println!(
            "{} [{}]",
            ups.len(),
            ups.iter()
                .map(|u| format!("{},{}", u.0, u.1))
                .collect::<Vec<String>>()
                .join(",")
        );

        let mut map: Vec<Vec<i32>> = Vec::new();
        map.push(ups.iter().map(|u| u.1 - u.0).collect());
        for n in 1..ups.len() {
            // 可以向后取 n 个
            let mut max: Vec<i32> = Vec::new();
            for i in 0..ups.len() {
                let up = &ups[i];
                let last = map[n - 1][i];
                let index = i + n;
                if index < ups.len() {
                    let up2 = &ups[index];
                    let current = up2.1 - up.0;
                    max.push(if last < current { current } else { last });
                } else {
                    max.push(last);
                }
            }
            map.push(max)
        }

        // 下面开始选取 k 个
        fn get_max(
            map: &Vec<Vec<i32>>,
            end: usize,
            k: usize,
            cache: &mut HashMap<(usize, usize), i32>,
        ) -> i32 {
            if k == 0 {
                return 0;
            }
            let key = (end, k);
            if let Some(max) = cache.get(&key) {
                return *max;
            }
            if end <= k {
                let mut sum = 0;
                for i in 0..end {
                    sum += map[0][i];
                }
                cache.insert(key, sum);
                return sum;
            }
            let mut max = 0;
            for nn in (0..end).rev() {
                // 进行拆分
                let right = map[end - nn - 1][nn];
                let left = get_max(map, nn, k - 1, cache);
                let m = left + right;
                if max < m {
                    max = m;
                }
            }
            cache.insert(key, max);
            max
        }

        let mut cache: HashMap<_, _> = HashMap::new();
        get_max(&map, ups.len(), k, &mut cache)
    }
}

fn main() {
    println!("{}", 188);

    let start = std::time::Instant::now();

    assert_eq!(2, Solution::max_profit(2, vec![2, 4, 1]));
    assert_eq!(7, Solution::max_profit(2, vec![3, 2, 6, 5, 0, 3]));
    assert_eq!(
        8,
        Solution::max_profit(2, vec![0, 1, 2, 1, 5, 0, 3, 2, 1, 0])
    );
    assert_eq!(
        13,
        Solution::max_profit(2, vec![1, 2, 4, 2, 5, 7, 2, 4, 9, 0])
    );
    assert_eq!(5, Solution::max_profit(1, vec![6, 1, 6, 4, 3, 0, 2]));
    assert_eq!(
        469,
        Solution::max_profit(
            7,
            vec![
                48, 12, 60, 93, 97, 42, 25, 64, 17, 56, 85, 93, 9, 48, 52, 42, 58, 85, 81, 84, 69,
                36, 1, 54, 23, 15, 72, 15, 11, 94,
            ]
        )
    );
    assert_eq!(
        2818,
        Solution::max_profit(
            29,
            vec![
                70, 4, 83, 56, 94, 72, 78, 43, 2, 86, 65, 100, 94, 56, 41, 66, 3, 33, 10, 3, 45,
                94, 15, 12, 78, 60, 58, 0, 58, 15, 21, 7, 11, 41, 12, 96, 83, 77, 47, 62, 27, 19,
                40, 63, 30, 4, 77, 52, 17, 57, 21, 66, 63, 29, 51, 40, 37, 6, 44, 42, 92, 16, 64,
                33, 31, 51, 36, 0, 29, 95, 92, 35, 66, 91, 19, 21, 100, 95, 40, 61, 15, 83, 31, 55,
                59, 84, 21, 99, 45, 64, 90, 25, 40, 6, 41, 5, 25, 52, 59, 61, 51, 37, 92, 90, 20,
                20, 96, 66, 79, 28, 83, 60, 91, 30, 52, 55, 1, 99, 8, 68, 14, 84, 59, 5, 34, 93,
                25, 10, 93, 21, 35, 66, 88, 20, 97, 25, 63, 80, 20, 86, 33, 53, 43, 86, 53, 55, 61,
                77, 9, 2, 56, 78, 43, 19, 68, 69, 49, 1, 6, 5, 82, 46, 24, 33, 85, 24, 56, 51, 45,
                100, 94, 26, 15, 33, 35, 59, 25, 65, 32, 26, 93, 73, 0, 40, 92, 56, 76, 18, 2, 45,
                64, 66, 64, 39, 77, 1, 55, 90, 10, 27, 85, 40, 95, 78, 39, 40, 62, 30, 12, 57, 84,
                95, 86, 57, 41, 52, 77, 17, 9, 15, 33, 17, 68, 63, 59, 40, 5, 63, 30, 86, 57, 5,
                55, 47, 0, 92, 95, 100, 25, 79, 84, 93, 83, 93, 18, 20, 32, 63, 65, 56, 68, 7, 31,
                100, 88, 93, 11, 43, 20, 13, 54, 34, 29, 90, 50, 24, 13, 44, 89, 57, 65, 95, 58,
                32, 67, 38, 2, 41, 4, 63, 56, 88, 39, 57, 10, 1, 97, 98, 25, 45, 96, 35, 22, 0, 37,
                74, 98, 14, 37, 77, 54, 40, 17, 9, 28, 83, 13, 92, 3, 8, 60, 52, 64, 8, 87, 77, 96,
                70, 61, 3, 96, 83, 56, 5, 99, 81, 94, 3, 38, 91, 55, 83, 15, 30, 39, 54, 79, 55,
                86, 85, 32, 27, 20, 74, 91, 99, 100, 46, 69, 77, 34, 97, 0, 50, 51, 21, 12, 3, 84,
                84, 48, 69, 94, 28, 64, 36, 70, 34, 70, 11, 89, 58, 6, 90, 86, 4, 97, 63, 10, 37,
                48, 68, 30, 29, 53, 4, 91, 7, 56, 63, 22, 93, 69, 93, 1, 85, 11, 20, 41, 36, 66,
                67, 57, 76, 85, 37, 80, 99, 63, 23, 71, 11, 73, 41, 48, 54, 61, 49, 91, 97, 60, 38,
                99, 8, 17, 2, 5, 56, 3, 69, 90, 62, 75, 76, 55, 71, 83, 34, 2, 36, 56, 40, 15, 62,
                39, 78, 7, 37, 58, 22, 64, 59, 80, 16, 2, 34, 83, 43, 40, 39, 38, 35, 89, 72, 56,
                77, 78, 14, 45, 0, 57, 32, 82, 93, 96, 3, 51, 27, 36, 38, 1, 19, 66, 98, 93, 91,
                18, 95, 93, 39, 12, 40, 73, 100, 17, 72, 93, 25, 35, 45, 91, 78, 13, 97, 56, 40,
                69, 86, 69, 99, 4, 36, 36, 82, 35, 52, 12, 46, 74, 57, 65, 91, 51, 41, 42, 17, 78,
                49, 75, 9, 23, 65, 44, 47, 93, 84, 70, 19, 22, 57, 27, 84, 57, 85, 2, 61, 17, 90,
                34, 49, 74, 64, 46, 61, 0, 28, 57, 78, 75, 31, 27, 24, 10, 93, 34, 19, 75, 53, 17,
                26, 2, 41, 89, 79, 37, 14, 93, 55, 74, 11, 77, 60, 61, 2, 68, 0, 15, 12, 47, 12,
                48, 57, 73, 17, 18, 11, 83, 38, 5, 36, 53, 94, 40, 48, 81, 53, 32, 53, 12, 21, 90,
                100, 32, 29, 94, 92, 83, 80, 36, 73, 59, 61, 43, 100, 36, 71, 89, 9, 24, 56, 7, 48,
                34, 58, 0, 43, 34, 18, 1, 29, 97, 70, 92, 88, 0, 48, 51, 53, 0, 50, 21, 91, 23, 34,
                49, 19, 17, 9, 23, 43, 87, 72, 39, 17, 17, 97, 14, 29, 4, 10, 84, 10, 33, 100, 86,
                43, 20, 22, 58, 90, 70, 48, 23, 75, 4, 66, 97, 95, 1, 80, 24, 43, 97, 15, 38, 53,
                55, 86, 63, 40, 7, 26, 60, 95, 12, 98, 15, 95, 71, 86, 46, 33, 68, 32, 86, 89, 18,
                88, 97, 32, 42, 5, 57, 13, 1, 23, 34, 37, 13, 65, 13, 47, 55, 85, 37, 57, 14, 89,
                94, 57, 13, 6, 98, 47, 52, 51, 19, 99, 42, 1, 19, 74, 60, 8, 48, 28, 65, 6, 12, 57,
                49, 27, 95, 1, 2, 10, 25, 49, 68, 57, 32, 99, 24, 19, 25, 32, 89, 88, 73, 96, 57,
                14, 65, 34, 8, 82, 9, 94, 91, 19, 53, 61, 70, 54, 4, 66, 26, 8, 63, 62, 9, 20, 42,
                17, 52, 97, 51, 53, 19, 48, 76, 40, 80, 6, 1, 89, 52, 70, 38, 95, 62, 24, 88, 64,
                42, 61, 6, 50, 91, 87, 69, 13, 58, 43, 98, 19, 94, 65, 56, 72, 20, 72, 92, 85, 58,
                46, 67, 2, 23, 88, 58, 25, 88, 18, 92, 46, 15, 18, 37, 9, 90, 2, 38, 0, 16, 86, 44,
                69, 71, 70, 30, 38, 17, 69, 69, 80, 73, 79, 56, 17, 95, 12, 37, 43, 5, 5, 6, 42,
                16, 44, 22, 62, 37, 86, 8, 51, 73, 46, 44, 15, 98, 54, 22, 47, 28, 11, 75, 52, 49,
                38, 84, 55, 3, 69, 100, 54, 66, 6, 23, 98, 22, 99, 21, 74, 75, 33, 67, 8, 80, 90,
                23, 46, 93, 69, 85, 46, 87, 76, 93, 38, 77, 37, 72, 35, 3, 82, 11, 67, 46, 53, 29,
                60, 33, 12, 62, 23, 27, 72, 35, 63, 68, 14, 35, 27, 98, 94, 65, 3, 13, 48, 83, 27,
                84, 86, 49, 31, 63, 40, 12, 34, 79, 61, 47, 29, 33, 52, 100, 85, 38, 24, 1, 16, 62,
                89, 36, 74, 9, 49, 62, 89,
            ]
        )
    );

    let end = std::time::Instant::now();

    println!("spend: {:?}", end - start);
}
