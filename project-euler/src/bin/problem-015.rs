//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2015
//!
//! 2×2 のマス目の左上からスタートした場合, 引き返しなしで右下にいくルートは 6 つある.
//!
//! では, 20×20 のマス目ではいくつのルートがあるか.

use euler_lib::combination;

fn main() {
    println!("{}", combination(&40_u64, &20));
}
