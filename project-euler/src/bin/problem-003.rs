//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%203
//!
//! 13195 の素因数は 5, 7, 13, 29 である.
//! 600851475143 の素因数のうち最大のものを求めよ.

use euler_lib::Prime;

fn main() {
    let mut prime = Prime::new();

    // println!("{:?}", prime.factorization(&600851475143_u64));
    println!("{}", prime.factorization(&600851475143_u64).last().unwrap());
}
