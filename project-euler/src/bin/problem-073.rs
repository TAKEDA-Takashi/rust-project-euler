//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2073
//!
//! nとdを正の整数として, 分数 n/d を考えよう. n<d かつ HCF(n,d)=1 のとき, 真既約分数と呼ぶ.
//!
//! d ≤ 8 について既約分数を大きさ順に並べると, 以下を得る:
//!
//! 1/8, 1/7, 1/6, 1/5, 1/4, 2/7, 1/3, 3/8, 2/5, 3/7, 1/2, 4/7, 3/5, 5/8, 2/3, 5/7, 3/4, 4/5, 5/6, 6/7, 7/8
//! 1/3と1/2の間には3つの分数が存在することが分かる.
//!
//! では, d ≤ 12,000 について真既約分数をソートした集合では, 1/3 と 1/2 の間に何個の分数があるか?

use num::Integer;

fn main() {
    let count: usize = (5..=12000)
        .map(|d| {
            (((d as f64 / 3.0).ceil() as usize)..=(d / 2))
                .filter(|n| n.gcd(&d) == 1)
                .count()
        })
        .sum();

    // println!("{:?}", max);
    println!("{}", count);
}
