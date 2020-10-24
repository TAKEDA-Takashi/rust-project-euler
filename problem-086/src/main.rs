//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2086
//!
//! 下に示す直方体は寸法が6×5×3である. この直方体の1つの頂点Sにクモがいる. また反対の頂点Fにはハエがいる. SからFまでの壁に沿って直線移動する最短ルートは図に示す通りで, この長さは10である.
//!
//! p_086.gif
//! この最短ルートの候補は3本あるが, 最短の長さがいつも整数とは限らない.
//!
//! さて, M×M×M以下の寸法の直方体について, 最短ルートが整数である直方体の数を考える. M=100のとき, 条件を満たす直方体は2060個ある. このM=100は個数が2000を超える最小のMである. なお, M=99のときは1975個である.
//!
//! 100万個を超える最小のMを求めよ.

use itertools::Itertools;
use num_integer::Integer;
use std::iter::repeat;

fn main() {
    let criteria = 1_000_000;
    // binary search; 初めてcriteriaを超えるmidを探索する
    let mut low = 1000;
    let mut high = 2000;
    let mut mid = (low + high) / 2;

    while mid != low && mid + 1 != high {
        let count = count_shortest_route_with_integer(mid);

        if count < criteria {
            low = mid;
            mid = (low + high) / 2;
        } else {
            high = mid + 1;
            mid = (low + high) / 2;
        }
    }

    println!("{}", mid);
}

fn count_shortest_route_with_integer(side_max: usize) -> usize {
    let ubound = ((2 * side_max) as f64).sqrt() as usize;
    (2..ubound)
        .flat_map(|m| repeat(m).zip(1..m))
        .filter_map(|(m, n)| primitive_pythagorean_triple(m, n))
        .filter(|&(a, ..)| a <= side_max)
        .flat_map(|(a, b, _)| {
            (1..)
                .take_while(|&n| a * n <= side_max)
                .map(|n| (a * n, b * n))
                .collect_vec()
        })
        .flat_map(|(a, b)| {
            let b_iter = (1..=b / 2)
                .map(|n| (n, b - n))
                .filter(|&(_, y)| y <= a)
                .map(|(x, y)| vec![x, y, a]);

            if b <= side_max {
                (1..=a / 2)
                    .map(|n| vec![n, a - n, b])
                    .chain(b_iter)
                    .collect_vec()
            } else {
                b_iter.collect_vec()
            }
        })
        .count()
}

fn primitive_pythagorean_triple(m: usize, n: usize) -> Option<(usize, usize, usize)> {
    assert!(m > n);

    if (m - n) % 2 != 1 || m.gcd(&n) != 1 {
        return None;
    }

    let a = m * m - n * n;
    let b = 2 * m * n;
    let c = m * m + n * n;

    if a < b {
        Some((a, b, c))
    } else {
        Some((b, a, c))
    }
}
