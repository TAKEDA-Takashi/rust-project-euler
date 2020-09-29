//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2010
//!
//! 10以下の素数の和は 2 + 3 + 5 + 7 = 17 である.
//! 200万以下の全ての素数の和を求めよ.

use problem_010::Prime;

fn main() {
    println!(
        "{}",
        Prime::new().take_while(|&p| p <= 2_000_000).sum::<usize>()
    );
}
