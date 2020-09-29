//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2028
//!
//! 1から初めて右方向に進み時計回りに数字を増やしていき, 5×5の螺旋が以下のように生成される:
//!
//! 21	22	23	24	25
//! 20	7	8	9	10
//! 19	6	1	2	11
//! 18	5	4	3	12
//! 17	16	15	14	13
//! 両対角線上の数字の合計は101であることが確かめられる.
//!
//! 1001×1001の螺旋を同じ方法で生成したとき, 対角線上の数字の和はいくつか?

fn main() {
    println!("{}", (1..=501).map(get_sum_edge_number).sum::<usize>());
}

fn get_sum_edge_number(n: usize) -> usize {
    if n == 1 {
        return 1;
    }

    4 * (get_right_top(n) - 3 * (n - 1))
}

fn get_right_top(n: usize) -> usize {
    (2 * n - 1).pow(2)
}
