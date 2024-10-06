//! A unit like ```Meter``` or ```Gallon/Hour```.
use std::fmt::{Display, Formatter};
use std::ops::{Div, DivAssign, Mul, MulAssign};
use indexmap::IndexMap;
use num::{BigRational, One, Zero};
use crate::unit::Unit::*;
use crate::{c, one, ratio, zero};

/// A Unit that represents a dimensionless value.
pub const UNITLESS: Unit = Compound(vec![], vec![]);

/// A Unit of measurement.
///
/// # Example:
/// ```
/// # use tantalum_unit::c;
/// # use tantalum_unit::unit::Unit;
/// // Simple units
/// let meter = Unit::Meter;
/// let year = Unit::Year;
///
/// // More complex units can be created using Unit::Compound
/// let joule_per_second = Unit::Compound(vec![Unit::Joule], vec![Unit::Second]);
/// let kilo_meter = Unit::Compound(vec![Unit::Kilo, Unit::Meter], vec![]);
///
/// // Or by multiplying/dividing units
/// use tantalum_unit::unit::Unit::*;
///
/// let joule_per_second = Joule / Second;
/// let kilo_meter = Kilo * Meter;
/// ```
#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub enum Unit {
    // Force
    Newton,

    // Energy
    Joule,

    // Electric resistance
    Ohm,

    // Frequency
    Hertz,

    // Voltage
    Volt,

    // Temperature
    Kelvin,
    Celsius,
    Fahrenheit,

    // Area
    Hectare,

    // Magnetic field strength
    Tesla,

    // Information
    Bit,
    Byte,

    // Electric conductance
    Siemens,

    // Power
    Watt,

    // Volume
    Liter,
    CubicInch,
    CubicFeet,
    CubicYard,
    Pint,
    Quart,
    Gallon,

    // Pressure
    Pascal,

    // Inductance
    Henry,

    // SI modifiers
    Quecto,
    Ronto,
    Yocto,
    Zepto,
    Atto,
    Femto,
    Pico,
    Nano,
    Micro,
    Milli,
    Centi,
    Deci,
    Hecto,
    Kilo,
    Mega,
    Giga,
    Tera,
    Peta,
    Exa,
    Zetta,
    Yotta,
    Ronna,
    Quetta,

    // Amount of substance
    Mole,

    // Luminous intensity
    Candela,

    // Electric current
    Ampere,

    // Magnetic flux
    Weber,

    // Binary modifiers
    Kibi,
    Mebi,
    Gibi,
    Tebi,
    Pebi,
    Exbi,

    // Length
    Meter,
    AU,
    Inch,
    Feet,
    Yard,
    Mile,
    NauticalMile,
    LightYear,
    Parsec,

    // Electric charge
    Coulomb,

    // Mass
    Gram,
    Tonne,
    Dram,
    Ounce,
    Pound,

    // Electric capacitance
    Farad,

    // Time
    Second,
    Minute,
    Hour,
    Day,
    Month,
    Year,

    // Compound
    /// Represents a Unit as a fraction in the form
    /// ```
    /// # use tantalum_unit::unit::Unit::Compound;
    /// # let (numerator, denominator) = (vec![], vec![]);
    /// Compound(numerator, denominator);
    /// ```
    Compound(Vec<Unit>, Vec<Unit>),
}

