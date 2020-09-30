//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2035
//!
//! 197は巡回素数と呼ばれる. 桁を回転させたときに得られる数 197, 971, 719 が全て素数だからである.
//!
//! 100未満には巡回素数が13個ある: 2, 3, 5, 7, 11, 13, 17, 31, 37, 71, 73, 79, および97である.
//!
//! 100万未満の巡回素数はいくつあるか?

use problem_035::Prime;
use std::collections::{HashSet, VecDeque};

fn main() {
    let prime_set: HashSet<usize> = Prime::new().take_while(|&p| p < 1_000_000).collect();
    let mut correct_set: HashSet<usize> = HashSet::new();

    for p in &prime_set {
        if correct_set.contains(&p) {
            continue;
        }

        let ns = gen_cycle(*p);
        if ns.iter().all(|n| prime_set.contains(n)) {
            correct_set.extend(ns);
        }
    }

    println!("{}", correct_set.len());
}

fn gen_cycle(n: usize) -> Vec<usize> {
    let mut n_chars: VecDeque<char> = n.to_string().chars().collect();
    let mut res = vec![n];

    for _ in 0..n_chars.len() - 1 {
        let c = n_chars.pop_front().unwrap();
        n_chars.push_back(c);
        res.push(n_chars.iter().collect::<String>().parse().unwrap());
    }

    res
}
