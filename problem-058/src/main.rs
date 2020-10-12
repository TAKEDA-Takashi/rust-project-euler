//! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%2058
//!
//! 1から始めて, 以下のように反時計回りに数字を並べていくと, 辺の長さが7の渦巻きが形成される.
//!
//! 37	36	35	34	33	32	31
//! 38	17	16	15	14	13	30
//! 39	18	5	4	3	12	29
//! 40	19	6	1	2	11	28
//! 41	20	7	8	9	10	27
//! 42	21	22	23	24	25	26
//! 43	44	45	46	47	48	49
//! 面白いことに, 奇平方数が右下の対角線上に出現する. もっと面白いことには, 対角線上の13個の数字のうち, 8個が素数である. ここで割合は8/13 ≈ 62%である.
//!
//! 渦巻きに新しい層を付け加えよう. すると辺の長さが9の渦巻きが出来る. 以下, この操作を繰り返していく. 対角線上の素数の割合が10%未満に落ちる最初の辺の長さを求めよ.

// 問題28に似てる

use euler_lib::is_prime;

fn main() {
    println!("{}", 2 * find_under_ten_per() - 1);
}

fn find_under_ten_per() -> usize {
    fn find_under_ten_per0(prime_count: usize, count: usize, term: usize) -> usize {
        if prime_count * 100 / count < 10 && prime_count != 0 {
            term
        } else {
            find_under_ten_per0(
                prime_count
                    + get_edge_number(term + 1)
                        .iter()
                        .filter(|m| is_prime(*m))
                        .count(),
                count + 4,
                term + 1,
            )
        }
    }

    find_under_ten_per0(0, 1, 1)
}

fn get_edge_number(n: usize) -> [usize; 4] {
    assert!(n > 1);

    let rb = get_right_bottom(n);
    let d = 2 * (n - 1);

    [rb - 3 * d, rb - 2 * d, rb - d, rb]
}

fn get_right_bottom(n: usize) -> usize {
    (2 * n - 1).pow(2)
}
