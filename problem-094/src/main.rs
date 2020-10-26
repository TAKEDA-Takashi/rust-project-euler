//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2094
//!
//! 一辺の長さが整数の正三角形は面積が整数にならないことを示すのは簡単である. しかし, 5-5-6の辺を持つ殆ど正三角形に近い擬正三角形 (almost equilateral triangle) は面積が12で整数である.
//!
//! 以降, 二等辺三角形で, 3つめの辺の長さが他と1つしか違わないもの (5-5-6, 5-5-4等) を, 擬正三角形と呼ぶ.
//!
//! さて, 周囲の長さが1,000,000,000以下の面積が整数になる擬正三角形を考え, その周囲の長さの総和を求めよ.

use euler_lib::primitive_pythagorean_triple;
use std::iter::repeat;

fn main() {
    let sum = (2..22500_i64) // ubound
        .flat_map(|m| repeat(m).zip(1..m))
        .filter_map(|(m, n)| primitive_pythagorean_triple(m, n))
        .filter(|(a, _, c)| (c - 2 * a).abs() == 1 && a + c <= 500_000_000)
        .map(|(a, _, c)| 2 * a + 2 * c)
        .sum::<i64>();

    println!("{}", sum);
}
