//! A unit like ```Meter``` or ```Gallon/Hour```.
use std::fmt::{Display, Formatter};
use std::ops::{Div, DivAssign, Mul, MulAssign};
use indexmap::IndexMap;
use num::{Zero, One};
use crate::unit::Unit::*;
use crate::{define_units, int, one, ratio, zero};
use crate::scalable_integer::{BigRational};

/// A Unit that represents a dimensionless value.
pub const UNITLESS: Unit = Compound(vec![], vec![]);


define_units!(
    // Force
    Newton, "newton", "N", zero!(), one!(), (Kilo * Gram * Meter) / (Second * Second);

    // Energy
    Joule, "joule", "J",zero!(), one!(), (Kilo * Gram * Meter * Meter) / (Second * Second);

    // Electric resistance
    Ohm, "ohm", "Ω",zero!(), one!(), (Kilo * Gram * Meter * Meter) / (Second * Second * Second * Ampere * Ampere);

    // Frequency
    Hertz, "herzt", "Hz", zero!(), one!(), UNITLESS / Second;

    // Voltage
    Volt, "volt", "V", zero!(), one!(), (Kilo * Gram * Meter * Meter) / (Second * Second * Second * Ampere);

    // Temperature
    Kelvin, "kelvin", "K", zero!(), one!(), Kelvin;
    Celsius, "celsius", "°C", ratio!(5463, 20), one!(), Kelvin;
    Fahrenheit, "fahrenheit", "°F", ratio!(45967, 100), ratio!(13889, 25000), Kelvin;

    // Area
    Hectare, "hectare", "ha", zero!(), int!(10000), Meter * Meter;

    // Magnetic field strength
    Tesla, "tesla", "T", zero!(), one!(), (Kilo * Gram) / (Second * Second * Ampere);

    // Information
    Bit, "bit", "b", zero!(), one!(), Bit;
    Byte, "byte", "B", zero!(), int!(8), Bit;

    // Electric conductance
    Siemens, "siemens", "S", zero!(), one!(), (Second * Second * Second * Ampere * Ampere) / (Kilo * Gram * Meter * Meter);

    // Power
    Watt, "watt", "W", zero!(), one!(), (Kilo * Gram * Meter * Meter) / (Second * Second * Second);

    // Volume
    Liter, "liter", "L", zero!(), ratio!(1, 1000), Meter * Meter * Meter;
    CubicInch, "cubic inch", "in^3", zero!(), ratio!(2048383, 125000000000i64), Meter * Meter * Meter;
    CubicFeet, "cubic feet", "ft^3", zero!(), ratio!(55306341, 1953125000), Meter * Meter * Meter;
    CubicYard, "cubic yard", "yd^3", zero!(), ratio!(1493271207, 1953125000), Meter * Meter * Meter;
    Pint, "pint", "pt", zero!(), ratio!(473176473, 1000000000000i64), Meter * Meter * Meter;
    Quart, "quart", "qt", zero!(), ratio!(473176473, 500000000000i64), Meter * Meter * Meter;
    Gallon, "gallon", "gal", zero!(), ratio!(473176473, 125000000000i64), Meter * Meter * Meter;

    // Pressure
    Pascal, "pascal", "Pa", zero!(), one!(), (Kilo * Gram) / (Meter * Second * Second);

    // Inductance
    Henry, "henry", "H", zero!(), one!(), (Kilo * Gram * Meter * Meter) / (Second * Second * Ampere * Ampere);

    // Amount of substance
    Mole, "mole", "mol", zero!(), one!(), Mole;

    // Luminous intensity
    Candela, "candela", "cd", zero!(), one!(), Candela;

    // Electric current
    Ampere, "ampere", "A", zero!(), one!(), Ampere;

    // Magnetic flux
    Weber, "weber", "Wb", zero!(), one!(), (Kilo * Gram * Meter * Meter) / (Second * Second * Ampere);

    // Length
    Meter, "meter", "m", zero!(), one!(), Meter;
    AU, "astronomical unit", "AU", zero!(), ratio!(149597870691i64, 1), Meter;
    Inch, "inch", "in", zero!(), ratio!(127, 5000), Meter;
    Feet, "feet", "ft", zero!(), ratio!(381, 1250), Meter;
    Yard, "yard", "yd", zero!(), ratio!(1143, 1250), Meter;
    Mile, "mile", "mi", zero!(), ratio!(201168, 125), Meter;
    NauticalMile, "nautical mile", "nmi", zero!(), ratio!(1852, 1), Meter;
    LightYear, "light year", "ly", zero!(), ratio!(9460730472580800i64, 1), Meter;
    Parsec, "parsec", "pc", zero!(), ratio!(30857000000000000i64, 1), Meter;

    // Electric Charge
    Coulomb, "coulomb", "C", zero!(), one!(), Second * Ampere;

    // Mass
    Gram, "gram", "g", zero!(), one!(), Gram;
    Tonne, "tonne", "t", zero!(), ratio!(1000000, 1), Gram;
    Dram, "dram", "dr", zero!(), ratio!(17718451953i64, 10_000_000_000i64), Gram;
    Ounce, "ounce", "oz", zero!(), ratio!(45_359_237i64, 1_600_000i64), Gram;
    Pound, "pound", "lb", zero!(), ratio!(45359237, 100_000), Gram;

    // Electric capacitance
    Farad, "farad", "F", zero!(), one!(), (Second * Second * Second * Second * Ampere * Ampere) / (Kilo * Gram * Meter * Meter);

    // Time
    Second, "second", "s", zero!(), one!(), Second;
    Minute, "minute", "min", zero!(), ratio!(60, 1), Second;
    Hour, "hour", "h", zero!(), ratio!(3600, 1), Second;
    Day, "day", "d", zero!(), ratio!(86400, 1), Second;
    Month, "month", "mo", zero!(), ratio!(2629746, 1), Second;
    Year, "year", "yr", zero!(), ratio!(31557600, 1), Second;

    // SI modifiers
    Quecto, "quecto", "q", zero!(), ratio!(1, 1_000_000_000_000_000_000_000_000_000_000i128), UNITLESS;
    Ronto, "ronto", "r", zero!(), ratio!(1, 1_000_000_000_000_000_000_000_000_000i128), UNITLESS;
    Yocto, "yocto", "y", zero!(), ratio!(1, 1_000_000_000_000_000_000_000_000i128), UNITLESS;
    Zepto, "zepto", "z", zero!(), ratio!(1, 1_000_000_000_000_000_000_000i128), UNITLESS;
    Atto, "atto", "a", zero!(), ratio!(1, 1_000_000_000_000_000_000i128), UNITLESS;
    Femto, "femto", "f", zero!(), ratio!(1, 1_000_000_000_000_000i64), UNITLESS;
    Pico, "pico", "p", zero!(), ratio!(1, 1_000_000_000_000i64), UNITLESS;
    Nano, "nano", "n", zero!(), ratio!(1, 1_000_000_000), UNITLESS;
    Micro, "micro", "µ", zero!(), ratio!(1, 1_000_000), UNITLESS;
    Milli, "milli", "m", zero!(), ratio!(1, 1_000), UNITLESS;
    Centi, "centi", "c", zero!(), ratio!(1, 100), UNITLESS;
    Deci, "deci", "d", zero!(), ratio!(1, 10), UNITLESS;
    Hecto, "hecto", "h", zero!(), ratio!(100, 1), UNITLESS;
    Kilo, "kilo", "k", zero!(), ratio!(1_000, 1), UNITLESS;
    Mega, "mega", "M", zero!(), ratio!(1_000_000, 1), UNITLESS;
    Giga, "giga", "G", zero!(), ratio!(1_000_000_000, 1), UNITLESS;
    Tera, "tera", "T", zero!(), ratio!(1_000_000_000_000i64, 1), UNITLESS;
    Peta, "peta", "P", zero!(), ratio!(1_000_000_000_000_000i64, 1), UNITLESS;
    Exa, "exa", "E", zero!(), ratio!(1_000_000_000_000_000_000i128, 1), UNITLESS;
    Zetta, "zetta", "Z", zero!(), ratio!(1_000_000_000_000_000_000_000i128, 1), UNITLESS;
    Yotta, "yotta", "Y", zero!(), ratio!(1_000_000_000_000_000_000_000_000i128, 1), UNITLESS;
    Ronna, "ronna", "R", zero!(), ratio!(1_000_000_000_000_000_000_000_000_000i128, 1), UNITLESS;
    Quetta, "quetta", "Q", zero!(), ratio!(1_000_000_000_000_000_000_000_000_000_000i128, 1), UNITLESS;

    // IEC binary modifiers
    Kibi, "kibi", "Ki", zero!(), ratio!(1024, 1), UNITLESS;
    Mebi, "mebi", "Mi", zero!(), ratio!(1048576, 1), UNITLESS;
    Gibi, "gibi", "Gi", zero!(), ratio!(1073741824, 1), UNITLESS;
    Tebi, "tebi", "Ti", zero!(), ratio!(1099511627776i64, 1), UNITLESS;
    Pebi, "pebi", "Pi", zero!(), ratio!(1125899906842624i64, 1), UNITLESS;
    Exbi, "exbi", "Ei", zero!(), ratio!(1152921504606846976i64, 1), UNITLESS
);

