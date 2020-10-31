//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2099
//!
//! 指数の形で表される2つの数, 例えば 211 と 37, の大小を調べることは難しくはない. 電卓を使えば, 211 = 2048 < 37 = 2187 であることが確かめられる.
//!
//! しかし, 632382518061 > 519432525806 を確認することは非常に難しい (両者ともに300万桁以上になる).
//!
//! 各行に1組が書かれている1000個の組を含んだ22Kのテキストファイル base_exp.txt から, 最大の数が書かれている行の番号を求めよ.
//!
//! 注: ファイル中の最初の二行は上の例である.

use std::cmp::PartialOrd;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let (row_num, _) = BufReader::new(File::open("p099_base_exp.txt").unwrap())
        .lines()
        .filter_map(|line| line.ok())
        .enumerate()
        .map(|(i, line)| {
            let v = line
                .split(",")
                .filter_map(|s| s.parse::<u32>().ok())
                .collect::<Vec<_>>();

            (i + 1, (v[0] as f64).log10() * v[1] as f64)
        })
        .max_by(|(_, a), (_, b)| PartialOrd::partial_cmp(a, b).unwrap())
        .unwrap();

    println!("{}", row_num);
}
