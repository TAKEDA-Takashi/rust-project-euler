//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%20102
//!
//! 3つの異なる点が -1000 ≤ x, y ≤ 1000 かつ三角形となるように, デカルト平面上にランダムに与えられる.
//!
//! 以下の2つの三角形を考える.
//!
//! A(-340,495), B(-153,-910), C(835,-947)
//! X(-175,41), Y(-421,-714), Z(574,-645)
//! 三角形ABCが原点を内部に含み, XYZは原点を内部に含まないことが確かめられる.
//!
//! 27Kのテキストファイルtriangles.txt(右クリックしリンク先を保存して欲しい) にランダムな1000個の三角形が適当なフォーマットのもと含まれている. 内部に原点を含む三角形の数を答えよ.
//!
//! 注: ファイル中の最初の二つは三角形ABC, XYZである.

use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let contains_count = BufReader::new(File::open("p102_triangles.txt").unwrap())
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            line.split(",")
                .filter_map(|s| s.parse::<f64>().ok())
                .chunks(2)
                .into_iter()
                .map(|c| c.collect_vec())
                .collect_vec()
        })
        .map(|p| is_contains_origin(&p[0], &p[1], &p[2]))
        .filter(|contains| *contains)
        .count();

    println!("{}", contains_count);
}

fn is_contains_origin(a: &Vec<f64>, b: &Vec<f64>, c: &Vec<f64>) -> bool {
    let f_a = |x| x / a[0] * a[1];
    let f_b = |x| x / b[0] * b[1];

    if f_a(b[0]) > b[1] && f_a(c[0]) > c[1] || f_a(b[0]) < b[1] && f_a(c[0]) < c[1] {
        return false;
    }

    if f_b(a[0]) > a[1] && f_b(c[0]) > c[1] || f_b(a[0]) < a[1] && f_b(c[0]) < c[1] {
        return false;
    }

    true
}

#[test]
fn test() {
    assert!(is_contains_origin(
        &vec![-340., 495.],
        &vec![-153., -910.],
        &vec![835., -947.]
    ));

    assert!(!is_contains_origin(
        &vec![-175., 41.],
        &vec![-421., -714.],
        &vec![574., -645.]
    ));
}
