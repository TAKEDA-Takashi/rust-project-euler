//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2090
//! Cube digit pairs

use itertools::{iproduct, Itertools};

fn main() {
    let numbers = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    let count = iproduct!(
        numbers.iter().copied().combinations(6),
        numbers.iter().copied().combinations(6)
    )
    .filter(|(a, b)| depend_solve(&a, &b))
    .count();

    // aとbを逆にしたものが重複している
    println!("{:?}", count / 2);
}

fn depend_solve(a: &Vec<usize>, b: &Vec<usize>) -> bool {
    fn contains_6or9(v: &Vec<usize>) -> bool {
        v.contains(&6) || v.contains(&9)
    }

    for [x, y] in &[[0, 1], [0, 4], [2, 5], [8, 1]] {
        if !(a.contains(x) && b.contains(y) || b.contains(x) && a.contains(y)) {
            return false;
        }
    }

    // 09, 16, 36, 49, 64
    for x in &[0, 1, 3, 4] {
        if !(a.contains(x) && contains_6or9(b) || b.contains(x) && contains_6or9(a)) {
            return false;
        }
    }

    true
}
