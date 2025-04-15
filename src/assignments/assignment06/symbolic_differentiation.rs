//! Symbolic differentiation with rational coefficents.

use num::integer::gcd;
use std::fmt;
use std::ops::*;

/// Rational number represented by two isize, numerator and denominator.
///
/// Each Rational number should be normalized so that `demoninator` is nonnegative and `numerator`
/// and `demoninator` are coprime. See `normalize` for examples. As a corner case, 0 is represented
/// by `Rational { numerator: 0, demoninator: 0 }`.
///
/// For "natural use", it also overloads standard arithmetic operations, i.e, `+`, `-`, `*`, and
/// `/`.
///
/// See [here](https://doc.rust-lang.org/core/ops/index.html) for details.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rational {
    numerator: isize,
    denominator: isize,
}

// Some useful constants.

/// Zero
pub const ZERO: Rational = Rational::new(0, 0);
/// One
pub const ONE: Rational = Rational::new(1, 1);
/// Minus one
pub const MINUS_ONE: Rational = Rational::new(-1, 1);

impl Rational {
    /// Creates a new rational number.
    pub const fn new(numerator: isize, denominator: isize) -> Self {
        Self {
            numerator,
            denominator,
        }
    }
}

impl Add for Rational {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self == ZERO {
            rhs
        } else if rhs == ZERO {
            self
        } else {
            let mut numerator = self.numerator * rhs.denominator + self.denominator * rhs.numerator;
            let mut denominator = self.denominator * rhs.denominator;
            let gcd = gcd(numerator, denominator);

            numerator /= gcd;
            denominator /= gcd;

            if denominator < 0 {
                numerator = -numerator;
                denominator = -numerator;
            }

            Self {
                numerator,
                denominator,
            }
        }
    }
}

impl Mul for Rational {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        if self == ZERO || rhs == ZERO {
            ZERO
        } else {
            let mut numerator = self.numerator * rhs.numerator;
            let mut denominator = self.denominator * rhs.denominator;
            let gcd = gcd(numerator, denominator);

            numerator /= gcd;
            denominator /= gcd;

            if denominator < 0 {
                numerator = -numerator;
                denominator = -denominator;
            }

            Self {
                numerator,
                denominator,
            }
        }
    }
}

impl Sub for Rational {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let neg = rhs.mul(MINUS_ONE);
        self.add(neg)
    }
}

impl Div for Rational {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        if rhs == ZERO {
            panic!("devide by zero");
        } else {
            let rev = Self {
                numerator: rhs.denominator,
                denominator: rhs.numerator,
            };
            self.mul(rev)
        }
    }
}

/// Differentiable functions.
///
/// For simplicity, we only consider infinitely differentiable functions.
pub trait Differentiable: Clone {
    /// Differentiate.
    ///
    /// Since the return type is `Self`, this trait can only be implemented
    /// for types that are closed under differentiation.
    fn diff(&self) -> Self;
}

impl Differentiable for Rational {
    /// HINT: Consult <https://en.wikipedia.org/wiki/Differentiation_rules#Constant_term_rule>
    fn diff(&self) -> Self {
        ZERO
    }
}

/// Singleton polynomial.
///
/// Unlike regular polynomials, this type only represents a single term.
/// The `Const` variant is included to make `Polynomial` closed under differentiation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SingletonPolynomial {
    /// Constant polynomial.
    Const(Rational),
    /// Non-const polynomial.
    Polynomial {
        /// Coefficent of polynomial. Must be non-zero.
        coeff: Rational,
        /// Power of polynomial. Must be non-zero.
        power: Rational,
    },
}

impl SingletonPolynomial {
    /// Creates a new const polynomial.
    pub fn new_c(r: Rational) -> Self {
        Self::Const(r)
    }

    /// Creates a new polynomial.
    pub fn new_poly(coeff: Rational, power: Rational) -> Self {
        Self::Polynomial { coeff, power }
    }
}

impl Differentiable for SingletonPolynomial {
    /// HINT: Consult <https://en.wikipedia.org/wiki/Power_rule>
    fn diff(&self) -> Self {
        match self {
            Self::Const(r) => Self::Const(ZERO),
            Self::Polynomial { coeff, power } => {
                let coeff = (*coeff).mul(*power);
                let power = (*power).sub(ONE);
                if power == ZERO {
                    Self::Const(coeff)
                } else {
                    Self::Polynomial { coeff, power }
                }
            }
        }
    }
}

