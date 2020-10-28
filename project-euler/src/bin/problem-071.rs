//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2071
//!
//! nとdを正の整数として, 分数 n/d を考えよう. n<d かつ HCF(n,d)=1 のとき, 真既約分数と呼ぶ.
//!
//! d ≤ 8について既約分数を大きさ順に並べると, 以下を得る:
//!
//! 1/8, 1/7, 1/6, 1/5, 1/4, 2/7, 1/3, 3/8, 2/5, 3/7, 1/2, 4/7, 3/5, 5/8, 2/3, 5/7, 3/4, 4/5, 5/6, 6/7, 7/8
//! 3/7のすぐ左の分数は2/5である.
//!
//! d ≤ 1,000,000について真既約分数を大きさ順に並べたとき, 3/7のすぐ左の分数の分子を求めよ.

use num::{Integer, Rational};

fn main() {
    let criteria = Rational::new(3, 7);

    let max = (2..=1_000_000).fold(Rational::new(0, 1), |max, d| {
        if let Some(m) = (max.numer() + 1..)
            .filter(|n| n.gcd(&d) == 1)
            .map(|n| Rational::new(n, d))
            .take_while(|&r| r < criteria)
            .last()
        {
            if max < m {
                return m;
            }
        }
        max
    });

    // println!("{:?}", max);
    println!("{}", max.numer());
}
