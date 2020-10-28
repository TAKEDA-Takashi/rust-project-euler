//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2085
//!
//! 注意深く数えると, 横が3, 縦が2の長方形の格子には, 18個の長方形が含まれている.
//!
//! p085.png
//! ぴったり2,000,000個の長方形を含むような長方形の格子は存在しない. 一番近い解を持つような格子の面積を求めよ.

use std::iter::repeat;

fn main() {
    let (n, m, _) = (1..=2000)
        .flat_map(|n| repeat(n).zip(1..=2000)) // upper bound
        .map(|(n, m)| (n, m, get_rectangle_total(n, m)))
        .min_by_key(|(.., a)| (2_000_000 - *a as isize).abs())
        .unwrap();

    println!("{}", n * m);
}

fn get_rectangle_total(n: usize, m: usize) -> usize {
    n * m * (n + 1) * (m + 1) / 4
}
