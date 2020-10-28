//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2077
//!
//! 10は素数の和として5通りに表すことができる:
//!
//! 7 + 3
//! 5 + 5
//! 5 + 3 + 2
//! 3 + 3 + 2 + 2
//! 2 + 2 + 2 + 2 + 2
//!
//! 素数の和としての表し方が5000通り以上になる最初の数を求めよ.

use euler_lib::Prime;
use std::collections::HashSet;

fn main() {
    let primes: Vec<_> = Prime::new().iter().take_while(|&p| p < 100).collect();
    let prime_set: HashSet<_> = primes.iter().copied().collect();

    let (n, _) = (4..)
        .map(|n| (n, counting_prime_summations(&n, &primes, &prime_set)))
        .filter(|&(_, cnt)| cnt >= 5000)
        .next()
        .unwrap();

    println!("{}", n);
}

fn counting_prime_summations(n: &usize, primes: &Vec<usize>, prime_set: &HashSet<usize>) -> usize {
    fn counting_prime_summations0(
        n: usize,
        idx: usize,
        primes: &Vec<usize>,
        prime_set: &HashSet<usize>,
    ) -> usize {
        if n < 4 {
            0
        } else if idx == 0 {
            if n % 2 == 1 {
                0
            } else {
                1
            }
        } else {
            if n < primes[idx] {
                counting_prime_summations0(n, idx - 1, primes, prime_set)
            } else {
                let m = n - primes[idx];
                counting_prime_summations0(n, idx - 1, primes, prime_set)
                    + counting_prime_summations0(m, idx, primes, prime_set)
                    + if prime_set.contains(&m) && m <= primes[idx] {
                        1
                    } else {
                        0
                    }
            }
        }
    }

    counting_prime_summations0(
        *n,
        primes.binary_search(&n).unwrap_or_else(|i| i - 1),
        primes,
        prime_set,
    )
}
