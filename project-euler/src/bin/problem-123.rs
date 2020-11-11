//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%20123
//!
//! pn を n 番目の素数とする. (p1 = 2, p2 = 3, ...) r を (pn - 1)n + (pn + 1)n を pn2 で割った余りとする.
//!
//! 例えば, n = 3 のとき, p3 = 5 であり, 43 + 63 = 280 ≡ 5 mod 25.
//!
//! 余り r が 109 より大きくなる n の最小値は 7037 である.
//!
//! 余り r が 1010 より大きくなる最初の n を求めよ.

use euler_lib::Prime;

fn main() {
    let mut prime = Prime::<usize>::new();
    let mut prime_iter = prime.iter().enumerate().skip(7037).map(|(n, p)| (n + 1, p));

    let bound = 10_usize.pow(10);

    println!(
        "{}",
        prime_iter
            .find(|&(n, p)| 2 * (n + 1) * p > bound)
            .unwrap()
            .0
            + 1
    );
}
