//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%20134
//!
//! 連続する素数 p1 = 19, p2 = 23 について考える. 1219 は末尾の桁が p1 からなり p2 で割り切られる最小の数であることが確かめられる.
//!
//! 実際, p1 = 3, p2 = 5 を除けば, 全ての p2 > p1 なる連続する素数のペアについて, 末尾の桁が p1 からなり p2 で割り切られる数 n が存在する. S を n の最小のものであるとする.
//!
//! 5 ≤ p1 ≤ 1000000 を満たす連続する素数のペア全てに対し ∑ S を求めよ.

// 問題を整理すると b := 10^d. dはp1の桁数 => b * k + p1 == 0 mod p2 となる k を見つければよい。
// b * k == -p1 mod p2
// b * k == p2 - p1 mod p2
// k == b' * (p2 - p1) mod p2
//
// ここで b' は mod p2 の下でのkの逆元
// ところで、b := 10^d であったので b' = (10^d)'
// 10 ⊥ p2 なのでフェルマーの小定理から b' = b^(p2 - 2) mod p2

use euler_lib::{modpow, Prime};
use itertools::Itertools;

fn main() {
    let ubound = 1_000_000;
    let prime_iter = Prime::<u128>::new().iter().skip(2);

    println!(
        "{}",
        prime_iter
            .tuple_windows()
            .take_while(|(p1, _)| *p1 <= ubound)
            .map(|(p1, p2)| {
                let digit = ((p1 as f64).log10() as u32) + 1;
                let base = 10_u128.pow(digit);
                let inv = modpow(base, p2 - 2, p2);
                base * (inv * (p2 - p1) % p2) + p1
            })
            .sum::<u128>()
    );
}
