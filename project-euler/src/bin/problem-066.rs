//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2066
//!
//! 次の形式の, 2次のディオファントス方程式を考えよう:
//!
//! x2 - Dy2 = 1
//! たとえば D=13 のとき, x を最小にする解は 6492 - 13×1802 = 1 である.
//!
//! D が平方数(square)のとき, 正整数のなかに解は存在しないと考えられる.
//!
//! D = {2, 3, 5, 6, 7} に対して x を最小にする解は次のようになる:
//!
//! 32 - 2×22 = 1
//! 22 - 3×12 = 1
//! 92 - 5×42 = 1
//! 52 - 6×22 = 1
//! 82 - 7×32 = 1
//! したがって, D ≤ 7 に対して x を最小にする解を考えると, D=5 のとき x は最大である.
//!
//! D ≤ 1000 に対する x を最小にする解で, x が最大になるような D の値を見つけよ.

// https://ja.wikipedia.org/wiki/%E3%83%9A%E3%83%AB%E6%96%B9%E7%A8%8B%E5%BC%8F

use num::{one, zero, BigInt, BigRational};

fn main() {
    println!(
        "{}",
        (2..=1000)
            .map(|n| (n, (n as f64).sqrt(), vec![]))
            .filter_map(|(n, s, v)| (s.floor() != s).then(|| (n, s as i32, v)))
            .map(|(n, z, mut v)| {
                let mut b = -z;
                let mut d = 1;

                while d != 1 || v.is_empty() {
                    d = (n - b * b) / d;
                    let a = (z + b.abs()) / d;
                    b = (z + b.abs()) % d - z;

                    v.push(a);
                }

                (n, z, v)
            })
            .map(|(n, a, v)| (
                n,
                BigRational::from_integer(BigInt::from(a))
                    + v.iter().rev().skip(1).fold(
                        zero::<BigRational>(),
                        |acc, n| one::<BigRational>()
                            / (acc + BigRational::from_integer(BigInt::from(*n)))
                    ),
                v.len() % 2 == 0
            ))
            .map(|(n, ratio, is_even)| if is_even {
                (n, ratio.numer().clone(), ratio.denom().clone())
            } else {
                (
                    n,
                    ratio.numer() * ratio.numer() + BigInt::from(n) * ratio.denom() * ratio.denom(),
                    BigInt::from(2) * ratio.numer() * ratio.denom(),
                )
            })
            .max_by_key(|(_, x, _)| x.clone())
            .unwrap()
            .0
    );
}
