//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%20103
//!
//! 大きさ n の集合 A の要素の和を S(A) で表す. 空でなく共通要素を持たないいかなる 2 つの部分集合 B と C に対しても以下の性質が真であれば, A を特殊和集合と呼ぼう.
//!
//! i. S(B) ≠ S(C); つまり, 部分集合の和が等しくてはならない.
//! ii. B が C より多くの要素を含んでいればこのとき S(B) > S(C) となる.
//!
//! ある n に対し S(A) が最小化されていれば, それを最適特殊和集合と呼ぼう. はじめの 5 つの最適特殊和集合は下に与えられる.
//!
//! n = 1: {1}
//! n = 2: {1, 2}
//! n = 3: {2, 3, 4}
//! n = 4: {3, 5, 6, 7}
//! n = 5: {6, 9, 11, 12, 13}
//!
//! ある最適特殊和集合 A = {a1, a2, ... , an} に対し, 次の最適特殊和集合は B = {b, a1+b, a2+b, ... ,an+b} となりそうである. ここで b は前列の「中央の」要素である.
//!
//! この「法則」を用いると n = 6 に対する最適特殊和集合は A = {11, 17, 20, 22, 23, 24} で, S(A) = 117 と予期するだろう. しかしこれは, 最適に近い集合を与えるアルゴリズムを用いたにすぎないため, 最適特殊和集合とはならない. n = 6 に対する最適特殊和集合は A = {11, 18, 19, 20, 22, 25} で, S(A) = 115 である. これに対応する集合の文字列は 111819202225 である.
//!
//! A を n = 7 に対する最適特殊和集合とするとき, その集合の文字列を求めよ.
//!
//! 注意: この問題は Problem 105 と 106 に関連している.

use itertools::Itertools;
use std::collections::{BTreeSet, HashSet};

fn main() {
    let s6 = vec![11, 18, 19, 20, 22, 25];
    let ns = next_nearly_optimal_set(&s6);

    let optimize_ns = optimize(&ns);

    println!("{:?}", optimize_ns);
    println!("{}", optimize_ns.iter().join(""));
    println!(
        "{} -> {}",
        ns.iter().sum::<usize>(),
        optimize_ns.iter().sum::<usize>()
    );
}

fn optimize(ns: &Vec<usize>) -> Vec<usize> {
    fn optimize0(ns: &Vec<usize>, ubound: usize) -> Vec<usize> {
        let mut optimaze_ns = vec![ns[0]];
        optimaze_ns.extend(ns[1..].iter().map(|n| n - 5));

        let len = optimaze_ns.len();

        loop {
            let mut i = len - 1;

            optimaze_ns[i] += 1;

            while optimaze_ns.iter().sum::<usize>() >= ubound {
                if i == 0 {
                    return ns.clone();
                }

                optimaze_ns[i - 1] += 1;

                for j in i..len {
                    optimaze_ns[j] = optimaze_ns[j - 1] + 1;
                }

                i -= 1;
            }

            if optimaze_ns.iter().collect::<HashSet<_>>().len() != len
                || check_balance(&optimaze_ns) < 1
            {
                continue;
            }

            if check_combinations(&optimaze_ns) {
                return optimaze_ns;
            }
        }
    }

    optimize0(ns, ns.iter().sum::<usize>())
}

fn next_nearly_optimal_set(v: &Vec<usize>) -> Vec<usize> {
    let next_min = v[v.len() / 2];
    let mut next_set = vec![next_min];

    next_set.extend(v.iter().map(|n| n + next_min));
    next_set
}

fn check_balance(v: &Vec<usize>) -> isize {
    v[0..(v.len() + 1) / 2].iter().sum::<usize>() as isize
        - v[v.len() / 2 + 1..].iter().sum::<usize>() as isize
}

fn check_combinations(v: &Vec<usize>) -> bool {
    let set: BTreeSet<_> = v.iter().copied().collect();

    (2..=v.len() / 2).all(|n| {
        set.iter().copied().combinations(n).all(|p| {
            let p_sum = p.iter().sum::<usize>();

            (&set - &p.iter().copied().collect::<BTreeSet<_>>())
                .into_iter()
                .combinations(n)
                .all(|q| p_sum != q.iter().sum::<usize>())
        })
    })
}
