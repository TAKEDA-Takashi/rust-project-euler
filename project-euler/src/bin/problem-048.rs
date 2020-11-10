//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2048
//!
//! 次の式は, 11 + 22 + 33 + ... + 1010 = 10405071317 である.
//!
//! では, 11 + 22 + 33 + ... + 10001000 の最後の10桁を求めよ.

use euler_lib::modpow;

fn main() {
    println!(
        "{}",
        (1..1000).map(|n| modpow(n, n, 10000000000)).sum::<u128>() % 10000000000
    );
}
