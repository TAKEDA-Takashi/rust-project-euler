//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2020
//!
//! n × (n - 1) × ... × 3 × 2 × 1 を n! と表す.
//!
//! 例えば, 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800 となる.
//! この数の各桁の合計は 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27 である.
//!
//! では, 100! の各位の数字の和を求めよ.
//!
//! 注: Problem 16 も各位の数字の和に関する問題です。解いていない方は解いてみてください。

use num_bigint::BigUint;

fn main() {
    println!(
        "{}",
        factorial(100)
            .to_string()
            .chars()
            .filter_map(|c| c.to_digit(10))
            .sum::<u32>()
    );
}

fn factorial(n: usize) -> BigUint {
    match n {
        0 => BigUint::from(1_u32),
        1 => BigUint::from(1_u32),
        _ => (2..n).fold(BigUint::from(n), |acc, m| acc * m),
    }
}
