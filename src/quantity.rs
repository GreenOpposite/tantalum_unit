//! An arbitrary precision value with a ```Unit```.

use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use num::{FromPrimitive, One};
use crate::scalable_integer::BigRational;
use crate::unit::{Unit, UNITLESS};

/// An arbitrary precision value with a ```Unit```.
#[derive(Clone, Debug)]
pub struct Quantity {
    pub magnitude: BigRational,
    pub unit: Unit,
}

impl Quantity {
    pub fn new(magnitude: BigRational, unit: Unit) -> Self {
        Self {
            magnitude,
            unit,
        }
    }

    pub fn from_rational(ratio: BigRational) -> Self {
        Self {
            magnitude: ratio,
            unit: UNITLESS,
        }
    }

    pub fn from_rational_with_unit(ratio: BigRational, unit: Unit) -> Self {
        Self {
            magnitude: ratio,
            unit,
        }
    }

    pub fn from_f64(value: f64) -> Self {
        let ratio = num::BigRational::from_f64(value).unwrap();
        let ratio = BigRational::new_raw(ratio.numer().clone().into(),
                                         ratio.denom().clone().into());
        Self::from_rational(ratio)
    }

    pub fn from_f64_with_unit(value: f64, unit: Unit) -> Self {
        let ratio = num::BigRational::from_f64(value).unwrap();
        let ratio = BigRational::new_raw(ratio.numer().clone().into(),
                                         ratio.denom().clone().into());
        Self::from_rational_with_unit(ratio, unit)
    }

    pub fn from_i64(value: i64) -> Self {
        let ratio = BigRational::from_integer(value.into());
        Self::from_rational(ratio)
    }

    pub fn from_i64_with_unit(value: i64, unit: Unit) -> Self {
        let ratio = BigRational::from_integer(value.into());
        Self::from_rational_with_unit(ratio, unit)
    }

    /// Constructs a Quantity with a magnitude of 1 and the given unit.
    pub fn from_unit(unit: Unit) -> Self {
        Self {
            unit,
            magnitude: BigRational::one(),
        }
    }

    /// Converts the Quantity to SI base units.
    ///
    /// # Example:
    /// ```
    /// # use tantalum_unit::quantity::Quantity;
    /// # use tantalum_unit::ratio;
    /// # use tantalum_unit::unit::Unit;
    /// use tantalum_unit::scalable_integer::BigRational;
    /// use tantalum_unit::unit::Unit::*;
    ///
    /// let temperature = Quantity::from_i64_with_unit(0, Celsius); // 0°C
    /// let si_temperature = temperature.to_si_units(); // 273.15K
    ///
    /// let value = ratio!(5463, 20); // 273.15
    /// assert_eq!(si_temperature, Quantity::new(value, Kelvin));
    /// ```
    pub fn to_si_units(self) -> Self {
        let (offset, slope, unit) = self.unit.to_si_units();
        Self {
            unit,
            magnitude: (self.magnitude + offset) * slope,
        }
    }

    /// Removes all SI and binary modifiers from the unit and applies them to the magnitude.
    ///
    /// # Example:
    /// ```
    /// # use tantalum_unit::c;
    /// # use tantalum_unit::quantity::Quantity;
    /// # use tantalum_unit::unit::Unit;
    /// use tantalum_unit::unit::Unit::*;
    ///
    /// let length = Quantity::from_i64_with_unit(5, Kilo * Meter); // 5km
    /// let new_length = length.apply_modifiers();
    ///
    /// assert_eq!(new_length, Quantity::from_i64_with_unit(5_000, Meter)); // 5000m
    /// ```
    pub fn apply_modifiers(self) -> Self {
        let (numerator, denominator) = self.unit.flatten().to_fraction();

        let mut new_magnitude = self.magnitude;
        let mut new_num = vec![];
        let mut new_denom = vec![];

        for unit in numerator {
            if unit.is_modifier() {
                let (_, slope, _) = unit.to_si_units();
                new_magnitude *= slope;
            } else {
                new_num.push(unit);
            }
        }

        for unit in denominator {
            if unit.is_modifier() {
                let (_, slope, _) = unit.to_si_units();
                new_magnitude /= slope;
            } else {
                new_denom.push(unit);
            }
        }

        Self {
            magnitude: new_magnitude,
            unit: Unit::Compound(new_num, new_denom).simplify(),
        }
    }

