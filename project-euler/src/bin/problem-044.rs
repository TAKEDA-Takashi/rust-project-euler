//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2044
//!
//! 五角数は Pn = n(3n-1)/2 で生成される. 最初の10項は
//!
//! 1, 5, 12, 22, 35, 51, 70, 92, 117, 145, ...
//! である.
//!
//! P4 + P7 = 22 + 70 = 92 = P8 である. しかし差 70 - 22 = 48 は五角数ではない.
//!
//! 五角数のペア Pj と Pk について, 差と和が五角数になるものを考える. 差を D = |Pk - Pj| と書く. 差 D の最小値を求めよ.

use std::collections::HashSet;

fn main() {
    let mut pentagonal_set: HashSet<usize> = HashSet::new();

    for n in 2.. {
        let p = n * (3 * n - 1) / 2;

        if let Some(t) = pentagonal_set
            .iter()
            .find(|&t| pentagonal_set.contains(&(p - t)) && is_pentagonal(p + t))
        {
            println!("{}", p - t);
            break;
        }

        pentagonal_set.insert(p);
    }
}

fn is_pentagonal(p: usize) -> bool {
    let n = ((24 * p + 1) as f64).sqrt();
    // 平方根が整数
    if n.floor() != n {
        return false;
    }

    (n as isize + 1) % 6 == 0
}
