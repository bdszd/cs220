//! Big integer with infinite precision.

use std::fmt;
use std::iter::zip;
use std::ops::*;

/// An signed integer with infinite precision implemented with an "carrier" vector of `u32`s.
///
/// The vector is interpreted as a base 2^(32 * (len(carrier) - 1)) integer, where negative
/// integers are represented in their [2's complement form](https://en.wikipedia.org/wiki/Two%27s_complement).
///
/// For example, the vector `vec![44,345,3]` represents the integer
/// `44 * (2^32)^2 + 345 * (2^32) + 3`,
/// and the vector `vec![u32::MAX - 5, u32::MAX - 7]` represents the integer
/// `- (5 * 2^32 + 8)`
///
/// You will implement the `Add` and `Sub` trait for this type.
///
/// Unlike standard fix-sized intergers in Rust where overflow will panic, the carrier is extended
/// to save the overflowed bit. On the contrary, if the precision is too much (e.g, vec![0,0] is
/// used to represent 0, where `vec![0]` is sufficent), the carrier is truncated.
///
/// See [this section](https://en.wikipedia.org/wiki/Two%27s_complement#Arithmetic_operations) for a rouge guide on implementation,
/// while keeping in mind that the carrier should be extended to deal with overflow.
///
/// The `sign_extension()`, `two_complement()`, and `truncate()` are non-mandatory helper methods.
///
/// For testing and debugging purposes, the `Display` trait is implemented for you, which shows the
/// integer in hexadecimal form.
#[derive(Debug, Clone)]
pub struct BigInt {
    /// The carrier for `BigInt`.
    ///
    /// Note that the carrier should always be non-empty.
    pub carrier: Vec<u32>,
}

impl BigInt {
    /// Create a new `BigInt` from a `usize`.
    pub fn new(n: u32) -> Self {
        Self { carrier: vec![n] }
    }

    /// Creates a new `BigInt` from a `Vec<u32>`.
    ///
    /// # Panic
    ///
    /// Panics if `carrier` is empty.
    pub fn new_large(carrier: Vec<u32>) -> Self {
        assert!(!carrier.is_empty());
        Self { carrier }.truncate()
    }
}

const SIGN_MASK: u32 = 1 << 31;

impl BigInt {
    /// Extend `self` to `len` bits.
    fn sign_extension(&self, len: usize) -> Self {
        let mut new_carrier = self.carrier.clone();
        let sign_bit = new_carrier[0] & SIGN_MASK != 0;
        let extend_word = if sign_bit { u32::MAX } else { 0 };

        while new_carrier.len() < len {
            new_carrier.insert(0, extend_word);
        }
        BigInt {
            carrier: new_carrier,
        }
    }

    /// Compute the two's complement of `self`.
    fn two_complement(&self) -> Self {
        let mut ret = Vec::new();
        let mut carry = 1u64;
        for &x in self.carrier.iter().rev() {
            let inver = !x as u64;
            let sum = inver + carry;
            ret.insert(0, sum as u32);
            carry = sum >> 32;
        }

        BigInt { carrier: ret }
    }

    /// Truncate a `BigInt` to the minimum length.
    fn truncate(&self) -> Self {
        let mut carrier = self.carrier.clone();
        let sign_bit = (carrier[0] & SIGN_MASK) != 0;
        let extend_word = if sign_bit { u32::MAX } else { 0 };

        let mut first_keep = 0;

        while carrier.len() > 1 && carrier[0] == extend_word {
            let second = carrier[1];
            let expected_bit = (second & SIGN_MASK != 0) as u32;
            let expected_word = if expected_bit == 1 { u32::MAX } else { 0 };

            if expected_word != extend_word {
                break;
            }
            let _ = carrier.remove(0);
        }

        BigInt { carrier }
    }
}

impl Add for BigInt {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let max_len = self.carrier.len().max(rhs.carrier.len());

        let lhs = self.sign_extension(max_len);
        let rhs = rhs.sign_extension(max_len);
        let lhs_sign = self.carrier[0] & SIGN_MASK != 0;
        let rhs_sign = rhs.carrier[0] & SIGN_MASK != 0;

        let mut ret = Vec::with_capacity(max_len + 2);
        let mut carry = 0u64;

        for (a, b) in zip(lhs.carrier.iter().rev(), rhs.carrier.iter().rev()) {
            let sum = *a as u64 + *b as u64 + carry;
            ret.insert(0, sum as u32);
            carry = sum >> 32;
        }

        let first_sign = ret[0] & SIGN_MASK != 0;

        if rhs_sign == lhs_sign && first_sign != rhs_sign {
            if rhs_sign {
                ret.insert(0, u32::MAX);
            } else {
                ret.insert(0, 0_u32);
            }
        }

        // if !rhs_sign && !lhs_sign && first_sign {
        //     ret.insert(0, carry as u32);
        // }
        // if rhs_sign && lhs_sign && !first_sign {
        //     ret.insert(0, carry as u32);
        //     ret.insert(0, u32::MAX);
        // }

        BigInt { carrier: ret }.truncate()
    }
}

impl Sub for BigInt {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.add(rhs.two_complement())
    }
}

impl fmt::Display for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Hex formatting so that each u32 can be formatted independently.
        for i in self.carrier.iter() {
            write!(f, "{:08x}", i)?;
        }
        Ok(())
    }
}
