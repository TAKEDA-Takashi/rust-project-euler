//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%20138
//! 底の長さbが16, 脚の長さLが17の二等辺三角形を考える.
//!
//! p138.png
//! ピタゴラスの定理より, 三角形の高さh = √(172 − 82) = 15となる. 高さは底の長さより1だけ短い.
//!
//! b = 272, L = 305とすると, h = 273となり, これは底の長さより1だけ長い. この三角形はh = b ± 1という性質を持つ二等辺三角形の中で二番目に小さい.
//!
//! h = b ± 1, b, Lが全て正の整数であるとし, そのような二等辺三角形の中で小さい順に12個取ったときの∑Lを求めよ.

// 原始ピタゴラス数の生成式から条件を整理するとペル方程式に帰着できる。
// m^2 - n^2 - 4mn = +-1
// (m - 2n)^2 -5n^2 = +-1

use euler_lib::primitive_pythagorean_triple;
use std::iter::once;

fn main() {
    // M^2 - 5n^2 = -1
    let ns = (0..)
        .scan((2_u128, 1), |acc, _| {
            let x = 2 * acc.0 + 5 * acc.1;
            let y = acc.0 + 2 * acc.1;
            *acc = (x, y);
            Some((x, y))
        })
        .map(|(a, b)| (a + 2 * b, b))
        .filter_map(|(m, n)| primitive_pythagorean_triple(m, n))
        .take(11)
        .chain(once(primitive_pythagorean_triple(4, 1).unwrap()))
        .collect::<Vec<_>>();

    // println!("{:?}", ns);
    println!("{:?}", ns.iter().map(|(.., c)| c).sum::<u128>());
}
