//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2056
//!
//! Googol (10100)は非常に大きな数である: 1の後に0が100個続く. 100100は想像を絶する. 1の後に0が200回続く. その大きさにも関わらず, 両者とも数字和 ( 桁の和 ) は1である.
//!
//! a, b < 100 について自然数 ab を考える. 数字和の最大値を答えよ.

use num_bigint::BigUint;
use std::iter::repeat;

fn main() {
    println!(
        "{}",
        (1..100_u32)
            .flat_map(|n| repeat(n).zip(1..100))
            .map(|(a, b)| BigUint::from(a)
                .pow(b)
                .to_string()
                .chars()
                .filter_map(|c| c.to_digit(10))
                .sum::<u32>())
            .max()
            .unwrap()
    );
}
