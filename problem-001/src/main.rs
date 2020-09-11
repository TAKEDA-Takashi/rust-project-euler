//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%201
//!
//! 10未満の自然数のうち, 3 もしくは 5 の倍数になっているものは 3, 5, 6, 9 の4つがあり, これらの合計は 23 になる.
//! 同じようにして, 1000 未満の 3 か 5 の倍数になっている数字の合計を求めよ.

fn main() {
    println!(
        "{:?}",
        (1..1000)
            .filter(|n| { n % 3 == 0 || n % 5 == 0 })
            .sum::<usize>()
    )
}
