//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2072
//!
//! nとdを正の整数として, 分数 n/d を考えよう. n<d かつ HCF(n,d)=1 のとき, 真既約分数と呼ぶ.
//!
//! d ≤ 8について真既約分数を大きさ順に並べると, 以下を得る:
//!
//! 1/8, 1/7, 1/6, 1/5, 1/4, 2/7, 1/3, 3/8, 2/5, 3/7, 1/2, 4/7, 3/5, 5/8, 2/3, 5/7, 3/4, 4/5, 5/6, 6/7, 7/8
//! この集合は21個の要素をもつことが分かる.
//!
//! d ≤ 1,000,000について, 真既約分数の集合は何個の要素を持つか?

// 要は2-1000000のファイ関数の総和を求めればいい

use euler_lib::Prime;
use itertools::Itertools;

fn main() {
    let prime = Prime::<usize>::new();

    let sum: usize = (2..=1_000_000)
        .map(|n| phi(&n, &prime.factorization(&n)))
        .sum();
    println!("{}", sum);
}

fn phi(n: &usize, v: &Vec<usize>) -> usize {
    v.iter().dedup().fold(*n, |prod, p| prod / p * (p - 1))
}
