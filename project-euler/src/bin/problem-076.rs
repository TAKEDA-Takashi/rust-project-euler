//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2076
//!
//! 5は数の和として6通りに書くことができる:
//!
//! 4 + 1
//! 3 + 2
//! 3 + 1 + 1
//! 2 + 2 + 1
//! 2 + 1 + 1 + 1
//! 1 + 1 + 1 + 1 + 1
//!
//! 2つ以上の正整数の和としての100の表し方は何通りか.

fn main() {
    println!("{}", counting_summations(&100));
}

fn counting_summations(n: &usize) -> usize {
    fn counting_summation0(n: usize, head: usize) -> usize {
        if head == 1 || n < 2 {
            1
        } else {
            let residue = n - head;
            counting_summation0(n, head - 1) + counting_summation0(residue, residue.min(head))
        }
    }

    counting_summation0(*n, n - 1)
}
