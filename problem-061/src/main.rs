//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2061
//!
//! 三角数, 四角数, 五角数, 六角数, 七角数, 八角数は多角数であり, それぞれ以下の式で生成される.
//!
//! 三角数	P3,n=n(n+1)/2	1, 3, 6, 10, 15, ...
//! 四角数	P4,n=n2	1, 4, 9, 16, 25, ...
//! 五角数	P5,n=n(3n-1)/2	1, 5, 12, 22, 35, ...
//! 六角数	P6,n=n(2n-1)	1, 6, 15, 28, 45, ...
//! 七角数	P7,n=n(5n-3)/2	1, 7, 18, 34, 55, ...
//! 八角数	P8,n=n(3n-2)	1, 8, 21, 40, 65, ...
//! 3つの4桁の数の順番付きの集合 (8128, 2882, 8281) は以下の面白い性質を持つ.
//!
//! この集合は巡回的である. 最後の数も含めて, 各数の後半2桁は次の数の前半2桁と一致する
//! それぞれ多角数である: 三角数 (P3,127=8128), 四角数 (P4,91=8281), 五角数 (P5,44=2882) がそれぞれ別の数字で集合に含まれている
//! 4桁の数の組で上の2つの性質をもつはこの組だけである.
//! 三角数, 四角数, 五角数, 六角数, 七角数, 八角数が全て表れる6つの巡回する4桁の数からなる唯一の順序集合の和を求めよ.

use itertools::Itertools;
use std::collections::HashSet;

const NARROW_ROUND: usize = 10;

fn main() {
    let triangle = (1..)
        .map(|n| n * (n + 1) / 2)
        .skip_while(|&f| f < 1000)
        .take_while(|&f| f < 10000)
        .collect::<HashSet<usize>>();

    let square = (1..)
        .map(|n| n * n)
        .skip_while(|&f| f < 1000)
        .take_while(|&f| f < 10000)
        .collect::<HashSet<usize>>();

    let pentagonal = (1..)
        .map(|n| n * (3 * n - 1) / 2)
        .skip_while(|&f| f < 1000)
        .take_while(|&f| f < 10000)
        .collect::<HashSet<usize>>();

    let hexagonal = (1..)
        .map(|n| n * (2 * n - 1))
        .skip_while(|&f| f < 1000)
        .take_while(|&f| f < 10000)
        .collect::<HashSet<usize>>();

    let heptagonal = (1..)
        .map(|n| n * (5 * n - 3) / 2)
        .skip_while(|&f| f < 1000)
        .take_while(|&f| f < 10000)
        .collect::<HashSet<usize>>();

    let octagonal = (1..)
        .map(|n| n * (3 * n - 2))
        .skip_while(|&f| f < 1000)
        .take_while(|&f| f < 10000)
        .collect::<HashSet<usize>>();

    let v = vec![
        triangle, square, pentagonal, hexagonal, heptagonal, octagonal,
    ];

    let cyclical_numbers = (0..v.len())
        .permutations(v.len())
        .map(|perm| perm.iter().map(|&i| v[i].clone()).collect_vec())
        .filter_map(|mut v| {
            for _ in 0..NARROW_ROUND {
                for i in 0..v.len() {
                    v[i] = v[i]
                        .iter()
                        .filter(|&&a| {
                            v[(i + 1) % v.len()].iter().any(|&b| a % 100 == b / 100)
                                && v[(i + v.len() - 1) % v.len()]
                                    .iter()
                                    .any(|&b| b % 100 == a / 100)
                        })
                        .copied()
                        .collect::<HashSet<_>>();
                }

                if v.iter().any(|s| s.len() == 0) {
                    return None;
                } else if v.iter().all(|s| s.len() == 1) {
                    return Some(v);
                }
            }

            None
        })
        .next()
        .unwrap();

    println!(
        "{}",
        cyclical_numbers
            .iter()
            .map(|s| s.iter().next().unwrap())
            .sum::<usize>()
    );
}
