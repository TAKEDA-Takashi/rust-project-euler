//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%20127
//!
//! n の根基 (radical) を rad(n) と書き, n の異なる素因数の積とする. 例えば, 504 = 23 × 32 × 7 なので rad(504) = 2 × 3 × 7 = 42 である.
//!
//! 正整数の3つ組 (a, b, c) が abc-hit であるとは
//!
//! GCD(a, b) = GCD(b, c) = GCD(c, a) = 1
//! a < b
//! a + b = c
//! rad(abc) < c
//! の4つの性質を満たすことである.
//!
//! (5, 27, 32) は abc-hit である:
//!
//! GCD(5, 27) = GCD(5, 32) = GCD(27, 32) = 1
//! 5 < 27
//! 5 + 27 = 32
//! rad(4320) = 30 < 32
//! abc-hit は非常に稀である. c < 1000 のときには31個しかなく, そのときの ∑c は 12523 である.
//!
//! c < 120000 での ∑c を求めよ.

use euler_lib::Prime;
use itertools::Itertools;
use num::Integer;
use std::collections::{HashMap, HashSet};

fn main() {
    // let ubound = 1000;
    let ubound = 120_000;
    let prime = Prime::<usize>::new();
    let prime_set: HashSet<_> = prime.iter().take_while(|&p| p < ubound).collect();

    let mut rad_memo = HashMap::new();

    let mut sum = 0;

    for c in 9..ubound {
        if prime_set.contains(&c) {
            continue;
        }

        let rad_c = *rad_memo
            .entry(c)
            .or_insert_with(|| prime.factorization(&c).iter().dedup().product::<usize>());

        if c < rad_c * 2 {
            continue;
        }

        for a in 1..=c / 2 {
            if a.gcd(&c) != 1 {
                continue;
            }

            let b = c - a;

            if *rad_memo
                .entry(a)
                .or_insert_with(|| prime.factorization(&a).iter().dedup().product::<usize>())
                * *rad_memo
                    .entry(b)
                    .or_insert_with(|| prime.factorization(&b).iter().dedup().product::<usize>())
                * rad_c
                < c
            {
                // println!("{} {} {}", a, b, c);
                sum += c;
            }
        }
    }

    println!("{}", sum);
}
