//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%20125
//!
//! 回文数 595 は連続する平方数の和で表すことができるという面白い性質を持つ: 62+72+82+92+102+112+122
//!
//! 1000 未満で連続する平方数の和で表せる回文数はちょうど 11 あり, その合計は 4164 である. 正の整数の平方のみをこの問題では扱うため, 1=02+12 は含めないことに注意せよ.
//!
//! 回文数でありかつ連続する平方数の和で表せる, 108 未満のすべての数の合計を求めよ.

use euler_lib::is_palindrome;
use std::collections::HashSet;

fn main() {
    let ubound = 7072; // これ以上は8桁の数しか現れない
    let bound = 10_u128.pow(8);

    println!(
        "{}",
        (2..ubound)
            .flat_map(|n| {
                (0..=n - 2)
                    .map(move |m| square_sum(n) - square_sum(m))
                    .filter(|a| *a < bound && is_palindrome(a))
            })
            .collect::<HashSet<_>>()
            .iter()
            .sum::<u128>()
    );
}

fn square_sum(n: u128) -> u128 {
    n * (n + 1) * (2 * n + 1) / 6
}
