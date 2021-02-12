use itertools::Itertools;
use num::bigint::{RandBigInt, ToBigUint};
use num::{one, range, range_inclusive, BigUint, FromPrimitive, Integer, One, ToPrimitive};
use std::collections::HashSet;
use std::iter::once;

pub fn is_palindrome<T: ToString>(n: &T) -> bool {
    let s = n.to_string();
    s == s.chars().rev().collect::<String>()
}

pub fn is_binary_palindrome<T: std::fmt::Binary>(n: &T) -> bool {
    let s = format!("{:b}", n);
    s == s.chars().rev().collect::<String>()
}

pub fn mul_palindrome<T>(t: &(T, T)) -> Option<(T, T, T)>
where
    T: Integer + Clone + ToString,
{
    let m = t.0.clone() * t.1.clone();
    is_palindrome(&m).then(|| (t.0.clone(), t.1.clone(), m.clone()))
}

pub fn is_pandigital(s: &str) -> bool {
    let set = s.chars().collect::<HashSet<_>>();
    set.len() == 9 && !set.contains(&'0')
}

pub fn combination<T>(n: &T, r: &T) -> T
where
    T: Integer + Clone,
{
    if r.is_zero() {
        return one();
    }

    combination(n, &(r.clone() - one())) * (n.clone() - r.clone() + one()) / r.clone()
}

pub fn modpow<T, U>(b: T, e: U, m: T) -> T
where
    T: Integer + Copy,
    U: Integer + Copy + std::ops::Shr<u32, Output = U>,
{
    fn modpow0<T, U>(b: T, e: U, m: T, r: T) -> T
    where
        T: Integer + Copy,
        U: Integer + Copy + std::ops::Shr<u32, Output = U>,
    {
        if e.is_zero() {
            r
        } else {
            modpow0(b * b % m, e >> 1, m, if e.is_odd() { r * b % m } else { r })
        }
    }
    modpow0(b, e, m, one())
}

pub fn factorial<T>(n: &T) -> T
where
    T: Integer + FromPrimitive + ToPrimitive + Clone,
{
    let o: T = one();

    if n.is_zero() {
        o.clone()
    } else if *n == o {
        o.clone()
    } else {
        range(T::from_u32(2).unwrap(), n.clone()).fold(n.clone(), |acc, m| acc * m)
    }
}

pub fn get_divisors<T>(n: &T) -> Vec<T>
where
    T: Integer + FromPrimitive + ToPrimitive + Clone,
{
    range_inclusive(one(), n.clone() / T::from_u32(2).unwrap()).fold(vec![], |mut acc, m| {
        if (n.clone() % m.clone()).is_zero() {
            acc.push(m.clone());
        }
        acc
    })
}

/// 素因数の列から約数の個数を計算する。
/// psはソート済みの素数の列である必要があります。
pub fn divisor_count<T>(ps: &Vec<T>) -> usize
where
    T: Integer,
{
    // 約数の個数は素因数の指数に+1をして総乗する
    ps.iter()
        .skip_while(|n| **n == one())
        .group_by(|&n| n)
        .into_iter()
        .map(|(_, group)| group.count() + 1)
        .product::<usize>()
}

/// 素因数の列から約数を生成する。
pub fn make_divisors<T>(ps: &Vec<T>) -> Vec<T>
where
    T: Integer + Copy + std::iter::Product,
{
    once(one())
        .chain((1..=ps.len()).flat_map(|c| {
            ps.iter()
                .combinations(c)
                .map(|combi| combi.into_iter().copied().product::<T>())
        }))
        .sorted()
        .dedup()
        .collect_vec()
}

pub fn primitive_pythagorean_triple<T>(m: T, n: T) -> Option<(T, T, T)>
where
    T: Integer + FromPrimitive + Copy,
{
    assert!(m > n);

    let two = T::from_u32(2).unwrap();

    if (m - n) % two != one() || m.gcd(&n) != one() {
        return None;
    }

    let a = m * m - n * n;
    let b = two * m * n;
    let c = m * m + n * n;

    if a < b {
        Some((a, b, c))
    } else {
        Some((b, a, c))
    }
}

const MILLER_RABIN_ROUND: usize = 20;

pub fn is_prime<T>(n: &T) -> bool
where
    T: Integer
        + FromPrimitive
        + ToBigUint
        + std::ops::Shr<usize, Output = T>
        + std::ops::BitAnd<Output = T>
        + std::cmp::Eq
        + Clone,
{
    if n == &T::from_u32(2_u32).unwrap() || n == &T::from_u32(3_u32).unwrap() {
        true
    } else if n < &T::from_u32(2_u32).unwrap() || n.is_even() {
        false
    } else {
        miller_rabin_test(n)
    }
}

