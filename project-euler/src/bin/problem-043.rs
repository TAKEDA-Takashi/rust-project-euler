//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2043
//!
//! 数1406357289は0から9のパンデジタル数である (0から9が1度ずつ現れるので). この数は部分文字列が面白い性質を持っている.
//!
//! d1を上位1桁目, d2を上位2桁目の数とし, 以下順にdnを定義する. この記法を用いると次のことが分かる.
//!
//! d2d3d4=406 は 2 で割り切れる
//! d3d4d5=063 は 3 で割り切れる
//! d4d5d6=635 は 5 で割り切れる
//! d5d6d7=357 は 7 で割り切れる
//! d6d7d8=572 は 11 で割り切れる
//! d7d8d9=728 は 13 で割り切れる
//! d8d9d10=289 は 17 で割り切れる
//! このような性質をもつ0から9のパンデジタル数の総和を求めよ.

use itertools::Itertools;

fn main() {
    let ds = [2, 3, 5, 7, 11, 13, 17];

    println!(
        "{}",
        (0..=9)
            .permutations(10)
            .filter(|p| p[0] != 0
                && ds.iter().enumerate().all(|(i, d)| &p[i + 1..i + 4]
                    .iter()
                    .join("")
                    .parse()
                    .unwrap()
                    % d
                    == 0))
            .filter_map(|p| p.iter().join("").parse::<usize>().ok())
            .sum::<usize>()
    );
}
