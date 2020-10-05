//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2045
//!
//! 三角数, 五角数, 六角数は以下のように生成される.
//!
//! 三角数	Tn=n(n+1)/2	1, 3, 6, 10, 15, ...
//! 五角数	Pn=n(3n-1)/2	1, 5, 12, 22, 35, ...
//! 六角数	Hn=n(2n-1)	1, 6, 15, 28, 45, ...
//! T285 = P165 = H143 = 40755であることが分かる.
//!
//! 次の三角数かつ五角数かつ六角数な数を求めよ.

// a ∈ 六角数 => a ∈ 三角数

use problem_045::Intersect;

fn main() {
    println!(
        "{}",
        Intersect::new(
            (1..).map(|n: usize| n * (3 * n - 1) / 2),
            (1..).map(|n: usize| n * (2 * n - 1)),
        )
        .skip_while(|&n| n <= 40755)
        .next()
        .unwrap()
    );
}
