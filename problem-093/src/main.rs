//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2093
//!
//! 集合 {1, 2, 3, 4} の各数字をちょうど一度用い, また四則演算 (+, －, *, /) と括弧を使うことにより, 異なる正の整数を作ることができる.
//!
//! 例えば,
//!
//! 8 = (4 * (1 + 3)) / 2
//! 14 = 4 * (3 + 1 / 2)
//! 19 = 4 * (2 + 3) - 1
//! 36 = 3 * 4 * (2 + 1)
//!
//! 12 + 34 のように数字をつなげることは許されないことに注意しよう.
//!
//! 集合 {1, 2, 3, 4} を使うと, 36 を最大とする 31 個の異なる数が得られる. 最初の表現できない数に会うまで, 1 から 28 の各数を得ることができる.
//!
//! 最長の連続した正の整数 1 から n の集合を得ることができる, 4 つの異なる数字 a < b < c < d を見つけよ. 答えを文字列 abcd として与えよ.

use eval::eval;
use itertools::{iproduct, Itertools};
use std::collections::BTreeSet;

fn main() {
    let symbols = ['+', '-', '*', '/'];
    let mut max_numbers = (0, vec![]);

    for numbers in (1..10).combinations(4) {
        let result_set: BTreeSet<u64> = iproduct!(
            numbers.iter().permutations(4),
            iproduct!(&symbols, &symbols, &symbols)
        )
        .flat_map(|(nums, sym)| {
            vec![
                format!(
                    "(({}{}{}){}{}){}{}",
                    nums[0], sym.0, nums[1], sym.1, nums[2], sym.2, nums[3]
                ),
                format!(
                    "({}{}({}{}{})){}{}",
                    nums[0], sym.0, nums[1], sym.1, nums[2], sym.2, nums[3]
                ),
                format!(
                    "{}{}(({}{}{}){}{})",
                    nums[0], sym.0, nums[1], sym.1, nums[2], sym.2, nums[3]
                ),
                format!(
                    "{}{}({}{}({}{}{}))",
                    nums[0], sym.0, nums[1], sym.1, nums[2], sym.2, nums[3]
                ),
            ]
            .iter()
            .filter_map(|expr| eval(expr).ok())
            .filter_map(|n| {
                if n.is_f64() {
                    n.as_f64()
                        .map(|a| a.abs())
                        .filter(|a| *a == a.floor())
                        .map(|a| a as u64)
                } else if n.is_i64() {
                    n.as_i64().map(|a| a.abs() as u64)
                } else {
                    None
                }
            })
            .collect::<BTreeSet<_>>()
        })
        .collect();

        if result_set.contains(&1) {
            for (&a, &b) in result_set.iter().tuple_windows() {
                if a + 1 != b {
                    if a > max_numbers.0 {
                        max_numbers = (a, numbers.clone());
                    }
                    break;
                }
            }
        }
    }

    println!("{:?}", max_numbers);
}