impl Unit {
    /// Flattens nested Compound units without canceling units.
    pub fn flatten(self) -> Self {
        use Unit::*;

        match self {
            Compound(numerator, denominator) => {
                let mut flat_numerator = Vec::new();
                let mut flat_denominator = Vec::new();

                for unit in numerator {
                    match unit.flatten() {
                        Compound(inner_numerator, inner_denominator) => {
                            flat_numerator.extend(inner_numerator);
                            flat_denominator.extend(inner_denominator);
                        }
                        u => flat_numerator.push(u),
                    }
                }

                for unit in denominator {
                    match unit.flatten() {
                        Compound(inner_numerator, inner_denominator) => {
                            flat_numerator.extend(inner_denominator);
                            flat_denominator.extend(inner_numerator);
                        }
                        simple_unit => flat_denominator.push(simple_unit),
                    }
                }

                Compound(flat_numerator, flat_denominator)
            }
            u => u,
        }
    }

    /// Returns the unit in the form of ```(numerator, denominator)```
    pub fn to_fraction(self) -> (Vec<Unit>, Vec<Unit>) {
        use Unit::*;

        match self {
            Compound(n, d) => (n, d),
            u => (vec![u], vec![]),
        }
    }

    /// Cancels the units in a Compound unit
    ///
    /// # Example:
    /// ```
    /// # use tantalum_unit::c;
    /// # use tantalum_unit::unit::Unit;
    /// use tantalum_unit::unit::Unit::*;
    ///
    /// let unit = (Meter * Second) / Second; // ms/s
    /// let simplified = unit.simplify();
    ///
    /// assert_eq!(simplified, Meter);
    /// ```
    pub fn simplify(mut self) -> Self {
        use Unit::*;

        self = self.flatten();

        match self {
            Compound(ref mut num, ref mut denom) => {
                let mut i = 0;
                while i < num.len() {
                    if let Some(pos) = denom.iter().position(|d| d == &num[i]) {
                        num.remove(i);
                        denom.remove(pos);
                    } else {
                        i += 1;
                    }
                }
                if denom.is_empty() {
                    if num.len() == 1 {
                        num[0].clone()
                    } else {
                        Compound(num.clone(), vec![])
                    }
                } else {
                    Compound(num.clone(), denom.clone())
                }
            }
            u => u
        }
    }

