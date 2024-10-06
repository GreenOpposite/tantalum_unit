//! Helper macros for constructing ```BigRational``` and ```Unit```.

/// Constructs a ```BigRational``` from two ints.
///
/// # Example:
/// ```
/// # use num::BigRational;
/// # use tantalum_unit::ratio;
/// let value: BigRational = ratio!(17, 3); // For the fraction 17/3
/// ```
#[macro_export]
macro_rules! ratio {
    ($a:expr, $b:expr) => {
        num::BigRational::new($a.into(), $b.into())
    };
}

/// A ```BigRational``` representing the number ```0```.
#[macro_export]
macro_rules! zero {
    () => {
        num::BigRational::zero()
    };
}

/// A ```BigRational``` representing the number ```1```.
#[macro_export]
macro_rules! one {
    () => {
        num::BigRational::one()
    };
}

/// A helper for constructing compound units.
///
/// # Example:
/// ```
/// # use tantalum_unit::c;
/// # use tantalum_unit::unit::Unit;
/// # use tantalum_unit::unit::Unit::{Joule, Second, Volt, Ampere};
/// let joules_per_second: Unit = c!(Joule; Second);
/// let volt_ampere_second: Unit = c!(Volt, Ampere, Second;);
/// let per_ampere: Unit = c!(;Ampere);
/// ```
#[macro_export]
macro_rules! c {
    () => {
        Unit::Compound(vec![], vec![])
    };

    ($($a:expr),*;) => {
        Unit::Compound(vec![$($a),*], vec![])
    };

    (; $($b:expr),*) => {
        Unit::Compound(vec![], vec![$($b),*])
    };

    ($($a:expr),*; $($b:expr),*) => {
        Unit::Compound(vec![$($a),*], vec![$($b),*])
    };
}

/// Constructs a ```BigRational``` from an int.
///
/// # Example:
/// ```
/// # use num::BigRational;
/// # use tantalum_unit::int;
/// let eight: BigRational = int!(8);
/// ```
#[macro_export]
macro_rules! int {
    ($value:expr) => {
        num::BigRational::from_integer($value.into())
    };
}

