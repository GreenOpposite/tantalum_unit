# 🚀 Tantalum Unit: Blazingly Fast[^1] Unit Math in Rust 🦀
# Overview
Tantalum Unit is a high-performance[^1] 🚀🚀🚀🚀 Rust 🦀🦀🦀🦀 crate for doing math with units. It leverages arbitrary precision numbers to provide accurate calculations and seamless conversions between units.

# Key Features
- Arbitrary Precision: Perform calculations with high precision using arbitrary precision numbers. 💪💪💪💪
- Unit Conversions: Convert from any unit to any other unit, as long as they are dimensionally compatible. ✅✅✅✅✅
- Blazingly Fast[^1]: Optimized for performance, Tantalum Unit is designed to handle complex calculations with ease. 🚀🚀🚀

# Example
```rust
let speed           = Quantity::from_i64_with_unit(60, Mile / Hour);
let more_speed      = Quantity::from_i64_with_unit(45, Mile / Hour);
let even_more_speed = Quantity::from_i64_with_unit(1, Milli * AU / Year);

let combined_speed  = speed + more_speed + even_more_speed;
let converted_speed = combined_speed.convert_to((Kilo * Meter) / Hour).unwrap();

println!("{converted_speed}");
```

# Better crates to use
[dimensioned](https://github.com/paholg/dimensioned) and [cpc](https://github.com/probablykasper/cpc) are almost certainly better choices.


[^1]: Arbitrary precision arithmetic is actually not that fast...

*This crate was inspired by [runits](https://github.com/jesse99/runits).*
