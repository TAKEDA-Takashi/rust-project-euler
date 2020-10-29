//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2097
//!
//! 100万桁を超える初めての素数は1999年に発見された. これはメルセンヌ素数であり, 26972593-1 である. 実際, 2,098,960桁ある. それ以降も, より多くの桁になるメルセンヌ素数 (2p-1の形の数) が他にも発見されている.
//!
//! しかし, 2004年に, 非常に大きな非メルセンヌ素数が発見された. これは2,357,207桁の数であり, 28433×27830457+1である.
//!
//! この素数の末尾10桁を答えよ.

use num::BigUint;

fn main() {
    let modulo = BigUint::from(10_u32).pow(10);
    let mut n = BigUint::from(2_u32);
    n = n.modpow(&BigUint::from(7830457_u32), &modulo);
    println!("{}", (28433_u32 * n + 1_u32) % modulo);
}
