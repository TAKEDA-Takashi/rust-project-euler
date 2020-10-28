//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2021
//!
//! d(n) を n の真の約数の和と定義する. (真の約数とは n 以外の約数のことである. )
//! もし, d(a) = b かつ d(b) = a (a ≠ b のとき) を満たすとき, a と b は友愛数(親和数)であるという.
//!
//! 例えば, 220 の約数は 1, 2, 4, 5, 10, 11, 20, 22, 44, 55, 110 なので d(220) = 284 である.
//! また, 284 の約数は 1, 2, 4, 71, 142 なので d(284) = 220 である.
//!
//! それでは10000未満の友愛数の和を求めよ.

use euler_lib::get_divisors;

fn main() {
    println!(
        "{}",
        (2..10000)
            .filter_map(|n| {
                let div_sum = get_divisors(&n).iter().sum();
                let div_sum_rev = get_divisors(&div_sum).iter().sum();

                if n != div_sum && n == div_sum_rev {
                    Some(n)
                } else {
                    None
                }
            })
            .sum::<usize>()
    );
}
