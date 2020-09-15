//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%207
//!
//! 素数を小さい方から6つ並べると 2, 3, 5, 7, 11, 13 であり, 6番目の素数は 13 である.
//! 10 001 番目の素数を求めよ.

use problem_007::Prime;

fn main() {
    println!("{}", Prime::new().nth(10000).unwrap()); // 0-origin
}
