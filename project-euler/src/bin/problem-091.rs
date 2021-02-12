//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2091
//! Right triangles with integer coordinates

use std::iter::repeat;

fn main() {
    let ubound = 50;

    // Qがx軸上またはPがy軸上にある場合、自明な直角三角形ができる
    let trivial = 3 * ubound * ubound;
    let diagonal = (1..=ubound)
        .flat_map(|x| repeat(x).zip(1..=ubound))
        .map(|(x0, y0)| {
            let f = orthogonal_line(x0, y0);
            (0..=ubound)
                .filter_map(|x| {
                    f(x).filter(|&y| 0 <= y && y <= ubound && y != y0)
                        .map(|y| (x, y))
                })
                .count()
        })
        .sum::<usize>();

    println!("{}", trivial + diagonal as isize);
}

// (x0, y0)を通り直交する直線の式
fn orthogonal_line(x0: isize, y0: isize) -> impl Fn(isize) -> Option<isize> {
    move |x| {
        let m = (-x0 * x) + (x0 * x0 + y0 * y0);
        (m % y0 == 0).then(|| m / y0)
    }
}