    /// Converts the Quantity to an arbitrary Unit. Returns an Error if that is not possible.
    ///
    /// # Example:
    /// ```
    /// # use tantalum_unit::c;
    /// # use tantalum_unit::quantity::Quantity;
    /// # use tantalum_unit::unit::Unit;
    /// use tantalum_unit::unit::Unit::*;
    ///
    /// let joule_per_second = Quantity::from_i64_with_unit(3000, Joule / Second);
    /// let kilo_watt = joule_per_second.convert_to(Kilo * Watt).unwrap();
    ///
    /// assert_eq!(kilo_watt, Quantity::from_i64_with_unit(3, Kilo * Watt));
    /// ```
    pub fn convert_to(self, to: Unit) -> Result<Self, ()> {
        let (offset, slope, unit) = self.unit.to_si_units();
        let (offset_to, slope_to, unit_to) = to.clone().to_si_units();
        if unit != unit_to {
            return Err(());
        }
        let mut new_magnitude = self.magnitude;
        new_magnitude += offset;
        new_magnitude *= slope;
        new_magnitude /= slope_to;
        new_magnitude -= offset_to;
        Ok(Self {
            unit: to,
            magnitude: new_magnitude,
        })
    }

    pub fn is_unitless(&self) -> bool {
        self.unit.is_unitless()
    }
}

impl Display for Quantity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.magnitude, self.unit)
    }
}

impl Mul for Quantity {
    type Output = Quantity;

    fn mul(self, rhs: Self) -> Self::Output {
        Quantity {
            magnitude: self.magnitude * rhs.magnitude,
            unit: self.unit * rhs.unit,
        }
    }
}

impl MulAssign for Quantity {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.clone() * rhs;
    }
}

impl Mul<BigRational> for Quantity {
    type Output = Quantity;

    fn mul(self, rhs: BigRational) -> Self::Output {
        Self {
            unit: self.unit,
            magnitude: self.magnitude * rhs,
        }
    }
}

impl MulAssign<BigRational> for Quantity {
    fn mul_assign(&mut self, rhs: BigRational) {
        *self = self.clone() * rhs;
    }
}

impl Div for Quantity {
    type Output = Quantity;

    fn div(self, rhs: Self) -> Self::Output {
        Quantity {
            magnitude: self.magnitude / rhs.magnitude,
            unit: self.unit / rhs.unit,
        }
    }
}

impl DivAssign for Quantity {
    fn div_assign(&mut self, rhs: Self) {
        *self = self.clone() / rhs;
    }
}

impl Div<BigRational> for Quantity {
    type Output = Quantity;

    fn div(self, rhs: BigRational) -> Self::Output {
        Self {
            unit: self.unit,
            magnitude: self.magnitude / rhs,
        }
    }
}

impl DivAssign<BigRational> for Quantity {
    fn div_assign(&mut self, rhs: BigRational) { *self = self.clone() / rhs; }
}

impl Neg for Quantity {
    type Output = Quantity;

    fn neg(self) -> Self::Output {
        Self {
            magnitude: -self.magnitude,
            unit: self.unit,
        }
    }
}

impl Add for Quantity {
    type Output = Quantity;

    fn add(self, mut rhs: Self) -> Self::Output {
        let self_unit_symbol = self.unit.symbol();
        let rhs_unit_symbol = rhs.unit.symbol();

        rhs = rhs.convert_to(self.unit.clone())
            .expect(format!("Cannot convert {rhs_unit_symbol} to {self_unit_symbol}.").as_str());

        Self {
            magnitude: self.magnitude + rhs.magnitude,
            unit: self.unit,
        }
    }
}

impl AddAssign for Quantity {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.clone() + rhs;
    }
}

impl Sub for Quantity {
    type Output = Quantity;

