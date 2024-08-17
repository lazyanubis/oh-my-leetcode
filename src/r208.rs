#![allow(dead_code)]

#[derive(Debug)]
struct Trie {
    length: Option<usize>,
    end: bool,
    branch: std::collections::HashMap<String, Trie>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    fn new() -> Self {
        Self {
            length: None,
            end: false,
            branch: Default::default(),
        }
    }

    fn insert(&mut self, word: String) {
        self.inner_insert(&word);
    }
    #[inline]
    fn inner_insert(&mut self, word: &str) {
        let len = word.len();
        if len == 0 {
            self.end = true;
            return;
        }

        match self.length {
            Some(length) => {
                if length <= len {
                    let key = &word[..length];
                    match self.branch.get_mut(key) {
                        Some(node) => node.inner_insert(&word[length..]),
                        None => {
                            let mut node = Trie::new();
                            node.inner_insert(&word[length..]);
                            self.branch.insert(key.to_string(), node);
                        }
                    }
                } else {
                    let branch = std::mem::replace(&mut self.branch, Default::default());
                    for (key, node) in branch {
                        let key1 = &key[..len];
                        let key2 = &key[len..];
                        match self.branch.get_mut(key1) {
                            Some(_node) => {
                                _node.branch.insert(key2.into(), node);
                            }
                            None => {
                                let mut node2 = Trie::new();
                                // node2.end = true;
                                node2.length = Some(length - len);
                                node2.branch.insert(key2.into(), node);
                                self.branch.insert(key1.to_string(), node2);
                            }
                        }
                    }
                    self.length = Some(len);

                    self.inner_insert(word);
                }
            }
            None => {
                self.length = Some(len);
                let mut node = Trie::new();
                node.end = true;
                self.branch.insert(word.into(), node);
            }
        }
    }

    fn search(&self, word: String) -> bool {
        self.inner_search(&word)
    }
    #[inline]
    fn inner_search(&self, word: &str) -> bool {
        let len = word.len();
        if len == 0 {
            return self.end;
        }

        match self.length {
            Some(length) => {
                if length <= len {
                    let key = &word[..length];
                    self.branch
                        .get(key)
                        .is_some_and(|node| node.inner_search(&word[length..]))
                } else {
                    return false;
                }
            }
            None => return false,
        }
    }

    fn starts_with(&self, prefix: String) -> bool {
        self.inner_starts_with(&prefix)
    }
    #[inline]
    fn inner_starts_with(&self, prefix: &str) -> bool {
        let len = prefix.len();
        if len == 0 {
            return self.end || self.length.is_some();
        }

        match self.length {
            Some(length) => {
                if length <= len {
                    let key = &prefix[..length];
                    self.branch
                        .get(key)
                        .is_some_and(|node| node.inner_starts_with(&prefix[length..]))
                } else {
                    for (key, _) in &self.branch {
                        if key.starts_with(prefix) {
                            return true;
                        }
                    }
                    return false;
                }
            }
            None => return false,
        }
    }
}

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */

