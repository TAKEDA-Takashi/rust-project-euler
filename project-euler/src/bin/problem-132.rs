//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%20132
//!
//! 1のみからなる数をレピュニットという. R(k) を長さ k のレピュニットとする.
//!
//! 例えば, R(10) = 1111111111 = 11×41×271×9091 となり, 素因数の和は9414となる.
//!
//! R(109) の最初の40個の素因数の和を求めよ.

// R(n) = (10**n - 1) / 9
// (10**n - 1) / 9 == 0 mod p
// 10**n - 1 == 0 mod p
// 10**n == 1 mod p

use euler_lib::{modpow, Prime};

fn main() {
    let exp = 10_u32.pow(9);

    println!(
        "{}",
        Prime::<u128>::new()
            .iter()
            .filter(|&p| p != 3 && modpow(10, exp, p) == 1)
            .take(40)
            .sum::<u128>()
    );
}
