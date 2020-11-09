//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%20119
//!
//! 512 という数は興味深い数である. というのも, 各桁の和を何乗かしたものに等しくなっているからである: 5 + 1 + 2 = 8, 83 = 512 である. この特性を持つ他の数は例えば 614656 = 284 である.
//!
//! この数列の第 n 項を an と定義し, また 2 桁以上であるとしよう.
//!
//! a2 = 512, a10 = 614656 となる.
//!
//! a30 を求めよ.

use itertools::iproduct;
use std::collections::BTreeSet;

fn main() {
    let set: BTreeSet<_> = iproduct!(2..=70_u128, 2..=10_u32)
        .map(|(b, p)| (b, p, b.pow(p)))
        .filter(|(b, _, n)| {
            *b == n
                .to_string()
                .chars()
                .filter_map(|c| c.to_digit(10))
                .sum::<u32>() as u128
        })
        .map(|(.., n)| n)
        .collect();

    // println!("{:?}", set);
    println!("{:?}", set.iter().nth(29)); // 0-origin
}
