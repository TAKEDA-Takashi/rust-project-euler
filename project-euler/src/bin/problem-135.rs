//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%20135
//!
//! 正の整数x, y, z が等差数列の連続する項として与えられたとき, x2 - y2 - z2 = n がちょうど2個の解を持つような最小の正の整数 n は, n = 27である.
//!
//! 342 − 272 − 202 = 122 − 92 − 62 = 27
//!
//! n = 1155 は, 方程式がちょうど10個の解を持つ最小の値である.
//!
//! ちょうど10個の解を持つような n は, 100万までにいくつ存在するか?

// x, y, z := a + b, a, a - b
//
// (a + b)^2 - a^2 - (a - b)^2 = n
// a^2 + 2ab + b^2 - a^2 - a^2 + 2ab - b^2 = n
// 4ab - a^2 = n
// a(4b - a) = n

use euler_lib::{make_divisors, Prime};
use num::Integer;
use std::collections::HashMap;

fn main() {
    let ubound = 1_000_000;
    let prime = Prime::new();
    let mut table: HashMap<usize, usize> = HashMap::new();

    for n in 1155..ubound {
        let ps = prime.factorization(&n);

        for a in make_divisors(&ps) {
            let (b, rem) = (n / a + a).div_rem(&4);
            if rem != 0 || a <= b {
                continue;
            }

            *table.entry(n).or_default() += 1;
        }
    }

    println!("{}", table.iter().filter(|(_, v)| **v == 10).count());
}
