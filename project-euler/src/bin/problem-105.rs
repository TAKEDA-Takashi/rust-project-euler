//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%20105
//!
//! 大きさ n の集合 A の要素の和を S(A) で表す. 空でなく共通要素を持たないいかなる 2 つの部分集合 B と C に対しても以下の性質が真であれば, A を特殊和集合と呼ぼう.
//!
//! i. S(B)≠S(C); つまり, 部分集合の和が等しくてはならない.
//! ii. B が C より多くの要素を含んでいればこのとき S(B) > S(C) となる.
//!
//! たとえば, {81, 88, 75, 42, 87, 84, 86, 65} は, 65 + 87 + 88 = 75 + 81 + 84 であるため特殊和集合ではないが, {157, 150, 164, 119, 79, 159, 161, 139, 158} はすべての可能な部分集合の対の組み合わせについて両方のルールを満たしており, また S(A) = 1286 である.
//!
//! 7 から 12 の要素を含む 100 個の集合が記された 4K のテキストファイル sets.txt (右クリックして "名前をつけてリンク先を保存") を用いて (上の二例はファイルの最初の 2 集合である), 特殊和集合 A1, A2, ... , Ak をすべて特定し, S(A1) + S(A2) + ... + S(Ak) を求めよ.
//!
//! 注意: この問題は Problem 103 と 106 に関連している.

use itertools::Itertools;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let set_sum = BufReader::new(File::open("files/p105_sets.txt").unwrap())
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            line.split(",")
                .filter_map(|s| s.parse::<usize>().ok())
                .sorted()
                .collect_vec()
        })
        .filter(|v| check_balance(v) > 0 && check_combinations(v))
        .map(|v| v.iter().sum::<usize>())
        .sum::<usize>();

    println!("{}", set_sum);
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
