//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2062
//!
//! 立方数 41063625 (3453) は, 桁の順番を入れ替えると2つの立方数になる: 56623104 (3843) と 66430125 (4053) である. 41063625は, 立方数になるような桁の置換をちょうど3つもつ最小の立方数である.
//!
//! 立方数になるような桁の置換をちょうど5つもつ最小の立方数を求めよ.

use itertools::Itertools;
use std::cmp::Ord;

fn main() {
    let target_num = 5;

    println!(
        "{}",
        (1..)
            .filter_map(|exp| {
                (1..)
                    .map(|n| n * n * n)
                    .skip_while(|&c| c < 10_u64.pow(exp))
                    .take_while(|&c| c < 10_u64.pow(exp + 1))
                    .map(|c| (c.to_string().chars().sorted().collect::<String>(), c))
                    .sorted_by(|a, b| Ord::cmp(&a.0, &b.0))
                    .group_by(|(s, _)| s.clone())
                    .into_iter()
                    .map(|(_, group)| group.map(|(_, n)| n).collect_vec())
                    .filter(|ns| ns.len() == target_num)
                    .flatten()
                    .min()
            })
            .next()
            .unwrap()
    );
}
