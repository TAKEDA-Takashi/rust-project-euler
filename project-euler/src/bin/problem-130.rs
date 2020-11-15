//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%20130
//!
//! 1からのみなる数をレピュニットと呼ぶ. R(k)で長さkのレピュニットを表す. 例えばR(6) = 111111である.
//!
//! GCD(n, 10) = 1 となる正整数 n について, 必ず正整数 k が存在し n が R(k) を割り切ることが証明できる. A(n) でそのような最小の k を表す. 例: A(7) = 6. A(41) = 5.
//!
//! 5より大きい素数 p について, A(p) が p - 1 を割り切ることが知られている. p = 41 のときには, A(41) = 5 であり, 40は5で割り切れる.
//!
//! 非常に少ないのだが, 合成数においても上が成立する場合がある. 最初の5つの例は 91, 259, 451, 481, 703 である.
//!
//! GCD(n, 10) = 1 かつ A(n) が n - 1 を割り切るような最初の25個の合成数 n の総和を求めよ.

use euler_lib::Prime;
use num::{BigUint, Integer, Zero};

fn main() {
    let prime = Prime::<usize>::new();

    let sum = (91_usize..)
        .filter(|n| n.gcd(&10) == 1 && *n % 3 != 0 && !prime.is_prime(n))
        .filter_map(|n| {
            let mut t = BigUint::from(111_u32);

            loop {
                if (&t % n).is_zero() {
                    if (n - 1) % t.to_string().len() == 0 {
                        return Some(n);
                    }
                    return None;
                }

                t = t * 10_u32 + 1_u32;
            }
        })
        .take(25)
        .sum::<usize>();

    println!("{}", sum);
}
