//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%20112
//!
//! 左から右までどの桁もその左の桁を下回らない数を増加数と呼ぶ. 例えば, 134468.
//!
//! 同様に, どの桁もその右の桁を下回らない数を減少数と呼ぶ. 例えば, 66420.
//!
//! 増加数でも減少数でもない正の整数をはずみ数と呼ぶことにする. 例えば, 155349.
//!
//! 100以下にはずみ数が無いのは明らかだが, 1000未満では半数を少し上回る525個がはずみ数である.
//!
//! 実際, はずみ数の割合が50%に達する最少の数は538である.
//!
//! 驚くべきことに, はずみ数はますます一般的になり, 21780でははずみ数の割合は90%に達する.
//!
//! はずみ数の割合がちょうど99%になる最小の数を求めよ.

use itertools::Itertools;

fn main() {
    let mut not_bouncy = 0;

    for n in 1.. {
        if !is_bouncy_number(n) {
            not_bouncy += 1;
        }

        if n == not_bouncy * 100 {
            println!("{}", n);
            break;
        }
    }
}

fn is_bouncy_number(n: usize) -> bool {
    let ns = n.to_string().chars().collect_vec();
    !is_increasing_number(&ns) && !is_decreasing_number(&ns)
}

fn is_increasing_number(ns: &Vec<char>) -> bool {
    ns.iter().tuple_windows().all(|(a, b)| a <= b)
}

fn is_decreasing_number(ns: &Vec<char>) -> bool {
    ns.iter().tuple_windows().all(|(a, b)| a >= b)
}
