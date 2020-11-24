//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%20141
//!
//! 正の整数nをdで割った商と余りをそれぞれqとrで表す. d, q, rを適当に並び替えたときに正の項からなる等比数列（幾何数列）になる場合がある.
//!
//! 例えば58を6で割ると商が9で余りが4である. 4, 6, 9は公比3/2の幾何数列になっている. 以下, このような n を累進数と呼ぶ. (訳者注: progressive numberの定訳が分からないので適当な名前にしておく.)
//!
//! いくつかの累進数 9や10404=1022は平方数になっている. 100000未満の累進平方数の和は124657である.
//!
//! 1012未満の累進平方数の総和を答えよ.

use num::integer::Roots;
use std::collections::HashSet;

fn main() {
    let ubound = 10_u128.pow(12);

    let set: HashSet<_> = (2..ubound.cbrt())
        .flat_map(|m| {
            let m3 = m * m * m;
            (1..m)
                .take_while(|&n| n * (m3 + n) <= ubound)
                .flat_map(|n| {
                    (1..ubound)
                        .map(move |k| k * n * (k * m3 + n))
                        .take_while(|&res| res <= ubound)
                        .filter(|&res| {
                            let rt = res.sqrt();
                            rt * rt == res
                        })
                })
                .collect::<HashSet<_>>()
        })
        .collect();

    println!("{}", set.iter().sum::<u128>());
}
