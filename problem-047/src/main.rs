//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2047
//!
//! それぞれ2つの異なる素因数を持つ連続する2つの数が最初に現れるのは:
//!
//! 14 = 2 × 7
//! 15 = 3 × 5
//!
//! それぞれ3つの異なる素因数を持つ連続する3つの数が最初に現れるのは:
//!
//! 644 = 22 × 7 × 23
//! 645 = 3 × 5 × 43
//! 646 = 2 × 17 × 19
//!
//! 最初に現れるそれぞれ4つの異なる素因数を持つ連続する4つの数を求めよ. その最初の数はいくつか?

use itertools::Itertools;
use problem_047::PrimeFactorization;
use std::collections::HashSet;

fn main() {
    let mut prime_factorization = PrimeFactorization::new();

    let w = (120..)
        .tuple_windows()
        .find(|&(a, b, c, d)| {
            prime_factorization
                .exec(a)
                .iter()
                .collect::<HashSet<_>>()
                .len()
                == 4
                && prime_factorization
                    .exec(b)
                    .iter()
                    .collect::<HashSet<_>>()
                    .len()
                    == 4
                && prime_factorization
                    .exec(c)
                    .iter()
                    .collect::<HashSet<_>>()
                    .len()
                    == 4
                && prime_factorization
                    .exec(d)
                    .iter()
                    .collect::<HashSet<_>>()
                    .len()
                    == 4
        })
        .unwrap();

    println!("{}", w.0);
}
