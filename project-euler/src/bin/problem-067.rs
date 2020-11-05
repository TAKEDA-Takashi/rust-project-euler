//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2067
//!
//! 以下の三角形の頂点から下まで移動するとき, その数値の合計の最大値は23になる.
//!
//! 3
//! 7 4
//! 2 4 6
//! 8 5 9 3
//! この例では 3 + 7 + 4 + 9 = 23
//!
//! 100列の三角形を含んでいる15Kのテキストファイル triangle.txt (右クリックして, 『名前をつけてリンク先を保存』)の上から下まで最大合計を見つけよ.
//!
//! 注：これは, Problem 18のずっと難しいバージョンです.
//! 全部で299 通りの組み合わせがあるので, この問題を解決するためにすべてのルートをためすことは可能でありません！
//! あなたが毎秒1兆本の(1012)ルートをチェックすることができたとしても, 全てをチェックするために200億年以上かかるでしょう.
//! 解決するための効率的なアルゴリズムがあります. ;o)

#![allow(dead_code)]

use std::cmp;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut triangle = load_triangle();

    for row_idx in (0..triangle.len() - 1).rev() {
        for col_idx in 0..triangle[row_idx].len() {
            let left_weight = triangle[row_idx + 1][col_idx].get_total_cost();
            let right_weight = triangle[row_idx + 1][col_idx + 1].get_total_cost();

            let mut node = &mut triangle[row_idx][col_idx];
            node.left_weight = Some(left_weight);
            node.right_weight = Some(right_weight);
        }
    }

    // println!("{:?}", triangle);

    // print_max_route(&triangle);

    println!("{}", triangle[0][0].get_total_cost());
}

#[derive(Debug)]
struct Node {
    cost: usize,

    left_weight: Option<usize>,
    right_weight: Option<usize>,
}

impl Node {
    fn new(cost: usize) -> Node {
        Node {
            cost: cost,
            left_weight: None,
            right_weight: None,
        }
    }

    fn get_total_cost(&self) -> usize {
        self.cost
            + cmp::max(
                self.left_weight.unwrap_or(0),
                self.right_weight.unwrap_or(0),
            )
    }
}

fn load_triangle() -> Vec<Vec<Node>> {
    BufReader::new(File::open("files/p067_triangle.txt").unwrap())
        .lines()
        .filter_map(|result| result.ok())
        .map(|line| {
            line.split_ascii_whitespace()
                .filter_map(|s| s.parse::<usize>().ok())
                .map(|n| Node::new(n))
                .collect::<Vec<Node>>()
        })
        .collect()
}

fn print_max_route(graph: &Vec<Vec<Node>>) {
    fn print_max_route0(graph: &Vec<Vec<Node>>, row: usize, col: usize) {
        let node = &graph[row][col];

        print!("{} ", node.cost);

        if let Some(lw) = node.left_weight {
            if let Some(rw) = node.right_weight {
                if lw >= rw {
                    print_max_route0(graph, row + 1, col);
                } else {
                    print_max_route0(graph, row + 1, col + 1);
                }
            } else {
                print_max_route0(graph, row + 1, col);
            }
        }
    }

    print_max_route0(graph, 0, 0);
    println!();
}
