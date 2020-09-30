//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2037
//!
//! 3797は面白い性質を持っている. まずそれ自身が素数であり, 左から右に桁を除いたときに全て素数になっている (3797, 797, 97, 7). 同様に右から左に桁を除いたときも全て素数である (3797, 379, 37, 3).
//!
//! 右から切り詰めても左から切り詰めても素数になるような素数は11個しかない. 総和を求めよ.
//!
//! 注: 2, 3, 5, 7を切り詰め可能な素数とは考えない.

use problem_037::Prime;
use std::collections::BTreeSet;

fn main() {
    let mut prime_iter = Prime::new();
    let mut prime_set: BTreeSet<usize> = prime_iter.by_ref().take(4).collect();
    let mut v = vec![];

    for p in prime_iter {
        prime_set.insert(p);

        if is_prime_left_pad(p, &prime_set) && is_prime_right_pad(p, &prime_set) {
            v.push(p);

            // 11個見つける
            if v.len() == 11 {
                break;
            }
        }
    }

    println!("{}", v.iter().sum::<usize>());
}

fn is_prime_left_pad(n: usize, prime_set: &BTreeSet<usize>) -> bool {
    match n % 10 {
        1 | 9 => false,
        _ => {
            let mut s = &n.to_string()[1..];

            while s.len() > 1 {
                if !prime_set.contains(&s.parse().unwrap()) {
                    return false;
                }

                s = &s[1..];
            }

            true
        }
    }
}

fn is_prime_right_pad(n: usize, prime_set: &BTreeSet<usize>) -> bool {
    match &n.to_string()[..1] {
        "1" | "4" | "6" | "8" | "9" => false,
        _ => {
            let mut n = n / 10;

            while n > 1 {
                if !prime_set.contains(&n) {
                    return false;
                }

                n /= 10;
            }

            true
        }
    }
}
