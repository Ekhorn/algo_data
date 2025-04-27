use std::time::Duration;

use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};

extern crate quick_sort;
use quick_sort::{quick_sort_v2, quick_sort_v3, quick_sort_v4, quick_sort_v5, quick_sort_v6};
use rand::Rng;

pub fn criterion_benchmark(c: &mut Criterion) {
    let unsorted = vec![0, 1, 3, 2, 5, 6, 1, 6, 25, 6, 2];
    let expected = vec![0, 1, 1, 2, 2, 3, 5, 6, 6, 6, 25];
    for sorted in vec![
        quick_sort_v2(unsorted.clone()),
        quick_sort_v3(unsorted.clone()),
        quick_sort_v4(unsorted.clone()),
    ] {
        assert!(sorted == expected);
    }
    let mut clone = unsorted.clone();
    quick_sort_v5(&mut clone);
    assert!(clone == expected);
    let mut clone = unsorted.clone();
    quick_sort_v6(&mut clone);
    assert!(clone == expected);

    let mut group = c.benchmark_group("quick_sort");
    group
        .warm_up_time(Duration::new(10, 0))
        // .measurement_time(Duration::new(10, 0))
        .sample_size(1_000);

    let mut rng = rand::rng();

    let rows: Vec<Vec<_>> = vec![
        (0..1e1 as u32).map(|_| rng.random_range(0..255)).collect(),
        (0..1e2 as u32).map(|_| rng.random_range(0..255)).collect(),
        (0..1e3 as u32).map(|_| rng.random_range(0..255)).collect(),
        (0..1e4 as u32).map(|_| rng.random_range(0..255)).collect(),
        (0..1e5 as u32).map(|_| rng.random_range(0..255)).collect(),
    ];
    for row in rows {
        let id = BenchmarkId::new("quick_sort_std", &row.len());
        group.bench_function(id, |b| b.iter(|| black_box(row.clone()).sort()));

        let id = BenchmarkId::new("quick_sort_v2", &row.len());
        group.bench_function(id, |b| b.iter(|| quick_sort_v2(black_box(row.clone()))));

        let id = BenchmarkId::new("quick_sort_v3", &row.len());
        group.bench_function(id, |b| b.iter(|| quick_sort_v3(black_box(row.clone()))));

        let id = BenchmarkId::new("quick_sort_v4", &row.len());
        group.bench_function(id, |b| b.iter(|| quick_sort_v4(black_box(row.clone()))));

        let id = BenchmarkId::new("quick_sort_v5", &row.len());
        group.bench_function(id, |b| {
            b.iter(|| quick_sort_v5(black_box(&mut row.clone())))
        });

        let id = BenchmarkId::new("quick_sort_v6", &row.len());
        group.bench_function(id, |b| {
            b.iter(|| quick_sort_v6(black_box(&mut row.clone())))
        });
    }

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
