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

use euler_lib::{get_divisors, modpow, Prime};

fn main() {
    let d = Prime::<usize>::new()
        .iter()
        .take_while(|&p| p < 1000)
        .collect::<Vec<_>>() // 逆順で取り出すために一度Vecにしている
        .into_iter()
        .rev()
        .find(|&p| {
            get_divisors(&(p - 1))
                .iter()
                .skip(1)
                .all(|&m| modpow(10, m, p) != 1)
        });

    println!("{:?}", d)
}
