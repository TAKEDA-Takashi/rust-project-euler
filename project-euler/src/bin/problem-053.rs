//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2053
//!
//! 12345から3つ選ぶ選び方は10通りである.
//!
//! 123, 124, 125, 134, 135, 145, 234, 235, 245, 345.
//! 組み合わせでは, 以下の記法を用いてこのことを表す: 5C3 = 10.
//!
//! 一般に, r ≤ n について nCr = n!/(r!(n-r)!) である. ここで, n! = n×(n−1)×...×3×2×1, 0! = 1 と階乗を定義する.
//!
//! n = 23 になるまで, これらの値が100万を超えることはない: 23C10 = 1144066.
//!
//! 1 ≤ n ≤ 100 について, 100万を超える nCr は何通りあるか?

use euler_lib::combination;
use std::iter::repeat;

fn main() {
    println!(
        "{}",
        (1..=100)
            .flat_map(|n| repeat(n).zip(0..=n))
            .filter(|&(n, r)| combination::<u128>(&n, &r) > 1_000_000)
            .count()
    );
}
