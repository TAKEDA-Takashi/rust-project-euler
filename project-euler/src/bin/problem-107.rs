//! http://odz.sakura.ne.jp/projecteuler/index.php?Problem%20107
//! Minimal network

use itertools::Itertools;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::repeat;

fn main() {
    let graph = make_graph();

    let before_score = total_score(&graph);
    let after_score = optimize_graph(&graph, 1);

    println!("{}", before_score - after_score);
}

fn optimize_graph(graph: &Vec<Vec<usize>>, init_node: usize) -> usize {
    let mut graph = graph.clone();
    let mut after_score = 0;
    let mut commited_node = HashSet::new();

    commited_node.insert(init_node);

    while commited_node.len() != graph.len() {
        // 到達可能ノードから最小エッジを探索
        let min_edge = commited_node
            .iter()
            .flat_map(|&j| repeat(j).zip(0..graph[j].len()))
            .map(|(i, j)| (graph[i][j], i, j))
            .filter(|(cost, ..)| *cost != 0)
            .min_by_key(|(cost, ..)| *cost)
            .unwrap();

        // 到達可能ノードから新しく発見したノードへのエッジは不要
        commited_node.insert(min_edge.2);
        for &j in &commited_node {
            graph[min_edge.2][j] = 0;
            graph[j][min_edge.2] = 0;
        }

        after_score += min_edge.0;
    }

    after_score
}

fn make_graph() -> Vec<Vec<usize>> {
    BufReader::new(File::open("p107_network.txt").unwrap())
        .lines()
        .filter_map(|result| result.ok())
        .map(|line| {
            line.split(",")
                .map(|s| s.parse::<usize>().unwrap_or(0))
                .collect_vec()
        })
        .collect_vec()
}

fn total_score(graph: &Vec<Vec<usize>>) -> usize {
    graph.iter().map(|r| r.iter().sum::<usize>()).sum::<usize>() / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let graph = vec![
            vec![0, 16, 12, 21, 0, 0, 0],
            vec![16, 0, 0, 17, 20, 0, 0],
            vec![12, 0, 0, 28, 0, 31, 0],
            vec![21, 17, 28, 0, 18, 19, 23],
            vec![0, 20, 0, 18, 0, 0, 11],
            vec![0, 0, 31, 19, 0, 0, 27],
            vec![0, 0, 0, 23, 11, 27, 0],
        ];

        let before_score = total_score(&graph);
        let after_score = optimize_graph(&graph, 0);

        assert_eq!(243, before_score);
        assert_eq!(93, after_score);
        assert_eq!(150, before_score - after_score);
    }
}