fn main() {
    println!("{}", 208);

    let start = std::time::Instant::now();

    let mut obj = Trie::new();
    obj.insert("123".into());
    println!("{:?}", obj);
    assert_eq!(obj.search("123".into()), true);
    assert_eq!(obj.starts_with("123".into()), true);
    assert_eq!(obj.starts_with("1234".into()), false);

    let mut trie = Trie::new();
    trie.insert("apple".into());
    assert_eq!(trie.search("apple".into()), true); // 返回 True
    assert_eq!(trie.search("app".into()), false); // 返回 False
    assert_eq!(trie.starts_with("app".into()), true); // 返回 True
    trie.insert("app".into());
    assert_eq!(trie.search("app".into()), true); // 返回 True

    // let mut trie = Trie::new();
    // trie.insert("aa".into());
    // trie.insert("ab".into());
    // trie.insert("a".into());
    // trie.insert("az".into());
    // println!("{:#?}", trie);

    // let op = [
    //     "Trie",
    //     "insert",
    //     "insert",
    //     "insert",
    //     "insert",
    //     "insert",
    //     "insert",
    //     "insert",
    //     "insert",
    //     "insert",
    //     "insert",
    //     "insert",
    //     "insert",
    //     "insert",
    //     "insert",
    //     "insert",
    //     "insert",
    //     "insert",
    //     "insert",
    //     "insert",
    //     "insert",
    //     "insert",
    //     "insert",
    //     "insert",
    //     "insert",
    //     "insert",
    //     "insert",
    //     "insert",
    //     "insert",
    //     "insert",
    //     "insert",
    //     "insert",
    //     "insert",
    //     "insert",
    //     "insert",
    //     "insert",
    //     "insert",
    //     "insert",
    //     "insert",
    //     "insert",
    //     "insert",
    //     "insert",
    //     "insert",
    //     "insert",
    //     "insert",
    //     "insert",
    //     "insert",
    //     "insert",
    //     "insert",
    //     "insert",
    //     "insert",
    //     "insert",
    //     "insert",
    //     "search",
    //     "search",
    //     "search",
    //     "search",
    //     "startsWith",
    //     "startsWith",
    //     "startsWith",
    //     "startsWith",
    // ];

    // let values = [
    //     vec![],
    //     vec!["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"],
    //     vec!["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab"],
    //     vec!["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaac"],
    //     vec!["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaad"],
    //     vec!["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaae"],
    //     vec!["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaf"],
    //     vec!["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaag"],
    //     vec!["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaah"],
    //     vec!["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaai"],
    //     vec!["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaj"],
    //     vec!["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaak"],
    //     vec!["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaal"],
    //     vec!["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaam"],
    //     vec!["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaan"],
    //     vec!["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaao"],
    //     vec!["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaap"],
    //     vec!["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaq"],
    //     vec!["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaar"],
    //     vec!["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaas"],
    //     vec!["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaat"],
    //     vec!["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaau"],
    //     vec!["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaav"],
    //     vec!["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaw"],
    //     vec!["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaax"],
    //     vec!["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaay"],
    //     vec!["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaz"],
    //     vec!["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"],
    //     vec!["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab"],
    //     vec!["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaac"],
    //     vec!["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaad"],
    //     vec!["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaae"],
    //     vec!["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaf"],
    //     vec!["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaag"],
    //     vec!["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaah"],
    //     vec!["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaai"],
    //     vec!["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaj"],
    //     vec!["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaak"],
    //     vec!["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaal"],
    //     vec!["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaam"],
    //     vec!["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaan"],
    //     vec!["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaao"],
    //     vec!["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaap"],
    //     vec!["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaq"],
    //     vec!["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaar"],
    //     vec!["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaas"],
    //     vec!["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaat"],
    //     vec!["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaau"],
    //     vec!["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaav"],
    //     vec!["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaw"],
    //     vec!["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaax"],
    //     vec!["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaay"],
    //     vec!["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaz"],
    //     vec!["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"],
    //     vec!["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"],
    //     vec!["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"],
    //     vec!["a"],
    //     vec!["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"],
    //     vec!["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"],
    //     vec!["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"],
    //     vec!["a"],
    // ];
    // let results = [
    //     false, false, false, false, false, false, false, false, false, false, false, false, false,
    //     false, false, false, false, false, false, false, false, false, false, false, false, false,
    //     false, false, false, false, false, false, false, false, false, false, false, false, false,
    //     false, false, false, false, false, false, false, false, false, false, false, false, false,
    //     false, true, false, true, false, true, false, true, true,
    // ];
    // let mut trie = Trie::new();
    // for i in 1..op.len() {
    //     match op[i] {
    //         "Trie" => {
    //             trie = Trie::new();
    //         }
    //         "insert" => {
    //             trie.insert(values[i][0].into());
    //         }
    //         "search" => {
    //             println!("{i} {:#?}", trie);
    //             assert_eq!(trie.search(values[i][0].into()), results[i]);
    //         }
    //         "startsWith" => {
    //             assert_eq!(trie.starts_with(values[i][0].into()), results[i]);
    //         }
    //         _ => unreachable!(),
    //     }
    // }
 

    // assert_eq!(Solution::can_finish(2, vec![vec![1, 0]]), true);
    // assert_eq!(Solution::can_finish(2, vec![vec![1, 0], vec![0, 1]]), false);
    // assert_eq!(
    //     Solution::can_finish(3, vec![vec![1, 0], vec![1, 2], vec![0, 1]]),
    //     false
    // );

    let end = std::time::Instant::now();

    println!("spend: {:?}", end - start);
}
