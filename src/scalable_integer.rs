use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign};
use num::{BigInt, CheckedAdd, Integer, Num, One, Zero};
use num::rational::Ratio;
use crate::scalable_integer::ScalableInteger::{Big, Double, Single};

pub type BigRational = Ratio<ScalableInteger>;

#[derive(Debug, Clone, PartialOrd, Ord)]
pub enum ScalableInteger {
    Single(i64),
    Double(i128),
    Big(BigInt),
}

impl ScalableInteger {
    fn max_size(a: ScalableInteger, b: ScalableInteger) -> (ScalableInteger, ScalableInteger) {
        use ScalableInteger::*;
        match (a, b) {
            (Single(a), Single(b)) => { (Single(a), Single(b)) }
            (Single(a), Double(b)) => { (Double(a.into()), Double(b)) }
            (Single(a), Big(b)) => { (Big(a.into()), Big(b)) }
            (Double(a), Single(b)) => { (Double(a.into()), Double(b.into())) }
            (Double(a), Double(b)) => { (Double(a), Double(b)) }
            (Double(a), Big(b)) => { (Big(a.into()), Big(b)) }
            (Big(a), Single(b)) => { (Big(a), Big(b.into())) }
            (Big(a), Double(b)) => { (Big(a), Big(b.into())) }
            (Big(a), Big(b)) => { (Big(a), Big(b)) }
        }
    }

    fn promote_size(self) -> Self {
        use ScalableInteger::*;
        match self {
            Single(n) => { Double(n.into()) }
            Double(n) => { Big(n.into()) }
            Big(n) => { Big(n) }
        }
    }

    fn demote_size(self) -> Self {
        use ScalableInteger::*;
        match self {
            Single(n) => { Single(n) }
            Double(n) => {
                if let Ok(x) = n.try_into() {
                    Single(x)
                } else {
                    Double(n)
                }
            }
            Big(n) => {
                if let Ok(x) = n.clone().try_into() {
                    Single(x)
                } else if let Ok(x) = n.clone().try_into() {
                    Double(x)
                } else {
                    Big(n)
                }
            }
        }
    }
}

impl From<BigInt> for ScalableInteger {
    fn from(v: BigInt) -> Self {
        ScalableInteger::Big(v)
    }
}

impl From<u8> for ScalableInteger { fn from(value: u8) -> Self { Single(value.into()) } }
impl From<i8> for ScalableInteger { fn from(value: i8) -> Self { Single(value.into()) } }
impl From<u32> for ScalableInteger { fn from(value: u32) -> Self { Single(value.into()) } }
impl From<i32> for ScalableInteger { fn from(value: i32) -> Self { Single(value.into()) } }
impl From<u64> for ScalableInteger { fn from(value: u64) -> Self { Double(value.into()).demote_size() } }
impl From<i64> for ScalableInteger { fn from(value: i64) -> Self { Single(value.into()).demote_size() } }
impl From<u128> for ScalableInteger { fn from(value: u128) -> Self { Big(value.into()).demote_size() } }
impl From<i128> for ScalableInteger { fn from(value: i128) -> Self { Double(value.into()).demote_size() } }

impl PartialEq for ScalableInteger {
    fn eq(&self, other: &Self) -> bool {
        let (a, b) = ScalableInteger::max_size(self.clone(), other.clone());
        match (a, b) {
            (Single(a), Single(b)) => { a.eq(&b) }
            (Double(a), Double(b)) => { a.eq(&b) }
            (Big(a), Big(b)) => { a.eq(&b) }
            _ => unreachable!()
        }
    }
}

impl Eq for ScalableInteger {}

impl Neg for ScalableInteger {
    type Output = ScalableInteger;

    fn neg(self) -> Self::Output {
        use ScalableInteger::*;
        match self {
            Single(n) => Single(-n),
            Double(n) => Double(-n),
            Big(n) => Big(-n),
        }
    }
}

impl Display for ScalableInteger {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use ScalableInteger::*;
        match self {
            Single(n) => { write!(f, "{n}") }
            Double(n) => { write!(f, "{n}") }
            Big(n) => { write!(f, "{n}") }
        }
    }
}

impl Mul for &ScalableInteger {
    type Output = ScalableInteger;

    fn mul(self, rhs: Self) -> Self::Output {
        self.clone() * rhs.clone()
    }
}

impl Div for &ScalableInteger {
    type Output = ScalableInteger;

    fn div(self, rhs: Self) -> Self::Output {
        self.clone() / rhs.clone()
    }
}

impl Add for &ScalableInteger {
    type Output = ScalableInteger;

    fn add(self, rhs: Self) -> Self::Output {
        self.clone() + rhs.clone()
    }
}

impl Sub for &ScalableInteger {
    type Output = ScalableInteger;

    fn sub(self, rhs: Self) -> Self::Output {
        self.clone() - rhs.clone()
    }
}

impl Zero for ScalableInteger {
    fn zero() -> Self {
        Self::Single(0)
    }

