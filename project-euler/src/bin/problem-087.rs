//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2087
//!
//! 素数の2乗と素数の3乗と素数の4乗の和で表される最小の数は28である. 50未満のこのような数は丁度4つある.
//!
//! 28 = 22 + 23 + 24
//! 33 = 32 + 23 + 24
//! 49 = 52 + 23 + 24
//! 47 = 22 + 33 + 24
//!
//! では, 50,000,000未満の数で, 素数の2乗と素数の3乗と素数の4乗の和で表される数は何個あるか?

use euler_lib::Prime;
use itertools::{iproduct, Itertools};
use std::collections::HashSet;

fn main() {
    let a_iter = Prime::<usize>::new()
        .iter()
        .take_while(|&p| p < 7072) // 50000000の平方根
        .collect_vec();
    let b_iter = a_iter
        .iter()
        .take_while(|&&p| p < 370) // 3乗根
        .copied()
        .collect_vec();
    let c_iter = a_iter
        .iter()
        .take_while(|&&p| p < 85) // 4乗根
        .copied()
        .collect_vec();

    let set = iproduct!(a_iter, b_iter, c_iter)
        .map(|(a, b, c)| a.pow(2) + b.pow(3) + c.pow(4))
        .filter(|n| *n < 50_000_000)
        .collect::<HashSet<_>>();

    println!("{:?}", set.len())
}
