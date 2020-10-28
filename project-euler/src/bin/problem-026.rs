//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2026
//!
//! 単位分数とは分子が1の分数である. 分母が2から10の単位分数を10進数で表記すると次のようになる.
//!
//! 1/2 = 0.5
//! 1/3 = 0.(3)
//! 1/4 = 0.25
//! 1/5 = 0.2
//! 1/6 = 0.1(6)
//! 1/7 = 0.(142857)
//! 1/8 = 0.125
//! 1/9 = 0.(1)
//! 1/10 = 0.1
//!
//! 0.1(6)は 0.166666... という数字であり, 1桁の循環節を持つ. 1/7 の循環節は6桁ある.
//!
//! d < 1000 なる 1/d の中で小数部の循環節が最も長くなるような d を求めよ.

use euler_lib::Prime;
use lazy_static::lazy_static;
use num::BigUint;

lazy_static! {
    static ref ZERO: BigUint = BigUint::from(0_u32);
    static ref ONE: BigUint = BigUint::from(1_u32);
    static ref TEN: BigUint = BigUint::from(10_u32);
}

fn main() {
    let ps: Vec<BigUint> = Prime::<BigUint>::new()
        .iter()
        .skip(3) // 2と5は循環小数にならない。3はついで
        .take_while(|p| *p < BigUint::from(1000_u32))
        .collect();

    println!("{}", find_max_cycle(ps).unwrap());
}

fn find_max_cycle(ps: Vec<BigUint>) -> Option<BigUint> {
    fn find_max_cycle0(ps: Vec<BigUint>, one_seq: BigUint) -> Option<BigUint> {
        match ps.len() {
            0 => None,
            1 => Some(ps[0].clone()),
            _ => find_max_cycle0(
                ps.into_iter().filter(|p| &one_seq % p != *ZERO).collect(),
                one_seq * &*TEN + &*ONE,
            ),
        }
    }

    find_max_cycle0(ps, BigUint::from(11_u32))
}
