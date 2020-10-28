//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2040
//!
//! 正の整数を順に連結して得られる以下の10進の無理数を考える:
//!
//! 0.123456789101112131415161718192021...
//! 小数第12位は1である.
//!
//! dnで小数第n位の数を表す. d1 × d10 × d100 × d1000 × d10000 × d100000 × d1000000 を求めよ.

fn main() {
    let d = (1..)
        .try_fold(
            String::from("0"), /* index=0 is sentinel */
            |mut acc, n| {
                acc.push_str(&n.to_string());
                if acc.len() > 1_000_000 {
                    Err(acc) // break
                } else {
                    Ok(acc)
                }
            },
        )
        .unwrap_or_else(|s| s);

    println!(
        "{}",
        vec![1, 10, 100, 1_000, 10_000, 100_000, 1_000_000]
            .iter()
            .filter_map(|&n| d[n..n + 1].parse::<usize>().ok())
            .product::<usize>()
    );
}
