///! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%204
///!
///! 左右どちらから読んでも同じ値になる数を回文数という. 2桁の数の積で表される回文数のうち, 最大のものは 9009 = 91 × 99 である.
///! では, 3桁の数の積で表される回文数の最大値を求めよ.
use std::iter::repeat;

use itertools::Itertools;

fn is_palindrome(n: i32) -> bool {
    let s = n.to_string();
    s == s.chars().rev().collect::<String>()
}

fn main() {
    let max_palindrome = (100..=999)
        .flat_map(|n| repeat(n).zip(n..=999))
        .map(|p| (p.0, p.1, p.0 * p.1))
        .filter(|t| is_palindrome(t.2))
        .sorted_by(|t1, t2| Ord::cmp(&t2.2, &t1.2)) // order by desc
        .next()
        .unwrap();

    // println!("{:?}", max_palindrome);
    println!("{}", max_palindrome.2);
}
