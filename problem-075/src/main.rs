//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2075
//!
//! ある長さの鉄線を折り曲げて3辺の長さが整数の直角三角形を作るとき, その方法が1通りしかないような最短の鉄線の長さは12cmである. 他にも沢山の例が挙げられる.
//!
//! 12 cm: (3,4,5)
//! 24 cm: (6,8,10)
//! 30 cm: (5,12,13)
//! 36 cm: (9,12,15)
//! 40 cm: (8,15,17)
//! 48 cm: (12,16,20)
//!
//! それとは対照的に, ある長さの鉄線 (例えば20cm) は3辺の長さが整数の直角三角形に折り曲げることができない. また2つ以上の折り曲げ方があるものもある. 2つ以上ある例としては, 120cmの長さの鉄線を用いた場合で, 3通りの折り曲げ方がある.
//!
//! 120 cm: (30,40,50), (20,48,52), (24,45,51)
//!
//! Lを鉄線の長さとする. 直角三角形を作るときに1通りの折り曲げ方しか存在しないような L ≤ 1,500,000 の総数を答えよ.
//!
//! 注: この問題は最近変更されました. あなたが正しいパラメータを使っているか確認してください.

use num_integer::Integer;
use std::collections::HashSet;
use std::iter::repeat;

fn main() {
    let ubound = 1_500_000;

    let v: Vec<_> = (2..870) // upper bound
        .flat_map(|m| repeat(m).zip(1..m))
        .filter_map(|(m, n)| primitive_pythagorean_triple(m, n))
        .map(|(a, b, c)| a + b + c)
        .filter(|&s| s <= ubound)
        .collect();

    let mut pythagorean_set: HashSet<_> = v.iter().copied().collect();
    let mut common_multiple_set = HashSet::new();

    v.iter().for_each(|&n| {
        (2..)
            .map(|m| n * m)
            .take_while(|&p| p <= ubound)
            .for_each(|p| {
                if !pythagorean_set.insert(p) {
                    common_multiple_set.insert(p);
                }
            });
    });

    println!("{}", pythagorean_set.len() - common_multiple_set.len());
}

fn primitive_pythagorean_triple(m: usize, n: usize) -> Option<(usize, usize, usize)> {
    assert!(m > n);

    if (m - n) % 2 != 1 || m.gcd(&n) != 1 {
        return None;
    }

    let a = m * m - n * n;
    let b = 2 * m * n;
    let c = m * m + n * n;

    if a < b {
        Some((a, b, c))
    } else {
        Some((b, a, c))
    }
}
