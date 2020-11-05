//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2022
//!
//! 5000個以上の名前が書かれている46Kのテキストファイル names.txt を用いる. まずアルファベット順にソートせよ.
//!
//! のち, 各名前についてアルファベットに値を割り振り, リスト中の出現順の数と掛け合わせることで, 名前のスコアを計算する.
//!
//! たとえば, リストがアルファベット順にソートされているとすると, COLINはリストの938番目にある. またCOLINは 3 + 15 + 12 + 9 + 14 = 53 という値を持つ. よってCOLINは 938 × 53 = 49714 というスコアを持つ.
//!
//! ファイル中の全名前のスコアの合計を求めよ.

use std::fs;

fn main() {
    let content = fs::read_to_string("files/p022_names.txt")
        .unwrap()
        .replace("\"", "");
    let mut names: Vec<&str> = content.split(",").collect();
    names.sort();

    println!(
        "{}",
        names
            .iter()
            .enumerate()
            .map(|(n, s)| (n + 1) * s.chars().map(|c| c as usize - 64).sum::<usize>())
            .sum::<usize>()
    );
}