/// Expoential function.(`e^x`)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Exp;

impl Exp {
    /// Creates a new exponential function.
    pub fn new() -> Self {
        Exp
    }
}

impl Default for Exp {
    fn default() -> Self {
        Self::new()
    }
}

impl Differentiable for Exp {
    /// HINT: Consult <https://en.wikipedia.org/wiki/Differentiation_rules#Derivatives_of_exponential_and_logarithmic_functions>
    fn diff(&self) -> Self {
        Exp
    }
}

/// Trigonometric functions.
///
/// The trig fucntions carry their coefficents to be closed under differntiation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Trignometric {
    /// Sine function.
    Sine {
        /// Coefficent
        coeff: Rational,
    },
    /// Cosine function.
    Cosine {
        /// Coefficent
        coeff: Rational,
    },
}

impl Trignometric {
    /// Creates a new sine function.
    pub fn new_sine(coeff: Rational) -> Self {
        Self::Sine { coeff }
    }

    /// Creates a new cosine function.
    pub fn new_cosine(coeff: Rational) -> Self {
        Self::Cosine { coeff }
    }
}

impl Differentiable for Trignometric {
    /// HINT: Consult <https://en.wikipedia.org/wiki/Differentiation_rules#Derivatives_of_trigonometric_functions>
    fn diff(&self) -> Self {
        match self {
            Self::Sine { coeff } => Self::Cosine { coeff: *coeff },
            Self::Cosine { coeff } => {
                let coeff = (*coeff).mul(MINUS_ONE);
                Self::Sine { coeff }
            }
        }
    }
}

/// Basic functions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BaseFuncs {
    /// Constant
    Const(Rational),
    /// Polynomial
    Poly(SingletonPolynomial),
    /// Exponential
    Exp(Exp),
    /// Trignometirc
    Trig(Trignometric),
}

impl Differentiable for BaseFuncs {
    fn diff(&self) -> Self {
        match self {
            Self::Const(r) => Self::Const(r.diff()),
            Self::Poly(p) => Self::Poly(p.diff()),
            Self::Exp(e) => Self::Exp(e.diff()),
            Self::Trig(t) => Self::Trig(t.diff()),
        }
    }
}

/// Complex functions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComplexFuncs<F> {
    /// Basic functions
    Func(F),
    /// Addition
    Add(Box<ComplexFuncs<F>>, Box<ComplexFuncs<F>>),
    /// Subtraction
    Sub(Box<ComplexFuncs<F>>, Box<ComplexFuncs<F>>),
    /// Multipliciation
    Mul(Box<ComplexFuncs<F>>, Box<ComplexFuncs<F>>),
    /// Division
    Div(Box<ComplexFuncs<F>>, Box<ComplexFuncs<F>>),
    /// Composition
    Comp(Box<ComplexFuncs<F>>, Box<ComplexFuncs<F>>),
}

impl<F: Differentiable> Differentiable for Box<F> {
    fn diff(&self) -> Self {
        Box::new(self.as_ref().diff())
    }
}

impl<F: Differentiable> Differentiable for ComplexFuncs<F> {
    /// HINT: Consult <https://en.wikipedia.org/wiki/Differentiation_rules#Elementary_rules_of_differentiation>
    fn diff(&self) -> Self {
        match self {
            Self::Func(f) => Self::Func(f.diff()),
            Self::Add(f1, f2) => Self::Add(f1.diff(), f2.diff()),
            Self::Sub(f1, f2) => Self::Sub(f1.diff(), f2.diff()),
            Self::Mul(f1, f2) => Self::Add(
                Box::new(Self::Mul(f1.diff(), f2.clone())),
                Box::new(Self::Mul(f1.clone(), f2.diff())),
            ),
            Self::Div(f1, f2) => Self::Div(
                Box::new(Self::Sub(
                    Box::new(Self::Mul(f1.diff(), f2.clone())),
                    Box::new(Self::Mul(f1.clone(), f2.diff())),
                )),
                Box::new(Self::Mul(f2.clone(), f2.clone())),
            ),
            Self::Comp(f1, f2) => Self::Mul(f2.diff(), Box::new(Self::Comp(f1.diff(), f2.clone()))),
        }
    }
}

/// Evaluate functions.
pub trait Evaluate {
    ///  Evaluate `self` at `x`.
    fn evaluate(&self, x: f64) -> f64;
}

