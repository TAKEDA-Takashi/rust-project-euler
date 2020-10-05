use std::cmp::Ordering;

pub struct Intersect<T1, T2>
where
    T1: Iterator,
    T2: Iterator,
{
    iter1: T1,
    iter2: T2,
}

impl<T1, T2> Intersect<T1, T2>
where
    T1: Iterator,
    T2: Iterator,
{
    /// 2つのイテレータの共通項を生成する。イテレータは単調増加である必要がある。
    ///
    /// # Example
    ///
    /// ```
    /// use problem_045::Intersect;
    ///
    /// let mut intersect = Intersect::new(vec![1,3,5,9,11,13].into_iter(), vec![1,2,3,5,8,13].into_iter());
    /// assert_eq!(intersect.next(), Some(1));
    /// assert_eq!(intersect.next(), Some(3));
    /// assert_eq!(intersect.next(), Some(5));
    /// assert_eq!(intersect.next(), Some(13));
    /// assert_eq!(intersect.next(), None);
    /// ```
    pub fn new(iter1: T1, iter2: T2) -> Intersect<T1, T2> {
        Intersect { iter1, iter2 }
    }
}

impl<T1, T2> Iterator for Intersect<T1, T2>
where
    T1: Iterator,
    T2: Iterator<Item = T1::Item>,
    T1::Item: Ord + Copy,
{
    type Item = T1::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let mut t1 = self.iter1.next();
        let mut t2 = self.iter2.next();

        loop {
            match t1.zip(t2).map(|(a, b)| a.cmp(&b)) {
                Some(Ordering::Less) => t1 = self.iter1.next(),
                Some(Ordering::Greater) => t2 = self.iter2.next(),
                Some(Ordering::Equal) => break,
                None => return None,
            }
        }

        t1
    }
}
