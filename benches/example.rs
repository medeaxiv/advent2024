use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn bench_fibonacci(c: &mut Criterion) {
    c.bench_function("example iter 20", |b| {
        b.iter(|| fibonacci_iter(black_box(20)))
    });

    c.bench_function("example rec 20", |b| {
        b.iter(|| fibonacci_rec(black_box(20)))
    });
}

criterion_group!(benches, bench_fibonacci);
criterion_main!(benches);

fn fibonacci_iter(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut a = 1;
            let mut b = 1;

            for _ in 2..n {
                let next = a + b;
                a = b;
                b = next;
            }

            b
        }
    }
}

fn fibonacci_rec(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci_rec(n - 2) + fibonacci_rec(n - 1),
    }
}
