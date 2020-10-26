//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2039
//!
//! 辺の長さが {a,b,c} と整数の3つ組である直角三角形を考え, その周囲の長さを p とする. p = 120のときには3つの解が存在する:
//!
//! {20,48,52}, {24,45,51}, {30,40,50}
//!
//! p ≤ 1000 のとき解の数が最大になる p はいくつか?

use euler_lib::primitive_pythagorean_triple;
use itertools::Itertools;
use std::iter::repeat;

fn main() {
    let v = (2..23) // mは23が上界; m(m+1) <= 500
        .flat_map(|m| repeat(m).zip(1..m))
        .filter_map(|(m, n)| primitive_pythagorean_triple(m, n))
        // 原始ピタゴラス数と相似な直角三角形を列挙
        .flat_map(|(a, b, c)| {
            (1..)
                .zip(repeat(a + b + c))
                .map(|(n, p)| n * p)
                .take_while(|&p| p <= 1000)
        })
        .sorted()
        .collect_vec();

    println!(
        "{}",
        v.iter()
            .group_by(|&n| n)
            .into_iter()
            .map(|(k, g)| (k, g.count()))
            .max_by_key(|(_, c)| *c)
            .unwrap()
            .0
    )
}
