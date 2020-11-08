//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%20117
//!
//! 黒い正方形のタイルと, 2 ユニットの長さの赤のタイル, 3 ユニットの長さの緑のタイル, 4 ユニットの長さの青のタイルから選んで組み合わせて, 5 ユニットの長さの 1 列をタイルで敷く方法はちょうど 15 通りある.
//!
//! 117_1.png
//! 長さ 50 ユニットの 1 列をタイルで敷く方法は何通りあるか.
//!
//! 注: この問題は Problem 116 に関連する

use cached::proc_macro::cached;
use euler_lib::factorial;
use itertools::iproduct;
use num::{one, BigUint};

fn main() {
    let n = 50;
    let mut sum = one::<BigUint>(); // ブロックなし

    sum += block_combinations(2, n) + block_combinations(3, n) + block_combinations(4, n);

    for (a, b, c) in iproduct!(0..=n / 2, 0..=n / 3, 0..=n / 4) {
        let s = a * 2 + b * 3 + c * 4;
        if (a == 0 && b == 0) || (b == 0 && c == 0) || (c == 0 && a == 0) || s > n {
            continue;
        }

        sum += permutation_with_repetition(&vec![a, b, c, n - s]);
    }

    println!("{}", sum);
}

#[cached]
fn block_combinations(m: usize, n: usize) -> usize {
    if m > n {
        0
    } else if m == n {
        1
    } else {
        block_combinations(m, n - 1) + block_combinations(m, n - m) + 1
    }
}

fn permutation_with_repetition(v: &Vec<usize>) -> BigUint {
    let n = v.iter().sum::<usize>();
    factorial(&BigUint::from(n))
        / v.iter()
            .map(|&m| factorial(&BigUint::from(m)))
            .product::<BigUint>()
}
