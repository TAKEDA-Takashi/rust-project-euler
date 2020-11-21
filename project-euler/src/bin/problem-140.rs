//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%20140
//!
//! Modified Fibonacci golden nuggets

use num::integer::Roots;
use std::iter::once;

fn main() {
    // n^2 - mn - m^2 = 1のパターン
    // N^2 - 5m^2 = 4
    let iter1 = once((1, 2))
        .chain(
            (0..)
                .scan((3_u128, 1), |acc, _| {
                    let x = (3 * acc.0 + 5 * acc.1) / 2;
                    let y = (acc.0 + 3 * acc.1) / 2;
                    *acc = (x, y);
                    Some((x, y))
                })
                .map(|(a, b)| (b, (a + b) / 2)),
        ) // m, n
        .take(15);

    // n^2 - mn - m^2 = 11のパターン
    let iter11 = (1_u128..)
        .filter_map(|m| {
            let d = 5 * m * m + 44;
            let rd = d.sqrt();

            if rd * rd == d && (m + rd) % 2 == 0 {
                let n = (m + rd) / 2;
                let right = 3 * m * m + n * m;

                if right % 11 == 0 {
                    return Some((m, n));
                }
            }
            None
        })
        .take(15);

    println!(
        "{}",
        iter1
            .chain(iter11)
            .map(|(m, n)| (3 * m * m + n * m) / (n * n - n * m - m * m))
            .sum::<u128>()
    );
}
