//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2060
//!
//! 素数3, 7, 109, 673は非凡な性質を持っている. 任意の2つの素数を任意の順で繋げると, また素数になっている. 例えば, 7と109を用いると, 7109と1097の両方が素数である. これら4つの素数の和は792である. これは, このような性質をもつ4つの素数の集合の和の中で最小である.
//!
//! 任意の2つの素数を繋げたときに別の素数が生成される, 5つの素数の集合の和の中で最小のものを求めよ.

use euler_lib::Prime;
use itertools::Itertools;
use lazy_static::lazy_static;
use std::iter::{once, repeat};
use std::sync::Mutex;

lazy_static! {
    static ref PRIME: Mutex<Prime<usize>> = Mutex::new(Prime::<usize>::new());
}

fn main() {
    let upper_bound = 10_000;

    let prime_list = PRIME
        .lock()
        .unwrap()
        .iter()
        .take_while(|&p| p < upper_bound)
        .collect_vec();
    let v = find_prime_pair_sets(&prime_list, 5).unwrap();

    println!("{}", v.iter().sum::<usize>());
}

fn find_prime_pair_sets(prime_list: &Vec<usize>, find_len: usize) -> Option<Vec<usize>> {
    fn find0(prime_list: &Vec<usize>, find_len: usize, v: Vec<usize>) -> Option<Vec<usize>> {
        if v.len() == find_len {
            Some(v)
        } else {
            let maximum = *v.last().unwrap();
            repeat(v)
                .zip(prime_list.iter().skip_while(|&&p| p <= maximum))
                .map(|(v, n)| v.into_iter().chain(once(*n)).collect_vec())
                .filter(is_prime_pair)
                .filter_map(|v| find0(prime_list, find_len, v))
                .next()
        }
    }

    prime_list
        .iter()
        .filter_map(|&p| find0(prime_list, find_len, vec![p]))
        .next()
}

fn is_prime_pair(v: &Vec<usize>) -> bool {
    let p = *v.last().unwrap();

    (0..v.len() - 1).all(|i| {
        let mut prime = PRIME.lock().unwrap();
        prime.is_prime(&(v[i] * 10_usize.pow((p as f64).log10() as u32 + 1) + p))
            && prime.is_prime(&(p * 10_usize.pow((v[i] as f64).log10() as u32 + 1) + v[i]))
    })
}
