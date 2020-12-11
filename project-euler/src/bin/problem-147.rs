//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%20147
//! Rectangles in cross-hatched grids

use std::cmp::Ord;

fn main() {
    let x_bound = 47;
    let y_bound = 43;

    let mut sum = 0;

    for y in 1..=y_bound {
        for x in y..=x_bound {
            let total = count_total_rectangles(x, y);
            sum += total;

            if x != y && x <= y_bound {
                sum += total;
            }
        }
    }

    println!("{}", sum);
}

fn count_total_rectangles(x: usize, y: usize) -> usize {
    assert!(x >= y);

    if x == 1 && y == 1 {
        1
    } else if x == y {
        count_total_rectangles(x, y - 1) + sum_additional_rectangles(x, y)
    } else {
        count_total_rectangles(x - 1, y) + sum_additional_rectangles(x, y)
    }
}

fn sum_additional_rectangles(x: usize, y: usize) -> usize {
    let count = (1..2 * y)
        .map(|n| up_and_down(x, y, n) + diagonal_rectangles(x, y, n))
        .sum::<usize>();

    count + regular_rectangles(x, y)
}

fn regular_rectangles(x: usize, y: usize) -> usize {
    x * y * (y + 1) / 2
}

fn todown(x: usize, y: usize, n: usize) -> usize {
    assert!(x >= y);
    assert!(n < 2 * y);

    match n {
        1 => Ord::min(2 * y - 1, 2 * (x - 1)),
        2 => todown(x, y, 1) - if x == y { 0 } else { 1 },
        3 => todown(x, y, 2) - 1,
        _ => todown(x, y, n - 2) - 2,
    }
}

fn up_and_down(x: usize, y: usize, n: usize) -> usize {
    if x == 1 && y == 1 {
        return 0;
    }

    if n == y {
        2 * todown(x, y, n) - 1
    } else if n < y {
        n + todown(x, y, n) - 1
    } else {
        up_and_down(x, y, 2 * y - n)
    }
}

fn diagonal_rectangles(x: usize, y: usize, n: usize) -> usize {
    assert!(x >= y);
    assert!(n < 2 * y);

    if n == 1 {
        0
    } else if n == y {
        (y - 1).pow(2) - if x == y && x % 2 == 1 { 1 } else { 0 }
    } else if n < y {
        (n - 1) * (todown(x, y, n) - 1) - if x == y && n % 2 == 1 { 1 } else { 0 }
    } else {
        diagonal_rectangles(x, y, 2 * y - n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_total_rectangles() {
        assert_eq!(18, count_total_rectangles(2, 2));
        assert_eq!(37, count_total_rectangles(3, 2));
        assert_eq!(87, count_total_rectangles(3, 3));
    }

    #[test]
    fn test_regular_rectangles() {
        assert_eq!(9, regular_rectangles(3, 2));
        assert_eq!(40, regular_rectangles(4, 4));
        assert_eq!(50, regular_rectangles(5, 4));
    }

    #[test]
    fn test_up_and_down() {
        assert_eq!(47, (1..8).map(|n| up_and_down(4, 4, n)).sum::<usize>());
        assert_eq!(49, (1..8).map(|n| up_and_down(5, 4, n)).sum::<usize>());
        assert_eq!(49, (1..8).map(|n| up_and_down(6, 4, n)).sum::<usize>());

        assert_eq!(79, (1..10).map(|n| up_and_down(5, 5, n)).sum::<usize>());
        assert_eq!(81, (1..10).map(|n| up_and_down(6, 5, n)).sum::<usize>());
        assert_eq!(81, (1..10).map(|n| up_and_down(7, 5, n)).sum::<usize>());
    }

    #[test]
    fn test_diagonal_square() {
        assert_eq!(
            9,
            (1..6).map(|n| diagonal_rectangles(3, 3, n)).sum::<usize>()
        );
        assert_eq!(
            10,
            (1..6).map(|n| diagonal_rectangles(4, 3, n)).sum::<usize>()
        );
        assert_eq!(
            10,
            (1..6).map(|n| diagonal_rectangles(5, 3, n)).sum::<usize>()
        );

        assert_eq!(
            33,
            (1..8).map(|n| diagonal_rectangles(4, 4, n)).sum::<usize>()
        );
        assert_eq!(
            35,
            (1..8).map(|n| diagonal_rectangles(5, 4, n)).sum::<usize>()
        );
        assert_eq!(
            35,
            (1..8).map(|n| diagonal_rectangles(6, 4, n)).sum::<usize>()
        );

        assert_eq!(
            81,
            (1..10).map(|n| diagonal_rectangles(5, 5, n)).sum::<usize>()
        );
        assert_eq!(
            84,
            (1..10).map(|n| diagonal_rectangles(6, 5, n)).sum::<usize>()
        );
        assert_eq!(
            84,
            (1..10).map(|n| diagonal_rectangles(7, 5, n)).sum::<usize>()
        );
    }
}
