//! http://odz.sakura.ne.jp/projecteuler/index.php?Problem%20110
//!
//! 次の等式で x, y, n は正の整数である.
//!
//! 1/x + 1/y = 1/n
//!
//! n = 1260 では 113 の異なる解があり, この n が解の個数が 100 を超える最小の値である.
//!
//! 解の数が 4,000,000 を超える最小の n を求めよ.
//!
//! 注: この問題は Problem 108 を非常に難しくしたケースである. 総当り法で解ける範囲を超えているので, 賢い解き方が求められる.

// 108と基本方針は同じ。ただし素因数の組み合わせが不明なので総当たりに近い感じで探索する。

use euler_lib::{divisor_count, Prime};
use itertools::iproduct;

fn main() {
    let target = 4_000_000 * 2;

    let pow_vec = find_best_pow(target);

    let prime = Prime::<u128>::new();
    let mut prime_iter = prime.iter();

    let n = pow_vec
        .iter()
        .rev()
        .zip(&[5, 3, 2, 1])
        .map(|(&tk, &p)| {
            prime_iter
                .by_ref()
                .take(tk as usize)
                .product::<u128>()
                .pow(p)
        })
        .product::<u128>();

    let solve = divisor_count(&prime.factorization(&(n * n))) / 2;
    println!("{}: {}", n, solve);
}

fn calc(ps: &Vec<u32>) -> u128 {
    [3_u128, 5, 7, 11]
        .iter()
        .zip(ps)
        .map(|(&b, &p)| b.pow(p))
        .product()
}

fn find_best_pow(target: u128) -> Vec<u32> {
    let mut current_diff = target;
    let mut best_pow = vec![0];

    for (a, b, c, d) in iproduct!(
        0..(target as f64).log(3.) as u32,
        0..(target as f64).log(5.) as u32,
        0..(target as f64).log(7.) as u32,
        0..(target as f64).log(11.) as u32
    ) {
        let try_pow = vec![a, b, c, d];

        let prod = calc(&try_pow);
        if prod > target {
            let score = prod - target;
            if current_diff > score {
                current_diff = score;
                best_pow = try_pow;
            }
        }
    }

    best_pow
}
