//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%205
//!
//! 2520 は 1 から 10 の数字の全ての整数で割り切れる数字であり, そのような数字の中では最小の値である.
//! では, 1 から 20 までの整数全てで割り切れる数字の中で最小の正の数はいくらになるか.

fn main() {
    let p = (2..=20)
        .fold(vec![], |mut acc, n| {
            acc.push(
                acc.iter()
                    .fold(n, |d, a| if d % a == 0 { d / a } else { d }),
            );
            acc
        })
        .iter()
        .product::<i32>();
    println!("{}", p);
}