impl Unit {
    /// Converts a Unit to its SI representation, removing prefixes and returning offset and slope.
    ///
    /// # Returns:
    /// ```
    /// # let (offset, slope, unit) = (0,0,0);
    /// (offset, slope, unit);
    /// ```
    ///
    /// # Example:
    /// ```
    /// # use tantalum_unit::unit::Unit;
    /// let temperature = Unit::Celsius;
    /// let (offset, slope, unit) = temperature.to_si_units();
    /// // Returns (273.15, 1.0, Unit::Kelvin) because Celsius is defined as C = K + 273.15
    /// ```
    pub fn to_si_units(mut self) -> (BigRational, BigRational, Unit) {
        use Unit::*;
        self = self.flatten();
        match self
        {
            // Force
            Newton => (zero!(), one!(), (Kilo * Gram * Meter) / (Second * Second)),

            // Energy
            Joule => (zero!(), one!(), (Kilo * Gram * Meter * Meter) / (Second * Second)),

            // Electric resistance
            Ohm => (zero!(), one!(), (Kilo * Gram * Meter * Meter) / (Second * Second * Second * Ampere * Ampere)),

            // Frequency
            Hertz => (zero!(), one!(), UNITLESS / Second),

            // Voltage
            Volt => (zero!(), one!(), (Kilo * Gram * Meter * Meter) / (Second * Second * Second * Ampere)),

            // Temperature
            Kelvin => (zero!(), one!(), Kelvin),
            Celsius => (ratio!(5463, 20), one!(), Kelvin),
            Fahrenheit => (ratio!(45967, 100), ratio!(13889, 25000), Kelvin),

            // Area
            Hectare => (zero!(), ratio!(10000, 1), Meter * Meter),

            // Magnetic field strength
            Tesla => (zero!(), one!(), (Kilo * Gram) / (Second * Second * Ampere)),

            // Information
            Bit => (zero!(), one!(), Bit),
            Byte => (zero!(), ratio!(8, 1), Bit),

            // Electric conductance
            Siemens => (zero!(), one!(), (Second * Second * Second * Ampere * Ampere) / (Kilo * Gram * Meter * Meter)),

            // Power
            Watt => (zero!(), one!(), (Kilo * Gram * Meter * Meter) / (Second * Second * Second)),

            // Volume
            Liter => (zero!(), ratio!(1, 1000), Meter * Meter * Meter),
            CubicInch => (zero!(), ratio!(2048383, 125000000000i64), Meter * Meter * Meter),
            CubicFeet => (zero!(), ratio!(55306341, 1953125000), Meter * Meter * Meter),
            CubicYard => (zero!(), ratio!(1493271207, 1953125000), Meter * Meter * Meter),
            Pint => (zero!(), ratio!(473176473, 1000000000000i64), Meter * Meter * Meter),
            Quart => (zero!(), ratio!(473176473, 500000000000i64), Meter * Meter * Meter),
            Gallon => (zero!(), ratio!(473176473, 125000000000i64), Meter * Meter * Meter),

            // Pressure
            Pascal => (zero!(), one!(), (Kilo * Gram) / (Meter * Second * Second)),

            // Inductance
            Henry => (zero!(), one!(), (Kilo * Gram * Meter * Meter) / (Second * Second * Ampere * Ampere)),

            // Amount of substance
            Mole => (zero!(), one!(), Mole),

            // Luminous intensity
            Candela => (zero!(), one!(), Candela),

            // Electric current
            Ampere => (zero!(), one!(), Ampere),

            // Magnetic flux
            Weber => (zero!(), one!(), (Kilo * Gram * Meter * Meter) / (Second * Second * Ampere)),

            // Length
            Meter => (zero!(), one!(), Meter),
            AU => (zero!(), ratio!(149597870691i64, 1), Meter),
            Inch => (zero!(), ratio!(127, 5000), Meter),
            Feet => (zero!(), ratio!(381, 1250), Meter),
            Yard => (zero!(), ratio!(1143, 1250), Meter),
            Mile => (zero!(), ratio!(201168, 125), Meter),
            NauticalMile => (zero!(), ratio!(1852, 1), Meter),
            LightYear => (zero!(), ratio!(9460730472580800i64, 1), Meter),
            Parsec => (zero!(), ratio!(30857000000000000i64, 1), Meter),

            // Electric Charge
            Coulomb => (zero!(), one!(), Second * Ampere),

            // Mass
            Gram => (zero!(), one!(), Gram),
            Tonne => (zero!(), ratio!(1000000, 1), Gram),
            Dram => (zero!(), ratio!(17718451953i64, 10_000_000_000i64), Gram),
            Ounce => (zero!(), ratio!(45_359_237i64, 1_600_000i64), Gram),
            Pound => (zero!(), ratio!(45359237, 100_000), Gram),

            // Electric capacitance
            Farad => (zero!(), one!(), (Second * Second * Second * Second * Ampere * Ampere) / (Kilo * Gram * Meter * Meter)),

            // Time
            Second => (zero!(), one!(), Second),
            Minute => (zero!(), ratio!(60, 1), Second),
            Hour => (zero!(), ratio!(3600, 1), Second),
            Day => (zero!(), ratio!(86400, 1), Second),
            Month => (zero!(), ratio!(2629746, 1), Second),
            Year => (zero!(), ratio!(31557600, 1), Second),

            // SI modifiers
            Quecto => (zero!(), ratio!(1, 1_000_000_000_000_000_000_000_000_000_000i128), UNITLESS),
            Ronto => (zero!(), ratio!(1, 1_000_000_000_000_000_000_000_000_000i128), UNITLESS),
            Yocto => (zero!(), ratio!(1, 1_000_000_000_000_000_000_000_000i128), UNITLESS),
            Zepto => (zero!(), ratio!(1, 1_000_000_000_000_000_000_000i128), UNITLESS),
            Atto => (zero!(), ratio!(1, 1_000_000_000_000_000_000i128), UNITLESS),
            Femto => (zero!(), ratio!(1, 1_000_000_000_000_000i64), UNITLESS),
            Pico => (zero!(), ratio!(1, 1_000_000_000_000i64), UNITLESS),
            Nano => (zero!(), ratio!(1, 1_000_000_000), UNITLESS),
            Micro => (zero!(), ratio!(1, 1_000_000), UNITLESS),
            Milli => (zero!(), ratio!(1, 1_000), UNITLESS),
            Centi => (zero!(), ratio!(1, 100), UNITLESS),
            Deci => (zero!(), ratio!(1, 10), UNITLESS),
            Hecto => (zero!(), ratio!(100, 1), UNITLESS),
            Kilo => (zero!(), ratio!(1_000, 1), UNITLESS),
            Mega => (zero!(), ratio!(1_000_000, 1), UNITLESS),
            Giga => (zero!(), ratio!(1_000_000_000, 1), UNITLESS),
            Tera => (zero!(), ratio!(1_000_000_000_000i64, 1), UNITLESS),
            Peta => (zero!(), ratio!(1_000_000_000_000_000i64, 1), UNITLESS),
            Exa => (zero!(), ratio!(1_000_000_000_000_000_000i128, 1), UNITLESS),
            Zetta => (zero!(), ratio!(1_000_000_000_000_000_000_000i128, 1), UNITLESS),
            Yotta => (zero!(), ratio!(1_000_000_000_000_000_000_000_000i128, 1), UNITLESS),
            Ronna => (zero!(), ratio!(1_000_000_000_000_000_000_000_000_000i128, 1), UNITLESS),
            Quetta => (zero!(), ratio!(1_000_000_000_000_000_000_000_000_000_000i128, 1), UNITLESS),

            // IEC binary modifiers
            Kibi => (zero!(), ratio!(1024, 1), UNITLESS),
            Mebi => (zero!(), ratio!(1048576, 1), UNITLESS),
            Gibi => (zero!(), ratio!(1073741824, 1), UNITLESS),
            Tebi => (zero!(), ratio!(1099511627776i64, 1), UNITLESS),
            Pebi => (zero!(), ratio!(1125899906842624i64, 1), UNITLESS),
            Exbi => (zero!(), ratio!(1152921504606846976i64, 1), UNITLESS),

            // Compound
            Compound(numerator, denominator) => {
                let mut offset = zero!();
                let mut slope = one!();
                let mut new_numerator = Vec::new();
                let mut new_denominator = Vec::new();

                for u in numerator {
                    let (n_offset, n_slope, n_unit) = u.to_si_units();
                    offset += n_offset;
                    // Multiply by the new slope without reducing the fraction
                    slope = BigRational::new_raw(slope.numer() * n_slope.numer(), slope.denom() * n_slope.denom());
                    new_numerator.push(n_unit);
                }

                for u in denominator {
                    let (n_offset, n_slope, n_unit) = u.to_si_units();
                    offset += n_offset;
                    // Divide by the new slope without reducing the fraction
                    slope = BigRational::new_raw(slope.numer() * n_slope.denom(), slope.denom() * n_slope.numer());
                    new_denominator.push(n_unit);
                }

                (offset, slope.reduced(), Compound(new_numerator, new_denominator).simplify())
            }
        }
    }


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

