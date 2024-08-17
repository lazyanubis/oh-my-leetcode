#![allow(dead_code)]
struct Solution;

impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        if n <= 2 {
            return 0;
        }
        if n == 3 {
            return 1;
        }

        #[inline(always)]
        fn is_primes(n: i32, primes: &Vec<i32>) -> bool {
            let max = (n as f64).sqrt() as i32;
            for i in primes {
                if *i > max {
                    break;
                }
                if n % i == 0 {
                    return false;
                }
            }
            true
        }

        let mut count = 1;
        let mut primes = vec![2];
        for i in 3..n {
            if is_primes(i, &primes) {
                primes.push(i);
                count += 1;
            }
        }
        count
    }
}

fn main() {
    println!("{}", 204);

    let start = std::time::Instant::now();

    assert_eq!(Solution::count_primes(10), 4);
    assert_eq!(Solution::count_primes(0), 0);
    assert_eq!(Solution::count_primes(0), 0);
    assert_eq!(Solution::count_primes(999983), 78497);

    let end = std::time::Instant::now();

    println!("spend: {:?}", end - start);
}
