//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%205
//!
//! 2520 は 1 から 10 の数字の全ての整数で割り切れる数字であり, そのような数字の中では最小の値である.
//! では, 1 から 20 までの整数全てで割り切れる数字の中で最小の正の数はいくらになるか.

use num_integer::Integer;

fn main() {
    println!("{}", (2..=20_u64).fold(1, |acc, n| acc * n / acc.gcd(&n)));
}
