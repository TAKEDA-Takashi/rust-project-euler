//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2063
//!
//! 5桁の数 16807 = 75は自然数を5乗した数である. 同様に9桁の数 134217728 = 89も自然数を9乗した数である.
//!
//! 自然数を n 乗して得られる n 桁の正整数は何個あるか?

use num::{one, range_inclusive, BigUint};

fn main() {
    println!(
        "{}",
        (1..22)
            .flat_map(|e| range_inclusive(one(), BigUint::from(9_u32))
                .map(|base| base.pow(e))
                .filter(|n| n.to_string().len() == e as usize)
                .collect::<Vec<_>>())
            .count()
    );
}
