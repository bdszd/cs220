//! Implement functions using `Iterator` trait

struct FindIter<'s, T: Eq> {
    query: &'s [T],
    base: &'s [T],
    curr: usize,
}

impl<T: Eq> Iterator for FindIter<'_, T> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while self.curr + self.query.len() <= self.base.len() {
            if &self.base[self.curr..self.curr + self.query.len()] == self.query {
                let ret = self.curr;
                self.curr += 1;
                return Some(ret);
            }
            self.curr += 1;
        }
        None
    }
}

/// Returns an iterator over substring query indexes in the base.
pub fn find<'s, T: Eq>(query: &'s [T], base: &'s [T]) -> impl 's + Iterator<Item = usize> {
    FindIter {
        query,
        base,
        curr: 0,
    }
}

/// Implement generic fibonacci iterator
struct FibIter<T> {
    // TODO: remove `_marker` and add necessary fields as you want
    curr: T,
    next: T,
}

impl<T: std::ops::Add<Output = T> + Copy> FibIter<T> {
    fn new(first: T, second: T) -> Self {
        Self {
            curr: first,
            next: second,
        }
    }
}

impl<T> Iterator for FibIter<T>
where
    T: std::ops::Add<Output = T> + Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let (mut a, mut b) = (self.curr, self.next);
        self.curr = b;
        self.next = a + b;
        Some(a)
    }
}

/// Returns and iterator over the generic fibonacci sequence starting from `first` and `second`.
/// This is a generic version of `fibonacci` function, which works for any types that implements
/// `std::ops::Add` trait.
pub fn fib<T>(first: T, second: T) -> impl Iterator<Item = T>
where
    T: std::ops::Add<Output = T> + Copy,
{
    FibIter::new(first, second)
}

/// Endpoint of range, inclusive or exclusive.
#[derive(Debug)]
pub enum Endpoint {
    /// Inclusive endpoint
    Inclusive(isize),

    /// Exclusive endpoint
    Exclusive(isize),
}

struct RangeIter {
    // TODO: add necessary fields as you want
    start: isize,
    end: isize,
    step: isize,
}

impl RangeIter {
    fn new(endpoints: (Endpoint, Endpoint), step: isize) -> Self {
        let (start, end) = if step > 0 {
            match (endpoints.0, endpoints.1) {
                (Endpoint::Inclusive(s), Endpoint::Inclusive(e)) => (s, e),
                (Endpoint::Inclusive(s), Endpoint::Exclusive(e)) => (s, e - 1),
                (Endpoint::Exclusive(s), Endpoint::Inclusive(e)) => (s + 1, e),
                (Endpoint::Exclusive(s), Endpoint::Exclusive(e)) => (s + 1, e - 1),
            }
        } else {
            match (endpoints.0, endpoints.1) {
                (Endpoint::Inclusive(s), Endpoint::Inclusive(e)) => (s, e),
                (Endpoint::Inclusive(s), Endpoint::Exclusive(e)) => (s, e + 1),
                (Endpoint::Exclusive(s), Endpoint::Inclusive(e)) => (s - 1, e),
                (Endpoint::Exclusive(s), Endpoint::Exclusive(e)) => (s - 1, e + 1),
            }
        };
        RangeIter { start, end, step }
    }
}

impl Iterator for RangeIter {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.step > 0 {
            if self.start <= self.end {
                let result = self.start;
                self.start += self.step;
                return Some(result);
            } else {
                return None;
            }
        }

        if self.start >= self.end {
            let result = self.start;
            self.start += self.step;
            Some(result)
        } else {
            None
        }
    }
}

/// Returns an iterator over the range [left, right) with the given step.
pub fn range(left: Endpoint, right: Endpoint, step: isize) -> impl Iterator<Item = isize> {
    RangeIter::new((left, right), step)
}

/// Write an iterator that returns all divisors of n in increasing order.
/// Assume n > 0.
///
/// Hint: trying all candidates from 1 to n will most likely time out!
/// To optimize it, make use of the following fact:
/// if x is a divisor of n that is greater than sqrt(n),
/// then n/x is a divisor of n that is smaller than sqrt(n).
struct Divisors {
    n: u64,
    curr: u64,
    factor: Vec<u64>, // TODO: you may define additional fields here
}

impl Iterator for Divisors {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let sqrt = (self.n as f64).sqrt();
        while self.curr <= sqrt.floor() as u64 {
            if self.n % self.curr == 0 {
                let ret = self.curr;
                self.factor.push(ret);
                self.curr += 1;
                return Some(ret);
            }
            self.curr += 1;
        }
        if sqrt == sqrt.trunc() {
            _ = self.factor.pop();
        }
        if let Some(x) = self.factor.pop() {
            Some(self.n / x)
        } else {
            None
        }
    }
}

/// Returns an iterator over the divisors of n.
pub fn divisors(n: u64) -> impl Iterator<Item = u64> {
    Divisors {
        n,
        curr: 1,
        factor: vec![], // TODO: you may define additional fields here
    }
}
