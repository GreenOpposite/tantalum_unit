#[macro_export]
macro_rules! define_units {
    ($($name:ident, $display_name:expr, $symbol:expr, $offset:expr, $slope:expr, $si_units:expr);*) => {
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
            $($name,)*
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
                use crate::scalable_integer::BigRational;

                self = self.flatten();
                match self {
                    $(Unit::$name => ($offset, $slope, $si_units),)*

                    Compound(numerator, denominator) => {
                        let mut offset = zero!();
                        let mut slope = one!();
                        let mut new_numerator = Vec::new();
                        let mut new_denominator = Vec::new();

                        for u in numerator {
                            let (n_offset, n_slope, n_unit) = u.to_si_units();
                            offset += n_offset;
                            // Multiply by the new slope without reducing the fraction
                            slope = BigRational::new_raw(slope.numer().clone() * n_slope.numer().clone(), slope.denom().clone() * n_slope.denom().clone());
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
                    $($name => $symbol.to_owned(),)*
                    Compound(n, d) => {
                        if n.is_empty() & &d.is_empty() {
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

            pub fn name(&self) -> String {
                use Unit::*;
                match self {
                    $($name => $display_name.to_owned(),)*

                    Compound(n, d) => {
                        if n.is_empty() & &d.is_empty() {
                            "".to_owned()
                        } else {
                            fn count_units(units: &[Unit]) -> IndexMap<String, usize> {
                                let mut counts = IndexMap::new();
                                for unit in units {
                                    let mut name = unit.name();
                                    if !unit.is_modifier() { name += " "; }
                                    *counts.entry(name).or_insert(0) += 1;
                                }
                                counts
                            }

                            fn format_units(counts: IndexMap<String, usize>) -> String {
                                counts
                                    .into_iter()
                                    .map(|(symbol, count)| {
                                        if count > 3 {
                                            format!("{} to the {}", symbol, count)
                                        } else if count == 3 {
                                            format!("cubic {}", symbol)
                                        } else if count == 2 {
                                            format!("square {}", symbol)
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
                                format!("reciprocal {}", denominator)
                            } else if denominator.is_empty() {
                                numerator
                            } else {
                                format!("{}per {}", numerator, denominator)
                            }
                        }
                    }
                }.trim_end().to_owned()
            }
        }
    };
}