impl Evaluate for Rational {
    fn evaluate(&self, x: f64) -> f64 {
        self.numerator as f64 / self.denominator as f64
    }
}

impl Evaluate for SingletonPolynomial {
    fn evaluate(&self, x: f64) -> f64 {
        match self {
            Self::Const(r) => r.evaluate(x),
            Self::Polynomial { coeff, power } => coeff.evaluate(x) * x.powf(power.evaluate(x)),
        }
    }
}

impl Evaluate for Exp {
    fn evaluate(&self, x: f64) -> f64 {
        x.exp()
    }
}

impl Evaluate for Trignometric {
    fn evaluate(&self, x: f64) -> f64 {
        match self {
            Self::Sine { coeff } => coeff.evaluate(x) * x.sin(),
            Self::Cosine { coeff } => coeff.evaluate(x) * x.cos(),
        }
    }
}

impl Evaluate for BaseFuncs {
    fn evaluate(&self, x: f64) -> f64 {
        match self {
            Self::Const(r) => r.evaluate(x),
            Self::Poly(p) => p.evaluate(x),
            Self::Exp(e) => e.evaluate(x),
            Self::Trig(t) => t.evaluate(x),
        }
    }
}

impl<F: Evaluate> Evaluate for ComplexFuncs<F> {
    fn evaluate(&self, x: f64) -> f64 {
        match self {
            Self::Func(f) => f.evaluate(x),
            Self::Add(f1, f2) => f1.evaluate(x) + f2.evaluate(x),
            Self::Sub(f1, f2) => f1.evaluate(x) - f2.evaluate(x),
            Self::Mul(f1, f2) => f1.evaluate(x) * f2.evaluate(x),
            Self::Div(f1, f2) => {
                if f2.evaluate(x) == 0.0 {
                    panic!("divide by zero");
                } else {
                    f1.evaluate(x) / f2.evaluate(x)
                }
            }
            Self::Comp(f1, f2) => f1.evaluate(f2.evaluate(x)),
        }
    }
}

impl fmt::Display for Rational {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if *self == ZERO {
            return write!(f, "0");
        } else if self.denominator == 1 {
            return write!(f, "{}", self.numerator);
        }
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}

impl fmt::Display for SingletonPolynomial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Const(r) => write!(f, "{r}"),
            Self::Polynomial { coeff, power } => {
                // coeff or power is zero
                if *coeff == ZERO {
                    return write!(f, "0");
                } else if *power == ZERO {
                    return write!(f, "{coeff}");
                }

                // Standard form of px^q
                let coeff = if *coeff == ONE {
                    "".to_string()
                } else if *coeff == MINUS_ONE {
                    "-".to_string()
                } else {
                    format!("({coeff})")
                };
                let var = if *power == ONE {
                    "x".to_string()
                } else {
                    format!("x^({power})")
                };
                write!(f, "{coeff}{var}")
            }
        }
    }
}

impl fmt::Display for Exp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "exp(x)")
    }
}

impl fmt::Display for Trignometric {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (func, coeff) = match self {
            Trignometric::Sine { coeff } => ("sin(x)", coeff),
            Trignometric::Cosine { coeff } => ("cos(x)", coeff),
        };

        if *coeff == ZERO {
            write!(f, "0")
        } else if *coeff == ONE {
            write!(f, "{func}")
        } else if *coeff == MINUS_ONE {
            write!(f, "-{func}")
        } else {
            write!(f, "({coeff}){func}")
        }
    }
}

impl fmt::Display for BaseFuncs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Const(r) => write!(f, "{r}"),
            Self::Poly(p) => write!(f, "{p}"),
            Self::Exp(e) => write!(f, "{e}"),
            Self::Trig(t) => write!(f, "{t}"),
        }
    }
}

impl<F: Differentiable + fmt::Display> fmt::Display for ComplexFuncs<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ComplexFuncs::Func(func) => write!(f, "{func}"),
            ComplexFuncs::Add(l, r) => write!(f, "({l} + {r})"),
            ComplexFuncs::Sub(l, r) => write!(f, "({l} - {r})"),
            ComplexFuncs::Mul(l, r) => write!(f, "({l} * {r})"),
            ComplexFuncs::Div(l, r) => write!(f, "({l} / {r})"),
            ComplexFuncs::Comp(l, r) => write!(f, "({l} ∘ {r})"),
        }
    }
}
