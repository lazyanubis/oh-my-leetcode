#![allow(dead_code)]
struct Solution;

// impl Solution {
//     pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
//         let mut board = board;
//         let mut ans = Vec::new();

//         #[inline]
//         fn inner_find_word(board: &mut Vec<Vec<char>>, i: usize, j: usize, word: &[char]) -> bool {
//             let tmp = board[i][j];
//             let first = word[0];
//             if tmp != first {
//                 return false;
//             }

//             let word = &word[1..];
//             if word.is_empty() {
//                 return true;
//             }

//             board[i][j] = ' ';

//             let mut r = false;
//             if (0 < j && inner_find_word(board, i, j - 1, word))
//                 || (j < board[0].len() - 1 && inner_find_word(board, i, j + 1, word))
//                 || (0 < i && inner_find_word(board, i - 1, j, word))
//                 || (i < board.len() - 1 && inner_find_word(board, i + 1, j, word))
//             {
//                 r = true;
//             }

//             board[i][j] = first;

//             r
//         }

//         let m = board.len();
//         let n = board[0].len();

//         for word in &words {
//             let _word = word.chars().collect::<Vec<_>>();
//             'a: for i in 0..m {
//                 for j in 0..n {
//                     if inner_find_word(&mut board, i, j, &_word) {
//                         ans.push(word.clone());
//                         break 'a;
//                     }
//                 }
//             }
//         }

//         ans
//     }
// }

impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        #[derive(Debug)]
        struct TrieNode {
            children: [Option<Box<TrieNode>>; 26],
            is_end: bool,
            is_children_found: bool,
        }
        impl TrieNode {
            fn new() -> Self {
                TrieNode {
                    children: Default::default(),
                    is_end: false,
                    is_children_found: false,
                }
            }
            fn insert(&mut self, word: &str) {
                if word.is_empty() {
                    self.is_end = true;
                    return;
                }
                let idx = (word.as_bytes()[0] - b'a') as usize;
                let child = self.children[idx].get_or_insert_with(|| Box::new(TrieNode::new()));
                child.insert(&word[1..]);
            }
        }
        let mut trie = TrieNode::new();
        for word in &words {
            trie.insert(word);
        }

        // println!("{:#?}", trie);

        let mut board = board;

        #[inline]
        fn inner_find_word(
            board: &mut Vec<Vec<char>>,
            i: usize,
            j: usize,
            node: &mut TrieNode,
        ) -> Vec<Vec<char>> {
            if node.is_children_found {
                return vec![];
            }

            let tmp = board[i][j];

            let mut ans = Vec::new();

            if node.is_end {
                ans.push(vec![]);
            }

            for ii in 0..26 {
                if let Some(child) = &mut node.children[ii] {
                    if tmp as usize == ii + 'a' as usize {
                        board[i][j] = ' ';
                        fn push(mut rs: Vec<Vec<char>>, tmp: char, ans: &mut Vec<Vec<char>>) {
                            for r in &mut rs {
                                r.push(tmp);
                            }
                            ans.extend(rs);
                        }
                        if child.is_end {
                            push(vec![vec![]], tmp, &mut ans);
                        }
                        if 0 < j {
                            push(inner_find_word(board, i, j - 1, child), tmp, &mut ans);
                        }
                        if j < board[0].len() - 1 {
                            push(inner_find_word(board, i, j + 1, child), tmp, &mut ans);
                        }
                        if 0 < i {
                            push(inner_find_word(board, i - 1, j, child), tmp, &mut ans);
                        }
                        if i < board.len() - 1 {
                            push(inner_find_word(board, i + 1, j, child), tmp, &mut ans);
                        }
                        board[i][j] = tmp;
                        break;
                    }
                }
            }

            let mut found = true;
            for ii in 0..26 {
                if node.children[ii]
                    .as_ref()
                    .is_some_and(|s| !s.is_children_found)
                {
                    found = false;
                    break;
                }
            }
            if found {
                node.is_children_found = true;
            }

            ans
        }

        let m = board.len();
        let n = board[0].len();

        let mut ans = std::collections::HashSet::new();

        for i in 0..m {
            for j in 0..n {
                let rs = inner_find_word(&mut board, i, j, &mut trie);
                for mut word in rs {
                    word.reverse();
                    let mut s = String::new();
                    for w in word {
                        s.push(w);
                    }
                    ans.insert(s);
                }
            }
        }

        ans.into_iter().collect::<Vec<_>>()
    }
}

fn main() {
    println!("No. {}", 212);

    let start = std::time::Instant::now();

    // assert_eq!(Solution::find_order(2, vec![vec![1, 0]]), vec![0, 1]);
    assert_eq!(
        Solution::find_words(vec![vec!['a'],], vec!['a'.into(),]),
        vec!['a'.to_string(),]
    );
    assert_eq!(
        Solution::find_words(
            vec![
                vec!['o', 'a', 'a', 'n'],
                vec!['e', 't', 'a', 'e'],
                vec!['i', 'h', 'k', 'r'],
                vec!['i', 'f', 'l', 'v']
            ],
            vec!["oath".into(), "pea".into(), "eat".into(), "rain".into()]
        ),
        vec!["eat".to_string(), "oath".to_string()]
    );
    assert_eq!(
        Solution::find_words(
            vec![
                vec!['a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'],
                vec!['a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'],
                vec!['a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'],
                vec!['a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'],
                vec!['a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'],
                vec!['a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'],
                vec!['a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'],
                vec!['a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'],
                vec!['a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'],
                vec!['a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'],
                vec!['a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'],
                vec!['a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a']
            ],
            vec![
                "a".into(),
                "aa".into(),
                "aaa".into(),
                "aaaa".into(),
                "aaaaa".into(),
                "aaaaaa".into(),
                "aaaaaaa".into(),
                "aaaaaaaa".into(),
                "aaaaaaaaa".into(),
                "aaaaaaaaaa".into()
            ]
        ),
        vec!["eat".to_string(), "oath".to_string()]
    );
    // assert_eq!(
    //     Solution::find_order(4, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]]),
    //     vec![0, 1, 2, 3]
    // );

    let end = std::time::Instant::now();

    println!("spend: {:?}", end - start);
}
