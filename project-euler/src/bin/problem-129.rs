//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%20129
//!
//! 1のみからなる数をレピュニットという. R(k) を長さ k のレピュニットとする. 例えば, R(6) = 111111 となる.
//!
//! GCD(n, 10) = 1 なる正の整数 n が与えられたとき, R(k) が n で割り切られるような k が常に存在することが示せる. A(n) をそのような k の最小のものとする. 例えば, A(7) = 6, A(41) = 5 となる.
//!
//! A(n) の値が10を超える最小の n は17である.
//!
//! A(n) の値が100万を超える最小の n を求めよ.

// 循環小数の循環節の長さが100万を超える数を探索する。
// nが素数であれば n-1 の約数であることは既知なので、そのうち長さが n-1 であるものが候補となる。
// A(p) = p - 1
// 一方でその数と 3のべき を乗算したものも候補となる。
// A(3^x * p) = 3^x * (p - 1)
// ただし
// A(3^x * p) = p - 1
// となる場合もあるため検査が必要

use euler_lib::{get_divisors, modpow, Prime};
use itertools::repeat_n;
use num::{BigUint, Integer, One, Zero};

thread_local!(static PRIME: Prime<usize> = Prime::new());

fn main() {
    let prime = Prime::<usize>::new();

    let n = (1_000_000..).filter(|&n| n.gcd(&10).is_one()).find(|n| {
        let v = prime.factorization(n);

        if v.len() == 1 {
            is_max_recurring(v[0])
        } else if v.len() == 2 && v[0] == 3 && is_max_recurring(v[1]) {
            !(repeat_n('1', (v[1] - 1) as usize)
                .collect::<String>()
                .parse::<BigUint>()
                .unwrap()
                % n)
                .is_zero()
        } else {
            false
        }
    });

    println!("{:?}", n);
}

/// 素数pの逆数(1/p)の循環節の長さがp-1ならtrueを返す。
fn is_max_recurring(p: usize) -> bool {
    get_divisors(&(p - 1))
        .iter()
        .skip(1)
        .all(|&m| modpow(10, m, p) != 1)
}
