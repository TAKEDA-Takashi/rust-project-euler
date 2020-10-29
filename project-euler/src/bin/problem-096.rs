//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2096
//! Su Doku

use project_euler::problem_096::Sudoku;
use regex::Regex;
use std::fs;

fn main() {
    let re = Regex::new(r"Grid \d\d\r\n").unwrap();
    let content = fs::read_to_string("p096_sudoku.txt").unwrap();
    let questions: Vec<Vec<Vec<u32>>> = re
        .split(&content)
        .filter(|line| !line.is_empty())
        .map(|grid| {
            grid.split("\r\n")
                .filter(|line| !line.is_empty())
                .map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect())
                .collect()
        })
        .collect();

    let mut sum = 0;
    for q in &questions {
        let sudoku = Sudoku::new(q);
        sudoku.solve();

        // println!("{:?}", sudoku.get_left_up_3digit());
        sum += sudoku.get_left_up_3digit();
    }

    println!("{}", sum);
}
