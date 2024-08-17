#![allow(dead_code)]
struct Solution;

impl Solution {
    // pub fn is_isomorphic(s: String, t: String) -> bool {
    //     let ss = s.chars().collect::<Vec<_>>();
    //     let tt = t.chars().collect::<Vec<_>>();
    //     if ss.len() != tt.len() {
    //         return false;
    //     }

    //     let mut map1 = std::collections::HashMap::new();
    //     let mut map2 = std::collections::HashMap::new();
    //     for i in 0..ss.len() {
    //         let s = ss[i];
    //         let t = tt[i];
    //         match (map1.get(&s), map2.get(&t)) {
    //             (None, None) => {
    //                 map1.insert(s, t);
    //                 map2.insert(t, s);
    //             }
    //             (Some(v1), Some(v2)) => {
    //                 if *v1 != t || *v2 != s {
    //                     return false;
    //                 }
    //             }
    //             _ => {
    //                 return false;
    //             }
    //         }
    //     }

    //     return true;
    // }
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let ss = s.as_bytes();
        let tt = t.as_bytes();
        if ss.len() != tt.len() {
            return false;
        }

        let mut map1 = [128; 128];
        let mut map2 = [128; 128];
        for i in 0..ss.len() {
            let s = ss[i] as usize;
            let t = tt[i] as usize;
            match (map1[s], map2[t]) {
                (128, 128) => {
                    map1[s] = t;
                    map2[t] = s;
                }
                (v1, v2) => {
                    if v1 != t || v2 != s {
                        return false;
                    }
                }
            }
        }

        return true;
    }
}

fn main() {
    println!("{}", 205);

    let start = std::time::Instant::now();

    assert_eq!(
        Solution::is_isomorphic("egg".to_string(), "add".to_string()),
        true
    );
    assert_eq!(
        Solution::is_isomorphic("foo".to_string(), "bar".to_string()),
        false
    );
    assert_eq!(
        Solution::is_isomorphic("badc".to_string(), "baba".to_string()),
        false
    );

    let end = std::time::Instant::now();

    println!("spend: {:?}", end - start);
}
