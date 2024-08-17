struct Solution;

// impl Solution {
//     pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
//         use std::collections::HashSet;

//         let s = s.as_bytes();

//         let mut set: HashSet<&[u8]> = HashSet::new();
//         // println!("{:?}", set);
//         let mut rs: HashSet<&[u8]> = HashSet::new();

//         let mut i = 0;
//         while i + 10 <= s.len() {
//             let ss = &s[i..i+10];
//             // println!("{:?} {}", set, i);
//             // println!("{:?}",  set.contains(ss));
//             if set.contains(ss) {
//                 rs.insert(ss);
//                 // println!("{:?}", ss);
//             } else {
//                 set.insert(ss);
//             }
//             i += 1;
//         }

//         use std::borrow::Cow;

//         rs.into_iter()
//         .map(|a| String::from_utf8_lossy(a))
//         .map(|a| match a {
//             Cow::Borrowed(v) => v.to_string(),
//             Cow::Owned(v) => v,
//         })
//         .collect::<Vec<String>>()
//     }
// }

impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        use std::collections::HashMap;

        let s = s.as_bytes();

        let mut map: HashMap<&[u8], usize> = HashMap::new();

        let mut i = 0;
        while i + 10 <= s.len() {
            let ss = &s[i..i + 10];
            // println!("{:?} {}", set, i);
            // println!("{:?}",  set.contains(ss));
            let v = map.entry(ss).or_insert(0);
            *v += 1;
            i += 1;
        }

        use std::borrow::Cow;

        map.into_iter()
            .filter(|a| a.1 > 1)
            .map(|a| String::from_utf8_lossy(a.0))
            .map(|a| match a {
                Cow::Borrowed(v) => v.to_string(),
                Cow::Owned(v) => v,
            })
            .collect::<Vec<String>>()
    }
}

fn main() {
    println!("{}", 187);

    // let hp = Solution::largest_number([1, 2, 9, 9, 91].to_vec());
    // let hp = Solution::largest_number([1, 2].to_vec());
    // let r = Solution::find_repeated_dna_sequences(String::from("AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT"));
    let r = Solution::find_repeated_dna_sequences(String::from("AAAAAAAAAAAAA"));

    println!("{:#?}", r);
}