    fn is_zero(&self) -> bool {
        match self {
            ScalableInteger::Single(n) => { n.is_zero() }
            ScalableInteger::Double(n) => { n.is_zero() }
            ScalableInteger::Big(n) => { n.is_zero() }
        }
    }
}

impl One for ScalableInteger {
    fn one() -> Self {
        Self::Single(1)
    }
}

impl Add<Self> for ScalableInteger {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let (lhs, rhs) = ScalableInteger::max_size(self, rhs);
        match (lhs.clone(), rhs.clone()) {
            (Single(a), Single(b)) => {
                if let Some(result) = a.checked_add(b) {
                    Single(result)
                } else {
                    let (lhs, rhs) = (lhs.promote_size(), rhs.promote_size());
                    lhs + rhs
                }
            }
            (Double(a), Double(b)) => {
                if let Some(result) = a.checked_add(b) {
                    Double(result)
                } else {
                    let (lhs, rhs) = (lhs.promote_size(), rhs.promote_size());
                    lhs + rhs
                }
            }
            (Big(a), Big(b)) => {
                Big(a + b)
            }
            _ => unreachable!()
        }.demote_size()
    }
}

impl AddAssign for ScalableInteger {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.clone() + rhs;
    }
}

impl Sub<Self> for ScalableInteger {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let rhs = rhs.neg();
        self + rhs
    }
}

impl SubAssign for ScalableInteger {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.clone() - rhs;
    }
}

impl Mul<Self> for ScalableInteger {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let (lhs, rhs) = ScalableInteger::max_size(self, rhs);
        match (lhs.clone(), rhs.clone()) {
            (Single(a), Single(b)) => {
                if let Some(result) = a.checked_mul(b) {
                    Single(result)
                } else {
                    let (lhs, rhs) = (lhs.promote_size(), rhs.promote_size());
                    lhs * rhs
                }
            }
            (Double(a), Double(b)) => {
                if let Some(result) = a.checked_mul(b) {
                    Double(result)
                } else {
                    let (lhs, rhs) = (lhs.promote_size(), rhs.promote_size());
                    lhs * rhs
                }
            }
            (Big(a), Big(b)) => {
                Big(a * b)
            }
            _ => unreachable!()
        }.demote_size()
    }
}

impl MulAssign for ScalableInteger {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.clone() * rhs;
    }
}

impl Div<Self> for ScalableInteger {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let (lhs, rhs) = ScalableInteger::max_size(self, rhs);
        match (lhs.clone(), rhs.clone()) {
            (Single(a), Single(b)) => {
                if let Some(result) = a.checked_div(b) {
                    Single(result)
                } else {
                    let (lhs, rhs) = (lhs.promote_size(), rhs.promote_size());
                    lhs / rhs
                }
            }
            (Double(a), Double(b)) => {
                if let Some(result) = a.checked_div(b) {
                    Double(result)
                } else {
                    let (lhs, rhs) = (lhs.promote_size(), rhs.promote_size());
                    lhs / rhs
                }
            }
            (Big(a), Big(b)) => {
                Big(a / b)
            }
            _ => unreachable!()
        }.demote_size()
    }
}

impl DivAssign for ScalableInteger {
    fn div_assign(&mut self, rhs: Self) {
        *self = self.clone() / rhs;
    }
}

impl RemAssign<Self> for ScalableInteger {
    fn rem_assign(&mut self, rhs: Self) {
        *self = self.clone() % rhs;
    }
}

impl Rem<Self> for ScalableInteger {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Num for ScalableInteger {
    type FromStrRadixErr = ();

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        todo!()
    }
}

impl Integer for ScalableInteger {
    fn div_floor(&self, other: &Self) -> Self {
        todo!()
    }

    fn mod_floor(&self, other: &Self) -> Self {
        todo!()
    }

    fn gcd(&self, other: &Self) -> Self {
        let (lhs, rhs) = ScalableInteger::max_size(self.clone(), other.clone());
        match (lhs, rhs) {
            (Single(a), Single(b)) => {
                Single(a.gcd(&b))
            }
            (Double(a), Double(b)) => {
                Double(a.gcd(&b))
            }
            (Big(a), Big(b)) => {
                Big(a.gcd(&b))
            }
            _ => unreachable!()
        }
    }

    fn lcm(&self, other: &Self) -> Self {
        let (lhs, rhs) = ScalableInteger::max_size(self.clone(), other.clone());
        match (lhs, rhs) {
            (Single(a), Single(b)) => {
                Single(a.lcm(&b))
            }
            (Double(a), Double(b)) => {
                Double(a.lcm(&b))
            }
            (Big(a), Big(b)) => {
                Big(a.lcm(&b))
            }
            _ => unreachable!()
        }
    }

    fn is_multiple_of(&self, other: &Self) -> bool {
        todo!()
    }

    fn is_even(&self) -> bool {
        match self {
            Single(n) => { n.is_even() }
            Double(n) => { n.is_even() }
            Big(n) => { n.is_even() }
        }
    }

    fn is_odd(&self) -> bool {
        !self.is_even()
    }

    fn div_rem(&self, other: &Self) -> (Self, Self) {
        todo!()
    }
}