    /// Checks if the unit is an SI or binary modifier like Micro or Kibi.
    pub fn is_modifier(&self) -> bool {
        match self {
            Yocto => true,
            Zepto => true,
            Atto => true,
            Femto => true,
            Pico => true,
            Nano => true,
            Micro => true,
            Milli => true,
            Centi => true,
            Deci => true,
            Hecto => true,
            Kilo => true,
            Mega => true,
            Giga => true,
            Tera => true,
            Peta => true,
            Exa => true,
            Zetta => true,
            Yotta => true,
            Kibi => true,
            Mebi => true,
            Gibi => true,
            Tebi => true,
            Pebi => true,
            Exbi => true,

            _ => false
        }
    }

    /// Checks if the unit represents a dimensionless value.
    pub fn is_unitless(&self) -> bool {
        *self == UNITLESS
    }
}

impl Mul for Unit {
    type Output = Unit;

    fn mul(self, rhs: Self) -> Self::Output {
        let (mut numer1, mut denom1) = self.to_fraction();
        let (mut numer2, mut denom2) = rhs.to_fraction();
        numer1.append(&mut numer2);
        denom1.append(&mut denom2);
        Compound(numer1, denom1).simplify()
    }
}

impl MulAssign for Unit {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.clone() * rhs;
    }
}

