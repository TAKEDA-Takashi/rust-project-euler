#[derive(Debug)]
pub struct Prime {
    ps: Vec<usize>,
}

impl Prime {
    /// 素数列を生成するイテレータを生成する。
    ///
    /// # Example
    ///
    /// ```
    /// use problem_007::Prime;
    ///
    /// let mut prime = Prime::new();
    /// assert_eq!(prime.next(), Some(2));
    /// assert_eq!(prime.next(), Some(3));
    /// assert_eq!(prime.next(), Some(5));
    /// assert_eq!(prime.next(), Some(7));
    /// ```
    pub fn new() -> Prime {
        Prime { ps: vec![] }
    }
}

impl Iterator for Prime {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
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
                        .take_while(|p| *p * *p <= *n)
                        .all(|p| n % p != 0)
                })?;

                self.ps.push(next_prime);
                Some(next_prime)
            }
        }
    }
}
