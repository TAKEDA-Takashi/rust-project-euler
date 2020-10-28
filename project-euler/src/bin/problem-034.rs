//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2034
//!
//! 145は面白い数である. 1! + 4! + 5! = 1 + 24 + 120 = 145となる.
//!
//! 各桁の数の階乗の和が自分自身と一致するような数の和を求めよ.
//!
//! 注: 1! = 1 と 2! = 2 は総和に含めてはならない.

fn main() {
    let fac = get_factorials(9);
    println!(
        "{}",
        (10..2_200_000_usize) // 9!*6=2,177,280なのでこの辺が上界
            .filter(|&n| {
                n.to_string()
                    .chars()
                    .filter_map(|c| c.to_digit(10).map(|d| d as usize))
                    .map(|m| fac[m])
                    .sum::<usize>()
                    == n
            })
            .sum::<usize>()
    );
}

fn get_factorials(u: usize) -> Vec<usize> {
    let mut v = vec![1];
    for n in 1..=u {
        v.push(n * v[n - 1]);
    }
    v
}
