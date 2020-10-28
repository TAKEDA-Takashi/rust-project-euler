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

use euler_lib::Prime;
use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    let mut prime = Prime::new();

    let w = (120..)
        .tuple_windows()
        .find(|(a, b, c, d)| {
            prime.factorization(a).iter().collect::<HashSet<_>>().len() == 4
                && prime.factorization(b).iter().collect::<HashSet<_>>().len() == 4
                && prime.factorization(c).iter().collect::<HashSet<_>>().len() == 4
                && prime.factorization(d).iter().collect::<HashSet<_>>().len() == 4
        })
        .unwrap();

    println!("{}", w.0);
}
