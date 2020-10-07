//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2048
//!
//! 次の式は, 11 + 22 + 33 + ... + 1010 = 10405071317 である.
//!
//! では, 11 + 22 + 33 + ... + 10001000 の最後の10桁を求めよ.

fn main() {
    println!(
        "{}",
        (1..1000).map(|n| modpow(n, n, 10000000000)).sum::<u128>() % 10000000000
    );
}

fn modpow(b: u128, e: u128, m: u128) -> u128 {
    modpow0(b, e, m, 1)
}

fn modpow0(b: u128, e: u128, m: u128, r: u128) -> u128 {
    if e == 0 {
        r
    } else {
        modpow0(b * b % m, e >> 1, m, if e & 1 == 1 { r * b % m } else { r })
    }
}
