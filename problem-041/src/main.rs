//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2041
//!
//! n桁パンデジタルであるとは, 1からnまでの数を各桁に1つずつ持つこととする.
//!
//! #下のリンク先にあるような数学的定義とは異なる
//!
//! 例えば2143は4桁パンデジタル数であり, かつ素数である. n桁（この問題の定義では9桁以下）パンデジタルな素数の中で最大の数を答えよ.

// パンデジタル数の定義から8桁と9桁の素数のパンデジタル数は存在しない（必ず3の倍数になる）

use itertools::Itertools;
use problem_041::is_prime;

fn main() {
    println!(
        "{}",
        (1..=7)
            .permutations(7)
            .filter_map(|v| { v.iter().join("").parse::<usize>().ok() })
            .filter(|&n| is_prime(n))
            .max()
            .unwrap()
    );
}
