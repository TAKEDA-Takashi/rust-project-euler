//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%20136
//!
//! x, y, z を等差数列となるような正の整数とする. 正の整数 n が n = 20 と与えられたときに, 方程式 x2 - y2 - z2 = n は唯一つの解を持つ.
//!
//! 132 - 102 - 72 = 20
//!
//! 実のところ100未満のnについて方程式が唯一つの解を持つようなnは25個存在する.
//!
//! 5000万未満のnについて方程式が唯一つの解を持つようなnは何個存在するか?

use euler_lib::Prime;

fn main() {
    let ubound = 50_000_000;
    let prime = Prime::<usize>::new();

    let sum = prime.iter().skip(1).take_while(|&p| p < ubound).fold(
        2, /* 4 and 16 */
        |mut sum, p| {
            if (p + 1) % 4 == 0 {
                sum += 1;
            }
            if p * 4 < ubound {
                sum += 1;
                if p * 16 < ubound {
                    sum += 1;
                }
            }

            sum
        },
    );

    println!("{}", sum);
}
