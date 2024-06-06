use std::io::BufReader;

use criterion::{criterion_group, criterion_main, Criterion};

static INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("p1", |b| {
        b.iter(|| p3::p1(&mut BufReader::new(INPUT.as_bytes())))
    });
    c.bench_function("yt::p1", |b| {
        b.iter(|| p3::yt::p1(&mut BufReader::new(INPUT.as_bytes())))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
