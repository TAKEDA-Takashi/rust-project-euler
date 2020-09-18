use std::cmp::Ordering;
use std::collections::BTreeMap;

pub struct PrimeFactorization {
    prime_factor_map: BTreeMap<usize, Vec<usize>>,
    prime_list: Vec<usize>,
}

impl PrimeFactorization {
    pub fn new() -> PrimeFactorization {
        PrimeFactorization {
            prime_factor_map: BTreeMap::new(),
            prime_list: vec![],
        }
    }

    pub fn exec(&mut self, n: usize) -> &Vec<usize> {
        if self.prime_factor_map.contains_key(&n) {
            return &self.prime_factor_map[&n];
        }

        let mut prime_iter = PrimeIterator {
            ps: &mut self.prime_list,
            index: 0,
        };

        self.prime_factor_map.insert(
            n,
            prime_factorization(n, &mut prime_iter, &self.prime_factor_map),
        );

        &self.prime_factor_map[&n]
    }
}

fn prime_factorization(
    n: usize,
    p_iter: &mut PrimeIterator,
    prime_factor_map: &BTreeMap<usize, Vec<usize>>,
) -> Vec<usize> {
    if n == 0 {
        panic!("0 cannot be factored");
    }

    let p = p_iter.next().unwrap();

    if prime_factor_map.contains_key(&n) {
        return prime_factor_map[&n].clone();
    }

    match n.cmp(&(p * p)) {
        Ordering::Less => vec![n],
        Ordering::Equal => vec![p, p],
        Ordering::Greater => {
            if n % p == 0 {
                let mut ps = vec![p];
                ps.extend(prime_factorization(n / p, p_iter, prime_factor_map));
                ps
            } else {
                prime_factorization(n, p_iter, prime_factor_map)
            }
        }
    }
}

#[derive(Debug)]
struct PrimeIterator<'a> {
    ps: &'a mut Vec<usize>,
    index: usize,
}

impl<'a> Iterator for PrimeIterator<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.ps.get(self.index);
        self.index += 1;

        if let Some(p) = n {
            return Some(*p);
        }

        match self.ps.last() {
            None => {
                self.ps.push(2);
                Some(2)
            }
            Some(2) => {
                self.ps.push(3);
                Some(3)
            }
            Some(last_prime) => {
                let next_prime = ((last_prime + 2)..).step_by(2).find(|n| {
                    self.ps
                        .iter()
                        .take_while(|&p| p * p <= *n)
                        .all(|p| n % p != 0)
                })?;

                self.ps.push(next_prime);
                Some(next_prime)
            }
        }
    }
}
