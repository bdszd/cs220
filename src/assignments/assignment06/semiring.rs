//! Semiring

use std::collections::HashMap;
use std::fmt::Debug;

use itertools::Itertools;

/// Semiring.
///
/// Consult <https://en.wikipedia.org/wiki/Semiring>.
pub trait Semiring: Debug + Clone + PartialEq {
    /// Additive identity.
    fn zero() -> Self;
    /// Multiplicative identity.
    fn one() -> Self;
    /// Addition operation.
    fn add(&self, rhs: &Self) -> Self;
    /// Multiplication operation.
    fn mul(&self, rhs: &Self) -> Self;
}

/// Converts integer to semiring value.
pub fn from_usize<T: Semiring>(value: usize) -> T {
    let mut result = T::zero();
    let one = T::one();

    for _ in 0..value {
        result = T::add(&result, &one);
    }

    result
}

impl Semiring for u64 {
    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }

    fn add(&self, rhs: &Self) -> Self {
        self + rhs
    }

    fn mul(&self, rhs: &Self) -> Self {
        self * rhs
    }
}

impl Semiring for i64 {
    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }

    fn add(&self, rhs: &Self) -> Self {
        self + rhs
    }

    fn mul(&self, rhs: &Self) -> Self {
        self * rhs
    }
}

impl Semiring for f64 {
    fn zero() -> Self {
        0.0
    }

    fn one() -> Self {
        1.0
    }

    fn add(&self, rhs: &Self) -> Self {
        self + rhs
    }

    fn mul(&self, rhs: &Self) -> Self {
        self * rhs
    }
}

/// Polynomials with coefficient in `C`.
///
/// For example, polynomial `x^2 + 5x + 6` is represented in `Polynomial<u64>` as follows:
///
/// ```ignore
/// Polynomial {
///     coefficients: {
///         2: 1,
///         1: 5,
///         0: 6,
///     },
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Polynomial<C: Semiring> {
    coefficients: HashMap<u64, C>,
}

impl<C: Semiring> Semiring for Polynomial<C> {
    fn zero() -> Self {
        let mut ret = HashMap::new();
        let _unused = ret.insert(0, C::zero());
        Polynomial { coefficients: ret }
    }

    fn one() -> Self {
        let mut ret = HashMap::new();
        let _unused = ret.insert(0, C::one());
        Polynomial { coefficients: ret }
    }

    fn add(&self, rhs: &Self) -> Self {
        let mut ret = HashMap::new();
        let mut temp = rhs.coefficients.clone();
        for (k, v) in &self.coefficients {
            if let Some(value) = rhs.coefficients.get(k) {
                let _unused = ret.insert(*k, v.add(value));
                let _unused = temp.remove(k);
            } else {
                let _unused = ret.insert(*k, v.clone());
            }
        }
        if !temp.is_empty() {
            ret.extend(temp);
        }
        Polynomial { coefficients: ret }
    }

    fn mul(&self, rhs: &Self) -> Self {
        let mut ret = HashMap::new();
        for (k1, v1) in &self.coefficients {
            for (k2, v2) in &rhs.coefficients {
                let _unused = ret.insert(k1 + k2, v1.mul(v2));
            }
        }
        Polynomial { coefficients: ret }
    }
}

impl<C: Semiring> Polynomial<C> {
    /// Constructs polynomial `x`.
    pub fn x() -> Self {
        let mut ret = HashMap::new();
        let _unused = ret.insert(1, C::one());
        Polynomial { coefficients: ret }
    }

    /// Evaluates the polynomial with the given value.
    pub fn eval(&self, value: C) -> C {
        let mut ret = C::zero();
        for (k, v) in &self.coefficients {
            let mut temp = C::one();
            for _ in 0..*k {
                temp = temp.mul(&value);
            }
            temp = temp.mul(v);
            let _unused = ret.add(&temp);
        }
        ret
    }

    /// Constructs polynomial `ax^n`.
    pub fn term(a: C, n: u64) -> Self {
        let mut ret = HashMap::new();
        let _unused = ret.insert(n, a);
        Polynomial { coefficients: ret }
    }
}

impl<C: Semiring> From<C> for Polynomial<C> {
    fn from(value: C) -> Self {
        let mut ret = HashMap::new();
        let _unused = ret.insert(0, value);
        Polynomial { coefficients: ret }
    }
}

/// Given a string `s`, parse it into a `Polynomial<C>`.
/// You may assume that `s` follows the criteria below.
/// Therefore, you do not have to return `Err`.
///
/// Assumptions:
/// - Each term is separated by ` + `.
/// - Each term is one of the following form: `a`, `x`, `ax`, `x^n`, and `ax^n`, where `a` is a
///   `usize` number and `n` is a `u64` number. This `a` should then be converted to a `C` type.
/// - In `a`, it is guaranteed that `a >= 1`.
/// - In `ax` and `ax^n`, it is guaranteed that `a >= 2`.
/// - In `x^n` and `ax^n`, it is guaranteed that `n >= 2`.
/// - All terms have unique degrees.
///
/// Consult `assignment06/grade.rs` for example valid strings.
///
/// Hint: `.split`, `.parse`, and `Polynomial::term`
impl<C: Semiring> std::str::FromStr for Polynomial<C> {
    type Err = (); // Ignore this for now...

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ret = HashMap::new();
        for item in s.split(" + ") {
            let (a, n) = if let Some((a, n)) = item.split_once("x^") {
                (a.parse::<usize>().unwrap(), n.parse::<u64>().unwrap())
            } else if let Some((a, _)) = item.split_once("x") {
                (a.parse::<usize>().unwrap(), 1)
            } else {
                (item.parse::<usize>().unwrap(), 0)
            };

            let coeff = from_usize::<C>(a);
            let _unused = ret.insert(n, coeff);
        }
        Ok(Polynomial { coefficients: ret })
    }
}
