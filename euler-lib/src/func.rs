use num::{one, range, range_inclusive, zero, FromPrimitive, Integer, ToPrimitive};

use std::string::ToString;

pub fn is_palindrome<T: ToString>(n: &T) -> bool {
    let s = n.to_string();
    s == s.chars().rev().collect::<String>()
}

pub fn mul_palindrome<T>(t: &(T, T)) -> Option<(T, T, T)>
where
    T: Integer + Clone + ToString,
{
    let m = t.0.clone() * t.1.clone();
    if is_palindrome(&m) {
        Some((t.0.clone(), t.1.clone(), m.clone()))
    } else {
        None
    }
}

pub fn gcd<T>(a: &T, b: &T) -> T
where
    T: Integer + Clone,
{
    if *b == zero() {
        a.clone()
    } else {
        gcd(b, &(a.clone() % b.clone()))
    }
}

pub fn combination<T>(n: &T, r: &T) -> T
where
    T: Integer + Clone,
{
    if *r == zero() {
        return one();
    }

    combination(n, &(r.clone() - one())) * (n.clone() - r.clone() + one()) / r.clone()
}

pub fn factorial<T>(n: &T) -> T
where
    T: Integer + FromPrimitive + ToPrimitive + Clone,
{
    let o: T = one();

    if *n == zero() {
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
        if n.clone() % m.clone() == zero() {
            acc.push(m.clone());
        }
        acc
    })
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
    fn test_mul_palindrome() {
        assert_eq!(Some((11, 11, 121)), mul_palindrome(&(11, 11)));
        assert_eq!(None, mul_palindrome(&(11, 12)));
    }

    #[test]
    fn test_gcd() {
        assert_eq!(4, gcd(&4, &12));
        assert_eq!(1, gcd(&11, &19));
        assert_eq!(5, gcd(&10, &25));
        assert_eq!(4, gcd(&20, &8));
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
}
