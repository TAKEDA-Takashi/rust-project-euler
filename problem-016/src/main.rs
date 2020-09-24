//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2016
//!
//! 215 = 32768 であり, 各位の数字の和は 3 + 2 + 7 + 6 + 8 = 26 となる.
//!
//! 同様にして, 21000 の各位の数字の和を求めよ.
//!
//! 注: Problem 20 も各位の数字の和に関する問題です。解いていない方は解いてみてください。

use num_bigint::BigUint;

fn main() {
    println!(
        "{}",
        BigUint::from(2_u32)
            .pow(1000)
            .to_string()
            .chars()
            .filter_map(|c| c.to_digit(10))
            .sum::<u32>()
    );
}
