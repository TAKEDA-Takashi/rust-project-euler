//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2082
//!
//! 注: この問題はProblem 81よりも挑戦しがいがあるだろう.
//!
//! 下記の5次の正方行列で, 一番左の列の任意のセルから開始し一番右の列の任意のセルで終わる道を探索する. ただし上下右にのみ移動できるものとする. 一番小さなパスは下で赤の太字で示されたものである. このときの合計は994になる.
//!
//! 131	673	234	103	18
//! 201	96	342	965	150
//! 630	803	746	422	111
//! 537	699	497	121	956
//! 805	732	524	37	331
//! 今, 31Kのテキストファイルmatrix.txtには80×80の行列が書かれている. 一番左の列から一番右の列へ移動する際の一番小さなパスの和を求めよ.

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::repeat;

fn main() {
    let mut matrix = matrix();
    let row_size = matrix[0].len();
    matrix.insert(0, repeat((usize::MAX, usize::MAX)).take(row_size).collect());

    for j in 1..row_size - 1 {
        for i in 1..matrix.len() {
            if matrix[i - 1][j].1 < matrix[i][j - 1].1 {
                matrix[i][j].1 += matrix[i - 1][j].1;
            } else {
                matrix[i][j].1 += matrix[i][j - 1].1;

                for i in (1..i).rev() {
                    let new_cost = matrix[i][j].0 + matrix[i + 1][j].1;
                    if new_cost < matrix[i][j].1 {
                        matrix[i][j].1 = new_cost;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    let mut min = usize::MAX;
    for i in 1..matrix.len() {
        matrix[i][row_size - 1].1 += matrix[i][row_size - 2].1;
        min = min.min(matrix[i][row_size - 1].1);
    }

    println!("{}", min);
    // matrix.iter().for_each(|v| println!("{:?}", v));
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
    // .map(|row| row.into_iter().map(|n| (n, n)).collect())
    // .collect()

    BufReader::new(File::open("files/p082_matrix.txt").unwrap())
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            line.split(",")
                .filter_map(|s| s.parse::<usize>().ok())
                .collect::<Vec<_>>()
        })
        .into_iter()
        .map(|row| row.into_iter().map(|n| (n, n)).collect())
        .collect()
}
