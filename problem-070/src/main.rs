//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2070
//!
//! オイラーのトーティエント関数 φ(n) (ファイ関数とも呼ばれる) とは, n 未満の正の整数で n と互いに素なものの個数を表す. 例えば, 1, 2, 4, 5, 7, 8 は9未満で9と互いに素であるので, φ(9) = 6 となる.
//! 1 は全ての正の整数と互いに素であるとみなされる. よって φ(1) = 1 である.
//!
//! 面白いことに, φ(87109)=79180 であり, 87109は79180を置換したものとなっている.
//!
//! 1 < n < 107 で φ(n) が n を置換したものになっているもののうち, n/φ(n) が最小となる n を求めよ.

use euler_lib::Prime;
use itertools::Itertools;
use std::cmp::PartialOrd;

fn main() {
    let upper_bound = 10_000_000;
    let mut prime = Prime::<usize>::new();
    let ps = prime
        .iter()
        .take_while(|p| p * p < upper_bound)
        .collect_vec();

    let (n, _) = ps
        .iter()
        .filter_map(|p| {
            let v = prime
                .iter()
                .skip_while(|np| np < p)
                .take_while(|np| p * np < upper_bound)
                .map(|np| (*p * np, (*p, np)))
                .collect_vec();

            v.iter()
                .rev()
                .map(|(n, pp)| (*n, calc_phi(n, pp)))
                .find(|(n, ph)| is_replacement(n, &ph.0))
        })
        .min_by(|(_, (_, s1)), (_, (_, s2))| PartialOrd::partial_cmp(&s1, &s2).unwrap())
        .unwrap();

    println!("{}", n);
}

fn is_replacement(a: &usize, b: &usize) -> bool {
    itertools::equal(
        a.to_string().chars().sorted(),
        b.to_string().chars().sorted(),
    )
}

// 2つの素数に限定したφ(n)およびn/φ(n)を計算する
fn calc_phi(n: &usize, (a, b): &(usize, usize)) -> (usize, f64) {
    let x = *a as f64;
    let y = *b as f64;

    if a == b {
        (n - a, x / (x - 1_f64))
    } else {
        ((a - 1) * (b - 1), (x / (x - 1_f64)) * (y / (y - 1_f64)))
    }
}
