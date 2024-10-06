use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tantalum_unit::c;
use tantalum_unit::quantity::Quantity;
use tantalum_unit::unit::Unit::{Coulomb, Joule, Kilo, Meter, Milli, Newton, Second, Volt, Watt};
use tantalum_unit::unit::Unit;

fn mul(i: i64) -> Quantity {
    let a = Quantity::from_i64_with_unit(15 * i, Joule);
    let b = Quantity::from_i64_with_unit(i + 1, Second);
    a * b
}

fn div(i: i64) -> Quantity {
    let a = Quantity::from_i64_with_unit(15 * i, Joule);
    let b = Quantity::from_i64_with_unit(i + 1, Second);
    a / b
}

fn add(i: i64) -> Result<Quantity, ()> {
    let a = Quantity::from_i64_with_unit(15 * i, Joule);
    let b = Quantity::from_i64_with_unit(i + 1, Joule);
    a + b
}

fn sub(i: i64) -> Result<Quantity, ()> {
    let a = Quantity::from_i64_with_unit(15 * i, Joule);
    let b = Quantity::from_i64_with_unit(i + 1, Joule);
    a - b
}

fn convert(i: i64) -> (Quantity, Quantity, Quantity) {
    let a = Quantity::from_i64_with_unit(15 + i, c!(Joule; Second));
    let result_a = a.clone().convert_to(c!(Milli, Watt;)).unwrap();
    let result_b = a.clone().convert_to(c!(Kilo, Coulomb, Volt; Second)).unwrap();
    let result_c = a.convert_to(c!(Newton, Meter; Second)).unwrap();
    (result_a, result_b, result_c)
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("mul", |b| b.iter(|| mul(black_box(20))));
    c.bench_function("div", |b| b.iter(|| div(black_box(20))));
    c.bench_function("add", |b| b.iter(|| add(black_box(20))));
    c.bench_function("sub", |b| b.iter(|| sub(black_box(20))));
    c.bench_function("convert", |b| b.iter(|| convert(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