    fn sub(self, mut rhs: Self) -> Self::Output {
        let self_unit_symbol = self.unit.symbol();
        let rhs_unit_symbol = rhs.unit.symbol();

        rhs = rhs.convert_to(self.unit.clone())
            .expect(format!("Cannot convert {rhs_unit_symbol} to {self_unit_symbol}.").as_str());

        Self {
            magnitude: self.magnitude - rhs.magnitude,
            unit: self.unit,
        }
    }
}

impl SubAssign for Quantity {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.clone() - rhs;
    }
}

impl PartialEq for Quantity {
    fn eq(&self, other: &Self) -> bool {
        self.unit == other.unit
            && self.magnitude.denom() == other.magnitude.denom()
            && self.magnitude.numer() == other.magnitude.numer()
    }
}

impl Eq for Quantity {}

#[cfg(test)]
mod tests {
    use crate::{int, ratio};
    use crate::unit::Unit::*;
    use crate::scalable_integer::BigRational;
    use super::*;

    macro_rules! eq {
        ($result:expr, $magnitude:expr, $unit:expr) => {
            assert_eq!($result.magnitude, $magnitude);
            assert_eq!($result.unit, $unit);
        };
    }

    macro_rules! q {
        ($v:expr, $u:expr) => {
            Quantity::new($v, $u)
        };
    }

    #[test]
    fn multiplication_int() {
        let a = q!(int!(12), Meter);
        let b = q!(int!(999), Meter);
        let result = a * b;
        eq!(result, int!(11988), Meter * Meter);
    }

    #[test]
    fn multiplication_ratio() {
        let a = q!(ratio!(13, 5), UNITLESS / Second);
        let b = q!(ratio!(3484, 13), Meter);
        let result = a * b;
        eq!(result, ratio!(3484, 5), Meter / Second);
    }

    #[test]
    fn div_int() {
        let a = q!(int!(12), Meter);
        let b = q!(int!(999), Meter);
        let result = a / b;
        eq!(result, ratio!(4, 333), UNITLESS);
    }

    #[test]
    fn div_ratio() {
        let a = q!(ratio!(13, 5), UNITLESS / Second);
        let b = q!(ratio!(3484, 13), Meter);
        let result = a / b;
        eq!(result, ratio!(13, 1340), UNITLESS / (Second * Meter));
    }

    #[test]
    fn add_int() {
        let a = q!(int!(8342), Gallon);
        let b = q!(int!(743), Gallon);
        let result = a + b;
        eq!(result, int!(9085), Gallon);

        let a = q!(int!(8342), Gallon);
        let b = q!(int!(743), Liter);
        let result = a + b;
        eq!(result, ratio!(4040113137766i64, 473176473i64), Gallon);
    }

    #[test]
    #[should_panic]
    fn invalid_add() {
        let a = q!(int!(8342), Gallon);
        let b = q!(int!(743), Joule / Candela);

        let _result = a + b;
    }

    #[test]
    fn sub_int() {
        let a = q!(int!(8342), Gallon);
        let b = q!(int!(743), Gallon);
        let result = a - b;
        eq!(result, int!(7599), Gallon);

        let a = q!(int!(8342), Gallon);
        let b = q!(int!(743), Liter);
        let result = a - b;
        eq!(result, ratio!(3854363137766i64, 473176473i64), Gallon);
    }

    #[test]
    #[should_panic]
    fn invalid_sub() {
        let a = q!(int!(8342), Gallon);
        let b = q!(int!(743), Joule / Candela);
        let _result = a - b;
    }

