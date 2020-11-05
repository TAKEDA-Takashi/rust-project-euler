//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2098
//!
//! CARE という単語の各文字をそれぞれ 1, 2, 9, 6 に置き換えることによって, 平方数 1296 = 362 ができる. 注目すべきことに, 同じ数字の置換をつかうことにより, アナグラムの RACE も平方数 9216 = 962 をつくる. CARE (と RACE) を平方アナグラム単語対と呼ぼう. 先頭のゼロは許されず, 異なる文字が同じ数字をもつこともないとする.
//!
//! 約 2,000 個の一般的な英単語を含む 16K のテキストファイルfilewords.txt (右クリックして "名前をつけてリンク先を保存") を用いて, 平方アナグラム単語対をすべて求めよ (回文となる単語はそれ自身のアナグラムとはみなさない).
//!
//! そのような対のメンバーから作られる最大の平方数は何か?
//!
//! 注: 作られるアナグラムは, すべて与えられたテキストファイルに含まれている.

// 同じアルファベットには同じ数字が入ってもよいはずだが、厳密にそこはチェックしていない

use itertools::Itertools;
use std::collections::HashSet;
use std::fs;

type PermutationMatrix = Vec<(usize, usize)>;

fn main() {
    let content = fs::read_to_string("files/p098_words.txt")
        .unwrap()
        .replace("\"", "");

    let max = content
        .split(",")
        .combinations(2)
        .filter(|v| v[0].len() == v[1].len() && v[0].chars().sorted().eq(v[1].chars().sorted()))
        .filter_map(|v| {
            let a = v[0].to_string().chars().collect_vec();
            let b = v[1].to_string().chars().collect_vec();
            find_max_anagramic_square(a.len() as u32, &get_permutation_matrix(&a, &b))
        })
        .max();

    println!("{:?}", max);
}

fn find_max_anagramic_square(digit: u32, matrix: &PermutationMatrix) -> Option<u64> {
    let lbound = (10_u64.pow(digit - 1) as f64).sqrt() as u64;
    let ubound = (10_u64.pow(digit) as f64).sqrt() as u64;

    (lbound + 1..ubound)
        .rev()
        .map(|n| n * n)
        .filter(|n| n.to_string().chars().collect::<HashSet<_>>().len() == digit as usize)
        .find(|n| has_anagram_pair(n, &matrix))
}

fn has_anagram_pair(n: &u64, matrix: &PermutationMatrix) -> bool {
    if let Some(m) = get_anagram_number(n, matrix) {
        let m_rt = (m as f64).sqrt();

        if m_rt == m_rt.floor() {
            return true;
        }
    }

    if let Some(m2) = get_anagram_number(n, &matrix.iter().rev().copied().collect_vec()) {
        let m2_rt = (m2 as f64).sqrt();
        m2_rt == m2_rt.floor()
    } else {
        false
    }
}

fn get_anagram_number(n: &u64, matrix: &PermutationMatrix) -> Option<u64> {
    let mut ns = n.to_string().chars().collect_vec();

    for v in matrix {
        let tmp = ns[v.0];
        ns[v.0] = ns[v.1];
        ns[v.1] = tmp;
    }

    // 先頭が0だと桁が変わるのでNG
    if ns[0] == '0' {
        None
    } else {
        Some(ns.iter().collect::<String>().parse().ok()?)
    }
}

fn get_permutation_matrix(v1: &Vec<char>, v2: &Vec<char>) -> PermutationMatrix {
    // 置換しないインデックスは0
    let d = v1
        .iter()
        .zip(v2)
        .map(|(&a, &b)| a as i32 - b as i32)
        .collect_vec();

    let mut v1 = v1.clone();
    let mut pmatrix = vec![];

    let mut i = 0;
    while i < v1.len() {
        if v1[i] == v2[i] {
            i += 1;
            continue;
        }

        let j = v2[i..]
            .iter()
            .enumerate()
            .position(|(j, n)| d[i + j] != 0 && *n == v1[i])
            .unwrap()
            + i;

        pmatrix.push((i, j));

        let tmp = v1[i];
        v1[i] = v1[j];
        v1[j] = tmp;
    }

    pmatrix
}
