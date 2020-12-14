//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%20149
//! Searching for a maximum-sum subsequence

// 縦横で最大値が求まったので斜めは省略

use cached::proc_macro::cached;
use itertools::Itertools;

fn main() {
    let table: Vec<_> = (1..=4_000_000)
        .map(|n| lagged_fibonacci_generator(n))
        .collect();

    let yoko_max = table
        .iter()
        .chunks(2000)
        .into_iter()
        .map(|chunk| {
            let (a, b) = chunk.fold((0, 0), |acc, &n| {
                let a = n.max(acc.0 + n);
                let b = a.max(acc.1);
                (a, b)
            });
            a.max(b)
        })
        .max();

    let tate_max = (0..2000)
        .map(|n| (0..2000).map(move |m| m * 2000 + n))
        .map(|indexes| {
            let (a, b) = indexes.fold((0, 0), |acc, i| {
                let a = table[i].max(acc.0 + table[i]);
                let b = a.max(acc.1);
                (a, b)
            });
            a.max(b)
        })
        .max();

    println!("{:?} {:?}", yoko_max, tate_max);
}

#[cached]
fn lagged_fibonacci_generator(k: i128) -> i128 {
    if 1 <= k && k <= 55 {
        (100_003 - 200_003 * k + 300_007 * k * k * k) % 1_000_000 - 500_000
    } else if 56 <= k && k <= 4_000_000 {
        (lagged_fibonacci_generator(k - 24) + lagged_fibonacci_generator(k - 55) + 1_000_000)
            % 1_000_000
            - 500_000
    } else {
        panic!("not supported value: {}", k);
    }
}
