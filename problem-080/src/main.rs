//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2080
//!
//! よく知られているように, 自然数の平方根が整数ではないならば, それは無理数である.
//!
//! そのような平方根の10進展開(decimal expansion)は繰り返しを持たない無限小数になる.
//!
//! 2の平方根は, 1.41421356237309504880...,であり, その頭から100桁の数字を合計すると475になる.
//!
//! 初めの100個の自然数の平方根のうち, 無理数について, それぞれの頭から100桁の数字を足した数の総和を求めよ.

use num::{zero, BigUint};

fn main() {
    let ten = BigUint::from(10_u32);
    let one_hand = BigUint::from(100_u32);

    let mut sum = 0;

    for n in 2..100_u32 {
        match n {
            4 | 9 | 16 | 25 | 36 | 49 | 64 | 81 => continue,
            _ => (),
        }

        let mut z = BigUint::from(n);
        let mut r;
        let mut q = zero::<BigUint>();

        for _ in 0..100 {
            let a = (0..=9_u32)
                .take_while(|a| (&ten * &q + a) * a <= z)
                .last()
                .unwrap();

            r = (&ten * &q + a) * a;
            q = (&ten * &q + a) + a;
            z = &one_hand * (&z - &r);

            sum += a;
        }
    }

    println!("{}", sum);
}
