//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%209
//!
//! ピタゴラス数(ピタゴラスの定理を満たす自然数)とは a < b < c で以下の式を満たす数の組である.
//!
//! a2 + b2 = c2
//! 例えば, 32 + 42 = 9 + 16 = 25 = 52 である.
//!
//! a + b + c = 1000 となるピタゴラスの三つ組が一つだけ存在する.
//! これらの積 abc を計算しなさい.

use std::iter::repeat;

fn make_pythagorean_triple(m: usize, n: usize) -> (usize, usize, usize) {
    assert!(m > n);

    let a = m * m - n * n;
    let b = 2 * m * n;
    let c = m * m + n * n;
    if a < b {
        (a, b, c)
    } else {
        (b, a, c)
    }
}

fn main() {
    let (a, b, c) = (2..)
        .flat_map(|n| repeat(n).zip(1..n)) // 原始ピラゴラス数に限定しないため互いに素は条件に含めない
        .map(|(m, n)| make_pythagorean_triple(m, n))
        .find(|(a, b, c)| a + b + c == 1000)
        .unwrap();

    // println!("a = {}, b = {}, c = {}", a, b, c);
    println!("{}", a * b * c);
}
