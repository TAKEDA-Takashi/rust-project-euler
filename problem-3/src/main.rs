///! http://odz.sakura.ne.jp/projecteuler/index.php?cmd=read&page=Problem%203
///!
///! 13195 の素因数は 5, 7, 13, 29 である.
///! 600851475143 の素因数のうち最大のものを求めよ.
use std::vec::Vec;

fn prime_factorization(n: usize) -> Vec<usize> {
    if n == 1 {
        return vec![1];
    }

    let mut prime_list = vec![];
    prime_factorization0(n, 2, &mut prime_list);
    prime_list
}

fn prime_factorization0(n: usize, v: usize, prime_list: &mut Vec<usize>) {
    if n == 0 {
        panic!("0 cannot be factored");
    }

    if v < 2 {
        panic!("divisor (is {}) must be greater than or equal to 2.", v);
    }

    if n < v {
        panic!(
            "divisor (is {}) must be less than or equal to dividend (is {}).",
            v, n
        );
    }

    if n == 1 {
        return;
    } else if n < v * v {
        return prime_list.push(n);
    }

    if n % v == 0 {
        prime_list.push(v);
        prime_factorization0(n / v, v, prime_list);
    } else if v == 2 {
        prime_factorization0(n, 3, prime_list);
    } else {
        prime_factorization0(n, v + 2, prime_list);
    }
}

fn main() {
    // println!("{:?}", prime_factorization(600851475143));
    println!("{}", prime_factorization(600851475143).last().unwrap());
}
