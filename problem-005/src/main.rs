//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%205
//!
//! 2520 は 1 から 10 の数字の全ての整数で割り切れる数字であり, そのような数字の中では最小の値である.
//! では, 1 から 20 までの整数全てで割り切れる数字の中で最小の正の数はいくらになるか.

fn main() {
    println!("{}", (2..=20).fold(1, |acc, n| acc * n / gcd(acc, n)));
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
