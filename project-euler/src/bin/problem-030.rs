//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2030
//!
//! 驚くべきことに, 各桁を4乗した数の和が元の数と一致する数は3つしかない.
//!
//! 1634 = 14 + 64 + 34 + 44
//! 8208 = 84 + 24 + 04 + 84
//! 9474 = 94 + 44 + 74 + 44
//! ただし, 1=14は含まないものとする. この数たちの和は 1634 + 8208 + 9474 = 19316 である.
//!
//! 各桁を5乗した数の和が元の数と一致するような数の総和を求めよ.

fn main() {
    println!(
        "{}",
        (10..299_999) // 9^5*5が295,245なのでこの辺が上界
            .filter(|&n| n
                .to_string()
                .chars()
                .filter_map(|c| c.to_digit(10))
                .map(|m| m.pow(5))
                .sum::<u32>()
                == n)
            .sum::<u32>()
    );
}