impl Div for Unit {
    type Output = Unit;

    fn div(self, rhs: Self) -> Self::Output {
        let (mut numer1, mut denom1) = self.to_fraction();
        let (mut numer2, mut denom2) = rhs.to_fraction();
        // Multiply with the reciprocal
        numer1.append(&mut denom2);
        denom1.append(&mut numer2);
        Compound(numer1, denom1).simplify()
    }
}

impl DivAssign for Unit {
    fn div_assign(&mut self, rhs: Self) {
        *self = self.clone() / rhs;
    }
}

impl Display for Unit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.symbol())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::c;

    #[test]
    fn flatten() {
        let result = ((Meter / Second) / Second).flatten();
        assert_eq!(result, Meter / (Second * Second));

        let result = (Second / (Meter / Second)).flatten();
        assert_eq!(result, (Second * Second) / Meter);

        let result = ((Meter / Second) / (Meter / Second)).flatten();
        assert_eq!(result, (Meter * Second) / (Second * Meter));

        let result = (((Watt / Joule) / Second) / (Meter / Second)).flatten();
        assert_eq!(result, (Watt * Second) / (Joule * Second * Meter));
    }

    #[test]
    fn simplify_simple() {
        let result = Joule.simplify();
        assert_eq!(result, Joule);

        let result = Meter.simplify();
        assert_eq!(result, Meter);

        let result = Gallon.simplify();
        assert_eq!(result, Gallon);

        let result = Year.simplify();
        assert_eq!(result, Year);
    }

    #[test]
    fn simplify_compound_unitless() {
        let result = c!(Second; Second).simplify();
        assert_eq!(result, UNITLESS);
    }

    #[test]
    fn simplify_compound() {
        let result = c!(Second, Watt; Second).simplify();
        assert_eq!(result, Watt);

        let result = c!(Second, Watt; Second).simplify();
        assert_eq!(result, Watt);

        let result = c!(Meter, Watt, Meter, AU; Second).simplify();
        assert_eq!(result, c!(Meter, Watt, Meter, AU; Second));

        let result = c!(Kelvin, Kelvin; Kelvin).simplify();
        assert_eq!(result, Kelvin);

        let result = c!(Year; Ounce, Year).simplify();
        assert_eq!(result, c!(; Ounce));
    }

    #[test]
    fn simplify_nested_compound() {
        let result = c!(c ! (Meter; Second); Second).simplify();
        assert_eq!(result, c!(Meter; Second, Second));

        let result = c!(c ! (c ! (Watt; Joule); Second); c ! (Meter; Second)).simplify();
        assert_eq!(result, c!(Watt; Joule, Meter));
    }

    #[test]
    fn to_fraction() {
        let result = Gram.to_fraction();
        assert_eq!(result, (vec![Gram], vec![]));

        let result = (Meter / Second).to_fraction();
        assert_eq!(result, (vec![Meter], vec![Second]));
    }
}
