//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2023
//!
//! 完全数とは, その数の真の約数の和がそれ自身と一致する数のことである. たとえば, 28の真の約数の和は, 1 + 2 + 4 + 7 + 14 = 28 であるので, 28は完全数である.
//!
//! 真の約数の和がその数よりも少ないものを不足数といい, 真の約数の和がその数よりも大きいものを過剰数と呼ぶ.
//!
//! 12は, 1 + 2 + 3 + 4 + 6 = 16 となるので, 最小の過剰数である. よって2つの過剰数の和で書ける最少の数は24である. 数学的な解析により, 28123より大きい任意の整数は2つの過剰数の和で書けることが知られている. 2つの過剰数の和で表せない最大の数がこの上限よりも小さいことは分かっているのだが, この上限を減らすことが出来ていない.
//!
//! 2つの過剰数の和で書き表せない正の整数の総和を求めよ.

use std::collections::HashSet;
use std::iter::repeat;

fn main() {
    let ns: HashSet<usize> = (12..=28111).filter(is_abundant_number).collect();
    let abundant_sum_set: HashSet<usize> = ns
        .iter()
        .flat_map(|a| repeat(a).zip(ns.iter()))
        .map(|(a, b)| a + b)
        .collect();

    println!(
        "{}",
        (1..=28123)
            .collect::<HashSet<usize>>()
            .difference(&abundant_sum_set)
            .sum::<usize>()
    );
}

fn is_abundant_number(n: &usize) -> bool {
    get_divisor(*n).iter().sum::<usize>() > *n
}

// Same problem-021
fn get_divisor(n: usize) -> Vec<usize> {
    (1..=n / 2).fold(vec![], |mut acc, m| {
        if n % m == 0 {
            acc.push(m);
        }
        acc
    })
}
