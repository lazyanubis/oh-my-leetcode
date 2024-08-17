#![allow(dead_code)]
struct WordDictionary {
    end: bool,
    children: Vec<Option<WordDictionary>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {
    fn new() -> Self {
        let mut children = Vec::with_capacity(26);
        for _ in 0..26 {
            children.push(None);
        }
        Self {
            end: false,
            children,
        }
    }

    fn add_word(&mut self, word: String) {
        self.inner_add_word(word.as_bytes());
    }
    fn inner_add_word(&mut self, word: &[u8]) {
        if word.is_empty() {
            self.end = true;
        } else {
            let index = (word[0] - b'a') as usize;
            if self.children[index].is_none() {
                self.children[index] = Some(WordDictionary::new());
            }
            if let Some(inner) = self.children[index].as_mut() {
                inner.inner_add_word(&word[1..]);
            }
        }
    }

    fn search(&self, word: String) -> bool {
        self.inner_search(word.as_bytes())
    }
    fn inner_search(&self, word: &[u8]) -> bool {
        if word.is_empty() {
            return self.end;
        }
        if word[0] == b'.' {
            for i in 0..26 {
                if self.children[i]
                    .as_ref()
                    .is_some_and(|inner| inner.inner_search(&word[1..]))
                {
                    return true;
                }
            }
            false
        } else {
            self.children[(word[0] - b'a') as usize]
                .as_ref()
                .is_some_and(|inner| inner.inner_search(&word[1..]))
        }
    }
}

fn main() {
    println!("No. {}", 211);

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
