use num::{zero, Integer};
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
}
