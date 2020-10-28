//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2042
//!
//! 三角数のn項は tn = ½n(n+1)で与えられる. 最初の10項は
//!
//! 1, 3, 6, 10, 15, 21, 28, 36, 45, 55, ...
//! である.
//!
//! 単語中のアルファベットを数値に変換した後に和をとる. この和を「単語の値」と呼ぶことにする. 例えば SKY は 19 + 11 + 25 = 55 = t10である. 単語の値が三角数であるとき, その単語を三角語と呼ぶ.
//!
//! 16Kのテキストファイル words.txt 中に約2000語の英単語が記されている. 三角語はいくつあるか?

use std::collections::HashSet;
use std::fs;

fn main() {
    let content = fs::read_to_string("p042_words.txt")
        .unwrap()
        .replace("\"", "");

    let tri_nums = (1..)
        .map(|n| n * (n + 1) / 2)
        .take_while(|&t| t < 193) // max word value is 192
        .collect::<HashSet<_>>();

    println!(
        "{}",
        content
            .split(",")
            .map(|s| s.chars().map(|c| c as u32 - 64).sum::<u32>())
            .filter(|n| tri_nums.contains(n))
            .count()
    );
}
