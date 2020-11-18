//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%20133
//!
//! 1のみからなる数をレピュニットという. R(k) を長さ k のレピュニットとする. 例えば, R(6) = 111111 となる.
//!
//! R(10n) というレピュニットについて考える.
//!
//! R(10), R(100), R(1000) は 17 では割り切れないが, R(10000) は 17 で割り切られる. さらに, R(10n) が 19 で割り切られるような n は存在しない. 驚くべきことに, R(10n) の因数となりうる100未満の素数は 11, 17, 41, 73 の4個のみである.
//!
//! R(10n) の因数となりえない100000未満の素数の和を求めよ.

use euler_lib::{modpow, Prime};

fn main() {
    let ubound = 100_000;
    let prime = Prime::<u128>::new();

    let mut v = vec![];

    for p in prime.iter().skip(3).take_while(|&p| p < ubound) {
        for i in 1..17 {
            if modpow(10, 10_u128.pow(i), p) == 1 {
                v.push(p);
                break;
            }
        }
    }

    // println!("{:?}", v);
    println!(
        "{}",
        prime.iter().take_while(|&p| p < ubound).sum::<u128>() - v.iter().sum::<u128>()
    );
}
