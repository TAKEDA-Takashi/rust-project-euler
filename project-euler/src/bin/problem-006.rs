//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%206
//!
//! 最初の10個の自然数について, その二乗の和は,
//!
//! 12 + 22 + ... + 102 = 385
//! 最初の10個の自然数について, その和の二乗は,
//!
//! (1 + 2 + ... + 10)2 = 3025
//! これらの数の差は 3025 - 385 = 2640 となる.
//!
//! 同様にして, 最初の100個の自然数について二乗の和と和の二乗の差を求めよ.

fn main() {
    let num = 100;

    let pow_sum = (1..=num).map(|n| n * n).sum::<i32>();
    let sum_pow = ((1..=num).sum::<i32>()).pow(2);

    println!("{}", sum_pow - pow_sum);
}
