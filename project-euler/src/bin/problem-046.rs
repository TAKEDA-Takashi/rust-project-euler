//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2046
//!
//! Christian Goldbachは全ての奇合成数は平方数の2倍と素数の和で表せると予想した.
//!
//! 9 = 7 + 2×12
//! 15 = 7 + 2×22
//! 21 = 3 + 2×32
//! 25 = 7 + 2×32
//! 27 = 19 + 2×22
//! 33 = 31 + 2×12
//!
//! 後に, この予想は誤りであることが分かった.
//!
//! 平方数の2倍と素数の和で表せない最小の奇合成数はいくつか?

use euler_lib::Prime;
use std::collections::HashSet;

fn main() {
    let mut odd_composite_set: HashSet<usize> = HashSet::new();
    let mut odd_iter = (3..).step_by(2);

    let (o, ..) = Prime::new()
        .iter()
        .skip(1)
        .filter_map(|p| {
            odd_composite_set.extend((1..p).map(|n| p + 2 * n * n));
            odd_iter
                .by_ref()
                .map(|o| (o, p, odd_composite_set.contains(&o)))
                .skip_while(|&(o, _, contain)| o < p && contain)
                .next()
        })
        .skip_while(|&(o, p, contain)| o == p || contain)
        .next()
        .unwrap();

    println!("{}", o);
}
