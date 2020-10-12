//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2036
//!
//! 585 = 10010010012 (2進) は10進でも2進でも回文数である.
//!
//! 100万未満で10進でも2進でも回文数になるような数の総和を求めよ.
//!
//! (注: 先頭に0を含めて回文にすることは許されない.)

use euler_lib::{is_binary_palindrome, is_palindrome};

fn main() {
    println!(
        "{}",
        (1..1_000_000)
            .filter(|n| is_palindrome(n) && is_binary_palindrome(n))
            .sum::<usize>()
    );
}
