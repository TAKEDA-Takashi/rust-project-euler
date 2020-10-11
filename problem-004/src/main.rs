//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%204
//!
//! 左右どちらから読んでも同じ値になる数を回文数という. 2桁の数の積で表される回文数のうち, 最大のものは 9009 = 91 × 99 である.
//! では, 3桁の数の積で表される回文数の最大値を求めよ.

use euler_lib::mul_palindrome;
use itertools::Itertools;

fn main() {
    let max_palindrome = (100..=999)
        .combinations(2)
        .filter_map(|v| mul_palindrome(&(v[0], v[1])))
        .sorted_by(|(.., p1), (.., p2)| Ord::cmp(p2, p1)) // order by desc
        .next()
        .unwrap();

    // println!("{:?}", max_palindrome);
    println!("{}", max_palindrome.2);
}