// nが奇素数 =>
//   ∃d ∈ N ∧ odd(d), ∃s ∈ Z s.t. n - 1 == 2^s * d. ∀a ∈ [2, n - 2]. ∃r ∈ Z ∧ 0 <= r < s.
//   a^d % n == 1 または a^((2^r)*d) % n == -1 i.e. (a^d)^(2^r) % n == -1
//   のいずれかが成り立つ
fn miller_rabin_test<T>(n: &T) -> bool
where
    T: Integer
        + FromPrimitive
        + ToBigUint
        + std::ops::Shr<usize, Output = T>
        + std::ops::BitAnd<Output = T>
        + Clone,
{
    let (d, s) = find_odd(n.clone() - one());

    let mut rng = rand::thread_rng();

    let n = n.to_biguint().unwrap();
    let d = d.to_biguint().unwrap();
    let lbound = BigUint::from(2_u32);
    let ubound = &n - 2_u32;

    (0..MILLER_RABIN_ROUND).all(|_| {
        let a = rng.gen_biguint_range(&lbound, &ubound);

        let y = a.modpow(&d, &n);
        if y.is_one() {
            // a^d % n == 1
            true
        } else {
            // ∃r ∈ Z ∧ 0 <= r < s. (a^d)^(2^r) % n == -1
            // (0_u32..s).any(|r| modpow(y.clone(), 2_u32.pow(r), n.clone()) == &n - one::<BigUint>())
            (0_u32..s).any(|r| y.modpow(&BigUint::from(2_u32).pow(r), &n) == &n - one::<BigUint>())
        }
    })
}

fn find_odd<T>(q: T) -> (T, u32)
where
    T: Integer + std::ops::Shr<usize, Output = T> + std::ops::BitAnd<Output = T> + Clone,
{
    let mut s: u32 = 0;
    let mut q = q.clone();

    while (q.clone() & one()).is_one() {
        q = q >> 1;
        s += 1;
    }

    (q, s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome(&110011));
        assert!(is_palindrome(&12321));
        assert!(!is_palindrome(&123456));
    }

    #[test]
    fn test_is_binary_palindrome() {
        assert!(is_binary_palindrome(&0b110011));
        assert!(is_binary_palindrome(&0b10101010101));
        assert!(!is_binary_palindrome(&0b11000));
    }

    #[test]
    fn test_mul_palindrome() {
        assert_eq!(Some((11, 11, 121)), mul_palindrome(&(11, 11)));
        assert_eq!(None, mul_palindrome(&(11, 12)));
    }

    #[test]
    fn test_combination() {
        assert_eq!(6, combination(&6, &1));
        assert_eq!(21, combination(&7, &2));
        assert_eq!(1, combination(&4, &4));
        assert_eq!(120, combination(&10, &3));
    }

    #[test]
    fn test_get_divisors() {
        assert_eq!(vec![1, 2, 3], get_divisors(&6));
        assert_eq!(vec![1], get_divisors(&5));
        assert_eq!(vec![1, 2, 4], get_divisors(&8));
        assert_eq!(vec![1, 2, 3, 6, 9], get_divisors(&18));
    }

    #[test]
    fn test_make_divisors() {
        assert_eq!(vec![1, 2, 3, 4, 6, 12], make_divisors(&vec![2, 2, 3]));
        assert_eq!(vec![1, 3, 5, 15], make_divisors(&vec![3, 5]));
        assert_eq!(vec![1, 2, 4, 8, 16], make_divisors(&vec![2, 2, 2, 2]));
    }

    #[test]
    fn test_primitive_pythagorean_triple() {
        assert_eq!(Some((3, 4, 5)), primitive_pythagorean_triple(2, 1));
        assert_eq!(Some((5, 12, 13)), primitive_pythagorean_triple(3, 2));
        assert_eq!(None, primitive_pythagorean_triple(3, 1));
    }

    #[test]
    fn test_is_prime() {
        assert!(is_prime(&9999991));
        assert!(is_prime(&9007199254740997_u64));
        assert!(!is_prime(&106330));
        assert!(!is_prime(&8635844967113809_u128));
        assert!(is_prime(&BigUint::from_u64(9007199254740997).unwrap()));
    }
}
