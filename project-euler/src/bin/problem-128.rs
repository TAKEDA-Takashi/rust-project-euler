//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%20128
//!
//! 1の六角形のタイルは, 12時から反時計回りに配置された2から7の6個の六角形のタイルの輪に囲まれている.
//!
//! 8から19, 20から37, 38から61, , , といった新しい輪も同様にして加えられるものとする. 下図に最初の3個の輪を示す.
//!
//! p128.png
//! タイル n とそれに隣接するタイルについて, 差の値が素数となる個数を PD(n) と定義する.
//!
//! 例えば, タイル8では時計回りに 12, 29, 11, 6, 1, 13 となるので, PD(8) = 3 である.
//!
//! 同様に, タイル17では 1, 17, 16, 1, 11, 10となるので, PD(17) = 2 となる.
//!
//! PD(n) の最大値は3であることが示せる.
//!
//! PD(n) = 3 となるタイルを昇順に並べた数列では, 10番目のタイルは271となる.
//!
//! この数列について2000番目のタイルを求めよ.

// 1-2のラインと右隣にしかPD(n) = 3は現れない
// それ以外のタイルは、差が1のタイルが2つと自分と同じ偶奇のタイルが2つ隣接するのでPD(n) = 3にはならない

use euler_lib::Prime;

fn main() {
    let prime = Prime::new();

    println!(
        "{}",
        vec![0, 1]
            .into_iter()
            .chain((2..).flat_map(|n| {
                let mut v = vec![];

                let a = get_2line_number(n);
                let b = get_2line_number(n + 1) - 1; // 右下
                let c = get_2line_number(n + 1) + 1; // 左上
                let d = get_2line_number(n + 2) - 1; // 右上

                if prime.is_prime(&(a - b).abs()) {
                    if prime.is_prime(&(a - c).abs()) && prime.is_prime(&(a - d).abs()) {
                        v.push(a);
                    }

                    let x = b;
                    let y = get_2line_number(n - 1); // 左下
                    let z = get_2line_number(n + 2) - 2; // 右上

                    if prime.is_prime(&(x - y).abs()) && prime.is_prime(&(x - z).abs()) {
                        v.push(x)
                    }
                }

                v
            }))
            .nth(2000 - 1) // 0-origin
            .unwrap()
    )
}

fn get_2line_number(n: isize) -> isize {
    3 * n * (n - 1) + 2
}
