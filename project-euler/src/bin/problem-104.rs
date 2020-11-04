//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%20104
//!
//! フィボナッチ数列は再帰的な関係によって定義される:
//!
//! Fn = Fn-1 + Fn-2
//! F541 (113桁)は, 下9桁に1から9までの数字をすべて含む初めてのフィボナッチ数である. そして, F2749 (575桁)は, 頭から9桁に1から9までの数字をすべて含む初めてのフィボナッチ数である.
//!
//! Fkが, 頭から9桁と下9桁のどちらも1から9までの数字をすべて含む初めてのフィボナッチ数とするとき, kを求めよ.

use euler_lib::{fibo_iter, is_pandigital};
use num::BigUint;

fn main() {
    let modulo = 10_u64.pow(9);
    let (n, _) = fibo_iter::<BigUint>()
        .enumerate()
        .skip(2749)
        .filter(|(_, f)| is_pandigital(&(f % modulo).to_string()))
        .find(|(_, f)| is_pandigital(&f.to_string()[0..9]))
        .unwrap();

    println!("{}", n + 1);
}
