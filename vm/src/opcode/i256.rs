use primitive_types::U256;
use std::cmp::Ordering;
use std::ops::{Div, Rem};

const SIGN_MASK: U256 = U256([
    0xffffffffffffffff,
    0xffffffffffffffff,
    0xffffffffffffffff,
    0x7fffffffffffffff,
]);

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Sign {
    Positive,
    Negative,
    NoSign,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct I256(Sign, U256);

impl I256 {
    pub fn zero() -> Self {
        Self(Sign::NoSign, U256::zero())
    }

    pub fn min() -> Self {
        Self(Sign::Negative, (U256::MAX & SIGN_MASK) + U256::one())
    }
}

impl From<U256> for I256 {
    fn from(v: U256) -> I256 {
        if v == U256::zero() {
            Self::zero()
        // checks first bit (U256 is little-endian)
        } else if v.bit(255) {
            // return two's complement form
            Self(Sign::Negative, !v + U256::one())
        } else {
            Self(Sign::Positive, v)
        }
    }
}

impl From<I256> for U256 {
    fn from(value: I256) -> U256 {
        let sign = value.0;
        if sign == Sign::NoSign {
            U256::zero()
        } else if sign == Sign::Positive {
            value.1
        } else {
            !value.1 + U256::from(1u64)
        }
    }
}

impl Div for I256 {
    type Output = I256;

    fn div(self, other: I256) -> I256 {
        if other == I256::zero() {
            return I256::zero();
        }

        if self == I256::min() && other.1 == U256::one() {
            return I256::min();
        }

        let magnitude = (self.1 / other.1) & SIGN_MASK;

        if magnitude == U256::zero() {
            return I256::zero();
        }

        match (self.0, other.0) {
            (Sign::NoSign, Sign::NoSign)
            | (Sign::NoSign, Sign::Positive)
            | (Sign::Positive, Sign::Positive)
            | (Sign::Positive, Sign::NoSign)
            | (Sign::Negative, Sign::Negative) => I256(Sign::Positive, magnitude),
            (Sign::NoSign, Sign::Negative)
            | (Sign::Negative, Sign::NoSign)
            | (Sign::Positive, Sign::Negative)
            | (Sign::Negative, Sign::Positive) => I256(Sign::Negative, magnitude),
        }
    }
}

impl Rem for I256 {
    type Output = I256;

    fn rem(self, other: I256) -> I256 {
        let r = self.1 % other.1;

        if r == U256::zero() {
            return I256::zero();
        }

        I256(self.0, r)
    }
}

impl Ord for I256 {
    fn cmp(&self, other: &I256) -> Ordering {
        match (self.0, other.0) {
            (Sign::NoSign, Sign::NoSign) => Ordering::Equal,
            (Sign::NoSign, Sign::Positive) => Ordering::Less,
            (Sign::NoSign, Sign::Negative) => Ordering::Greater,
            (Sign::Negative, Sign::NoSign) => Ordering::Less,
            (Sign::Negative, Sign::Positive) => Ordering::Less,
            (Sign::Negative, Sign::Negative) => self.1.cmp(&other.1).reverse(),
            (Sign::Positive, Sign::Negative) => Ordering::Greater,
            (Sign::Positive, Sign::NoSign) => Ordering::Greater,
            (Sign::Positive, Sign::Positive) => self.1.cmp(&other.1),
        }
    }
}

impl PartialOrd for I256 {
    fn partial_cmp(&self, other: &I256) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn division() {
        let neg_one = I256(Sign::Negative, U256::from(1));
        let hundred = I256(Sign::Positive, U256::from(100));
        let neg_hundred = I256(Sign::Negative, U256::from(100));

        let q = hundred / neg_one;
        assert_eq!(q, neg_hundred);
    }
}