    /// Returns the symbol for a unit. E.g "m" for Meter.
    ///
    /// This method respects the order in which units are added to a compound unit.
    /// ```
    /// # use tantalum_unit::c;
    /// # use tantalum_unit::unit::Unit;
    /// use tantalum_unit::unit::Unit::*;
    ///
    /// ((Volt * Ampere) / Second).symbol(); // Returns VA/s
    /// ((Ampere * Volt) / Second).symbol(); // Returns AV/s
    /// ```
    pub fn symbol(&self) -> String {
        use Unit::*;
        match self {
            // force
            Newton => "N".to_owned(),

            // energy
            Joule => "J".to_owned(),

            // electric_resistance
            Ohm => "ohm".to_owned(),

            // frequency
            Hertz => "Hz".to_owned(),

            // voltage
            Volt => "V".to_owned(),

            // temperature
            Kelvin => "K".to_owned(),
            Celsius => "C".to_owned(),
            Fahrenheit => "F".to_owned(),

            // area
            Hectare => "ha".to_owned(),

            // magnetic_field_strength
            Tesla => "T".to_owned(),

            // bits
            Bit => "b".to_owned(),
            Byte => "B".to_owned(),

            // electric_conductance
            Siemens => "S".to_owned(),

            // power
            Watt => "W".to_owned(),

            // volume
            Liter => "L".to_owned(),
            CubicInch => "in^3".to_owned(),
            CubicFeet => "ft^3".to_owned(),
            CubicYard => "yd^3".to_owned(),
            Pint => "pt".to_owned(),
            Quart => "qt".to_owned(),
            Gallon => "gal".to_owned(),

            // pressure
            Pascal => "Pa".to_owned(),

            // inductance
            Henry => "H".to_owned(),

            // si_modifiers
            Quecto => "q".to_owned(),
            Ronto => "r".to_owned(),
            Yocto => "y".to_owned(),
            Zepto => "z".to_owned(),
            Atto => "a".to_owned(),
            Femto => "f".to_owned(),
            Pico => "p".to_owned(),
            Nano => "n".to_owned(),
            Micro => "µ".to_owned(),
            Milli => "m".to_owned(),
            Centi => "c".to_owned(),
            Deci => "d".to_owned(),
            Hecto => "h".to_owned(),
            Kilo => "k".to_owned(),
            Mega => "M".to_owned(),
            Giga => "G".to_owned(),
            Tera => "T".to_owned(),
            Peta => "P".to_owned(),
            Exa => "E".to_owned(),
            Zetta => "Z".to_owned(),
            Yotta => "Y".to_owned(),
            Ronna => "R".to_owned(),
            Quetta => "Q".to_owned(),

            // amount_of_substance
            Mole => "mol".to_owned(),

            // luminous_intensity
            Candela => "cd".to_owned(),

            // electric_current
            Ampere => "A".to_owned(),

            // magnetic_flux
            Weber => "Wb".to_owned(),

            // binary_modifiers
            Kibi => "Ki".to_owned(),
            Mebi => "Mi".to_owned(),
            Gibi => "Gi".to_owned(),
            Tebi => "Ti".to_owned(),
            Pebi => "Pi".to_owned(),
            Exbi => "Ei".to_owned(),

            // length
            Meter => "m".to_owned(),
            AU => "ua".to_owned(),
            Inch => "in".to_owned(),
            Feet => "ft".to_owned(),
            Yard => "yd".to_owned(),
            Mile => "mi".to_owned(),
            NauticalMile => "nmi".to_owned(),
            LightYear => "ly".to_owned(),
            Parsec => "pc".to_owned(),

            // electric_charge
            Coulomb => "C".to_owned(),

            // mass
            Gram => "g".to_owned(),
            Tonne => "t".to_owned(),
            Dram => "dr".to_owned(),
            Ounce => "oz".to_owned(),
            Pound => "lb".to_owned(),

            // electric_capacitance
            Farad => "F".to_owned(),

            // time
            Second => "s".to_owned(),
            Minute => "min".to_owned(),
            Hour => "h".to_owned(),
            Day => "d".to_owned(),
            Month => "mo".to_owned(),
            Year => "yr".to_owned(),

            // compound
            Compound(n, d) => {
                if n.is_empty() && d.is_empty() {
                    "".to_owned()
                } else {
                    fn count_units(units: &[Unit]) -> IndexMap<String, usize> {
                        let mut counts = IndexMap::new();
                        for unit in units {
                            let symbol = unit.symbol();
                            *counts.entry(symbol).or_insert(0) += 1;
                        }
                        counts
                    }

                    fn format_units(counts: IndexMap<String, usize>) -> String {
                        counts
                            .into_iter()
                            .map(|(symbol, count)| {
                                if count > 1 {
                                    format!("{}^{}", symbol, count)
                                } else {
                                    symbol
                                }
                            })
                            .collect::<Vec<String>>()
                            .join("")
                    }

                    let numerator_counts = count_units(n);
                    let denominator_counts = count_units(d);

                    let numerator = format_units(numerator_counts);
                    let denominator = format_units(denominator_counts);

                    if numerator.is_empty() {
                        format!("1/{}", denominator)
                    } else if denominator.is_empty() {
                        numerator
                    } else {
                        format!("{}/{}", numerator, denominator)
                    }
                }
            }
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
        let result = c!(c!(Meter; Second); Second).simplify();
        assert_eq!(result, c!(Meter; Second, Second));

        let result = c!(c!(c!(Watt; Joule); Second); c!(Meter; Second)).simplify();
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
