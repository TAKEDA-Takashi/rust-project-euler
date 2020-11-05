//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2079
//!
//! オンラインバンクで通常使われるsecurity methodは, パスコードからランダムに選んだ3文字をユーザーに要求するものである.
//!
//! たとえば, パスコードが531278のとき, 2番目, 3番目, 5番目の文字を要求されるかもしれない. このとき, 期待される答えは: 317 である.
//!
//! テキストファイルkeylog.txtには, ログインに成功した50回の試行が記録されている.
//!
//! 3つの文字が常に順番通りに要求されるとするとき, ファイルを分析して, 可能なパスコードのなかでもっとも短いものを見つけよ.

// パスコードには同じ数字は現れない前提。現れる場合はロジックの改善が必要

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let key_logs: Vec<Vec<char>> = BufReader::new(File::open("files/p079_keylog.txt").unwrap())
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.chars().collect())
        .collect();

    let mut passcode = key_logs[0].clone();

    for i in 1..key_logs.len() {
        let indexes: Vec<Option<usize>> = (0..3)
            .map(|j| passcode.iter().position(|&c| c == key_logs[i][j]))
            .collect();

        for j in 0..indexes.len() {
            if indexes[j].is_none() {
                if let Some(idx) = indexes.iter().skip(j + 1).find_map(|&idx| idx) {
                    passcode.insert(idx, key_logs[i][j]);
                } else {
                    passcode.push(key_logs[i][j]);
                }
            }
        }

        if let Some((i0, i1)) = indexes[0].zip(indexes[1]).filter(|(i0, i1)| i0 > i1) {
            passcode.remove(i1);
            passcode.insert(i0, key_logs[i][1]);
        }
    }

    println!("{}", passcode.iter().collect::<String>());
}
