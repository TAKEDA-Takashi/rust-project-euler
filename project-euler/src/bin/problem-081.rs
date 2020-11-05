//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2081
//!
//! 下記の5次の正方行列で, 左上のセルから開始し右下のセルで終わるパスを探索する. ただし下方向と右方向にのみ移動できるものとする.
//! 通過したセルの和が最小となるパスは赤の太字で示されたもので, その値は2427である.
//!
//! 131	673	234	103	18
//! 201	96	342	965	150
//! 630	803	746	422	111
//! 537	699	497	121	956
//! 805	732	524	37	331
//! 今, 31Kのテキストファイルmatrix.txt (右クリックして, 『名前をつけてリンク先を保存』)には80×80の行列が書かれている.
//! 同様に左上のセルから開始し右下のセルで終わり, かつ右方向と下方向にのみ移動するときの最小のパスの和を求めよ.

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::repeat;

fn main() {
    let mut matrix = matrix();
    matrix.insert(0, repeat(usize::MAX).take(matrix[0].len()).collect());

    for i in 0..matrix.len() {
        matrix[i].insert(0, usize::MAX);
    }

    matrix[0][1] = 0;

    for i in 1..matrix.len() {
        for j in 1..matrix[i].len() {
            matrix[i][j] += matrix[i - 1][j].min(matrix[i][j - 1]);
        }
    }

    println!("{}", matrix.last().unwrap().last().unwrap());
}

fn matrix() -> Vec<Vec<usize>> {
    // vec![
    //     vec![131, 673, 234, 103, 18],
    //     vec![201, 96, 342, 965, 150],
    //     vec![630, 803, 746, 422, 111],
    //     vec![537, 699, 497, 121, 956],
    //     vec![805, 732, 524, 37, 331],
    // ]

    BufReader::new(File::open("files/p081_matrix.txt").unwrap())
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            line.split(",")
                .filter_map(|s| s.parse::<usize>().ok())
                .collect()
        })
        .collect()
}