    #[test]
    fn apply_modifiers_single_big() {
        let a = q!(int!(13), Yotta * Meter);
        let result = a.apply_modifiers();
        eq!(result, int!(13_000_000_000_000_000_000_000_000_i128), Meter);

        let a = q!(int!(13), Zetta * Meter);
        let result = a.apply_modifiers();
        eq!(result, int!(13_000_000_000_000_000_000_000_i128), Meter);

        let a = q!(int!(13), Exa * Meter);
        let result = a.apply_modifiers();
        eq!(result, int!(13_000_000_000_000_000_000_i128), Meter);

        let a = q!(int!(13), Peta * Meter);
        let result = a.apply_modifiers();
        eq!(result, int!(13_000_000_000_000_000_i128), Meter);

        let a = q!(int!(13), Tera * Meter);
        let result = a.apply_modifiers();
        eq!(result, int!(13_000_000_000_000_i128), Meter);

        let a = q!(int!(13), Giga * Meter);
        let result = a.apply_modifiers();
        eq!(result, int!(13_000_000_000_i128), Meter);

        let a = q!(int!(13), Mega * Meter);
        let result = a.apply_modifiers();
        eq!(result, int!(13_000_000_i128), Meter);

        let a = q!(int!(13), Kilo *Meter);
        let result = a.apply_modifiers();
        eq!(result, int!(13_000), Meter);

        let a = q!(int!(13), Hecto * Meter);
        let result = a.apply_modifiers();
        eq!(result, int!(13_00), Meter);

        let a = q!(int!(13), Meter);
        let result = a.apply_modifiers();
        eq!(result, int!(13), Meter);
    }

    #[test]
    fn apply_modifiers_single_small() {
        let a = q!(int!(13), Yocto * Meter);
        let result = a.apply_modifiers();
        eq!(result, ratio!(13, 1_000_000_000_000_000_000_000_000_i128), Meter);

        let a = q!(int!(13), Zepto * Meter);
        let result = a.apply_modifiers();
        eq!(result, ratio!(13, 1_000_000_000_000_000_000_000_i128), Meter);

        let a = q!(int!(13), Atto * Meter);
        let result = a.apply_modifiers();
        eq!(result, ratio!(13, 1_000_000_000_000_000_000_i128), Meter);

        let a = q!(int!(13), Femto * Meter);
        let result = a.apply_modifiers();
        eq!(result, ratio!(13, 1_000_000_000_000_000_i128), Meter);

        let a = q!(int!(13), Pico * Meter);
        let result = a.apply_modifiers();
        eq!(result, ratio!(13, 1_000_000_000_000_i128), Meter);

        let a = q!(int!(13), Nano * Meter);
        let result = a.apply_modifiers();
        eq!(result, ratio!(13, 1_000_000_000_i128), Meter);

        let a = q!(int!(13), Micro * Meter);
        let result = a.apply_modifiers();
        eq!(result, ratio!(13, 1_000_000_i128), Meter);

        let a = q!(int!(13), Milli * Meter);
        let result = a.apply_modifiers();
        eq!(result, ratio!(13, 1_000), Meter);

        let a = q!(int!(13), Centi * Meter);
        let result = a.apply_modifiers();
        eq!(result, ratio!(13, 100), Meter);

        let a = q!(int!(13), Deci * Meter);
        let result = a.apply_modifiers();
        eq!(result, ratio!(13, 10), Meter);
    }

    #[test]
    fn apply_modifiers_mixed() {
        let a = q!(int!(25), (Yotta * Meter) / Zetta);
        let result = a.apply_modifiers();
        eq!(result, int!(25_000), Meter);
    }

    #[test]
    fn conversion() {
        let a = q!(int!(152), Meter);
        let result = a.convert_to(Inch).unwrap();
        eq!(result, ratio!(760000, 127), Inch);

        let a = q!(int!(152), Meter);
        let result = a.convert_to(Joule);
        assert!(result.is_err());

        let a = q!(int!(38), Joule / Second);
        let result = a.convert_to(Watt).unwrap();
        eq!(result, int!(38), Watt);

        let a = q!(int!(3800), Joule / Second);
        let result = a.convert_to(Kilo * Watt).unwrap();
        eq!(result, ratio!(38, 10), Kilo * Watt);

        let a = q!(int!(3800), Joule / Second);
        let result = a.clone().convert_to((Kilo * Coulomb * Volt) / Second).unwrap();
        eq!(result, ratio!(38, 10), (Kilo * Coulomb * Volt) / Second);

        let a = q!(int!(3800), Joule / Second);
        let result = a.convert_to((Newton * Meter) / Second).unwrap();
        eq!(result, int!(3800), (Newton * Meter) / Second);
    }
}
