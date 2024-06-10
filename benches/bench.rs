use std::{fs::File, io::{BufReader, Read}};

use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use p3::{p1,  p2};


fn bench_p1(c: &mut Criterion) {
    c.bench_function("p1", |b| b.iter_batched(|| {
        let mut f = File::open("input.txt").expect("can't open file");
        let mut buf = String::new();
        f.read_to_string(&mut buf).expect("can't read file");
        buf
    }, | f| p1(&mut BufReader::new(f.as_bytes())), BatchSize::SmallInput));
}

fn bench_p2(c: &mut Criterion) {
    c.bench_function("p2", |b| b.iter_batched(|| {
        let mut f = File::open("input.txt").expect("can't open file");
        let mut buf = String::new();
        f.read_to_string(&mut buf).expect("can't read file");
        buf
    }, |f| p2(&mut BufReader::new(f.as_bytes())), BatchSize::SmallInput));
}


criterion_group!(benches, bench_p1, bench_p2);
criterion_main!(benches);
