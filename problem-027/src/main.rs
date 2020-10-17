//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2027
//!
//! オイラーは以下の二次式を考案している:
//!
//! n2 + n + 41.
//! この式は, n を0から39までの連続する整数としたときに40個の素数を生成する. しかし, n = 40 のとき 402 + 40 + 41 = 40(40 + 1) + 41 となり41で割り切れる. また, n = 41 のときは 412 + 41 + 41 であり明らかに41で割り切れる.
//!
//! 計算機を用いて, 二次式 n2 - 79n + 1601 という式が発見できた. これは n = 0 から 79 の連続する整数で80個の素数を生成する. 係数の積は, -79 × 1601 で -126479である.
//!
//! さて, |a| < 1000, |b| ≤ 1000 として以下の二次式を考える (ここで |a| は絶対値): 例えば |11| = 11 |-4| = 4である.
//!
//! n2 + an + b
//! n = 0 から始めて連続する整数で素数を生成したときに最長の長さとなる上の二次式の, 係数 a, b の積を答えよ.

use euler_lib::Prime;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref PRIME: Mutex<Prime<isize>> = Mutex::new(Prime::new());
}

fn main() {
    let (a, b) = find_longest_prime_gen(get_quadratic_function()).unwrap();
    println!("{}", a * b);
}

fn find_longest_prime_gen(
    v: Vec<(Box<dyn Fn(isize) -> isize>, isize, isize)>,
) -> Option<(isize, isize)> {
    fn find_longest_prime_gen0(
        v: Vec<(Box<dyn Fn(isize) -> isize>, isize, isize)>,
        n: isize,
    ) -> Option<(isize, isize)> {
        match v.len() {
            0 => None,
            1 => Some((v[0].1, v[0].2)),
            _ => find_longest_prime_gen0(
                v.into_iter()
                    .filter(|(f, ..)| PRIME.lock().unwrap().is_prime(&f.as_ref()(n)))
                    .collect(),
                n + 1,
            ),
        }
    }

    find_longest_prime_gen0(v, 0)
}

fn get_quadratic_function() -> Vec<(Box<dyn Fn(isize) -> isize>, isize, isize)> {
    let mut v: Vec<(Box<dyn Fn(isize) -> isize>, isize, isize)> = vec![];
    // a, b は共に奇数
    for a in (-999..999).step_by(2) {
        for b in (-999..999).step_by(2) {
            v.push((Box::new(move |n| n * n + a * n + b), a, b))
        }
    }

    v
}
