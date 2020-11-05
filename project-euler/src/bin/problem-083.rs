//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2083
//!
//! 注: この問題はProblem 81よりも非常に挑戦しがいがあるだろう.
//!
//! 下記の5次の正方行列で, 上下左右に移動し左上のセルから開始し右下のセルで終了する道を探索する. 一番小さな道は下で赤で示されており, このときの合計は2297になる.
//!
//! 131	673	234	103	18
//! 201	96	342	965	150
//! 630	803	746	422	111
//! 537	699	497	121	956
//! 805	732	524	37	331
//! 今, 31Kのテキストファイルmatrix.txtには80×80の行列が書かれている. 上下左右に移動し左上のセルから開始し右下のセルで終了する道に沿った和の最小を求めよ.

// using Dijkstra's algorithm

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::repeat;

fn main() {
    let (mut matrix, mut done_table) = init();
    let mut candidate = BinaryHeap::new();

    matrix[1][1].1 = matrix[1][1].0;
    candidate.push(Reverse((matrix[1][1].1, 1, 1)));

    while let Some(Reverse((cost, i, j))) = candidate.pop() {
        if done_table[i][j] {
            continue;
        }

        done_table[i][j] = true;

        vec![(i, j - 1), (i - 1, j), (i, j + 1), (i + 1, j)]
            .into_iter()
            .filter(|&(x, y)| !done_table[x][y])
            .for_each(|(x, y)| {
                let new_cost = matrix[x][y].0 + cost;
                if new_cost < matrix[x][y].1 {
                    matrix[x][y].1 = new_cost;
                    candidate.push(Reverse((matrix[x][y].1, x, y)));
                }
            });
    }

    // matrix.iter().for_each(|v| println!("{:?}", v));
    println!("{:?}", matrix[matrix.len() - 2][matrix[0].len() - 2]);
}

fn init() -> (Vec<Vec<(usize, usize)>>, Vec<Vec<bool>>) {
    let mut matrix = matrix();

    let mut done_table: Vec<Vec<bool>> = (0..matrix.len())
        .map(|_| repeat(false).take(matrix[0].len()).collect())
        .collect();

    matrix.insert(
        0,
        repeat((usize::MAX, usize::MAX))
            .take(matrix[0].len())
            .collect(),
    );

    matrix.push(
        repeat((usize::MAX, usize::MAX))
            .take(matrix[0].len())
            .collect(),
    );

    for i in 0..matrix.len() {
        matrix[i].insert(0, (usize::MAX, usize::MAX));
        matrix[i].push((usize::MAX, usize::MAX));
    }

    done_table.insert(0, repeat(true).take(done_table[0].len()).collect());
    done_table.push(repeat(true).take(done_table[0].len()).collect());

    for i in 0..done_table.len() {
        done_table[i].insert(0, true);
        done_table[i].push(true);
    }

    (matrix, done_table)
}

fn matrix() -> Vec<Vec<(usize, usize)>> {
    // vec![
    //     vec![131, 673, 234, 103, 18],
    //     vec![201, 96, 342, 965, 150],
    //     vec![630, 803, 746, 422, 111],
    //     vec![537, 699, 497, 121, 956],
    //     vec![805, 732, 524, 37, 331],
    // ]
    // .into_iter()
    // .map(|row| row.into_iter().map(|n| (n, usize::MAX)).collect())
    // .collect()

    BufReader::new(File::open("files/p083_matrix.txt").unwrap())
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            line.split(",")
                .filter_map(|s| s.parse::<usize>().ok())
                .collect::<Vec<_>>()
        })
        .into_iter()
        .map(|row| row.into_iter().map(|n| (n, usize::MAX)).collect())
        .collect()
}
