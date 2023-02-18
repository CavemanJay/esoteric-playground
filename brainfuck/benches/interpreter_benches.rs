#![allow(clippy::all, unused_imports, dead_code, unused_variables)]
use std::{env::current_dir, fs};

use brainfuck::engines::{self, *};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

fn bench_interpreters(c: &mut Criterion) {
    let mut group = c.benchmark_group("Interpreters");
    for prog_file in ["hello_world.bf", "hello_world2.bf"] {
        let prog = fs::read_to_string(current_dir().unwrap().join("data").join(prog_file))
            .expect("Failed to read input file");
        group.bench_with_input(BenchmarkId::new("basic", prog_file), &prog, |b, program| {
            b.iter(|| engines::WrappingEngine::run(program))
        });
        group.bench_with_input(
            BenchmarkId::new("wrapping", prog_file),
            &prog,
            |b, program| b.iter(|| engines::LinearEngine::run(program)),
        );
    }
    group.finish();
}

criterion_group!(benches, bench_interpreters);
criterion_main!(benches);
