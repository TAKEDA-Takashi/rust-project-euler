//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2032
//!
//! すべての桁に 1 から n が一度だけ使われている数をn桁の数がパンデジタル (pandigital) であるということにしよう: 例えば5桁の数 15234 は1から5のパンデジタルである.
//!
//! 7254 は面白い性質を持っている. 39 × 186 = 7254 と書け, 掛けられる数, 掛ける数, 積が1から9のパンデジタルとなる.
//!
//! 掛けられる数/掛ける数/積が1から9のパンデジタルとなるような積の総和を求めよ.
//!
//! HINT: いくつかの積は, 1通り以上の掛けられる数/掛ける数/積の組み合わせを持つが1回だけ数え上げよ.

// 条件を整理すると、(a, b, c)の3つ組（a < b）は (1桁, 4桁, 4桁)か(2桁, 3桁, 4桁)しかありえない。

use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    println!(
        "{}",
        (1..=9)
            .permutations(9)
            .filter_map(|p| {
                let a = p[0];
                let b = join_to_usize(&p[1..=4]);
                let c = join_to_usize(&p[5..=8]);

                if a * b == c {
                    return Some(c);
                }

                let a = join_to_usize(&p[0..=1]);
                let b = join_to_usize(&p[2..=4]);
                let c = join_to_usize(&p[5..=8]);

                if a * b == c {
                    return Some(c);
                }

                None
            })
            .collect::<HashSet<_>>() // 重複削除
            .iter()
            .sum::<usize>()
    );
}

fn join_to_usize(v: &[usize]) -> usize {
    v.iter().join("").parse().unwrap()
}
