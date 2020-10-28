//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2025
//!
//! フィボナッチ数列は以下の漸化式で定義される:
//!
//! Fn = Fn-1 + Fn-2, ただし F1 = 1, F2 = 1.
//! 最初の12項は以下である.
//!
//! F1 = 1
//! F2 = 1
//! F3 = 2
//! F4 = 3
//! F5 = 5
//! F6 = 8
//! F7 = 13
//! F8 = 21
//! F9 = 34
//! F10 = 55
//! F11 = 89
//! F12 = 144
//! 12番目の項, F12が3桁になる最初の項である.
//!
//! 1000桁になる最初の項の番号を答えよ.

use euler_lib::fibo_iter;
use num::BigUint;

fn main() {
    println!(
        "{}",
        fibo_iter::<BigUint>()
            .enumerate()
            .find(|(_, f)| f.to_string().len() == 1000)
            .unwrap()
            .0
            + 1 // 0-origin
    );
}
