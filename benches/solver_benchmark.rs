use criterion::{black_box, criterion_group, criterion_main, Criterion};
use slide_puzzle::solver::{divide_and_conquer::DacPuzzleSolver, optimal::find_swap_order};

lazy_static::lazy_static! {
    static ref PUZZLE_3X3_16: (Vec<u8>, usize, usize) = {
        (vec![8, 5, 2, 1, 0, 7, 6, 4, 3], 3, 3)
    };
    static ref PUZZLE_4X4_10: (Vec<u8>, usize, usize) = {
        (vec![0, 1, 2, 3, 4, 5, 6, 7, 12, 8, 10, 11, 13, 14, 9, 15], 4, 4)
    };
    static ref PUZZLE_4X4_17: (Vec<u8>, usize, usize) = {
        (vec![0, 5, 1, 3, 8, 4, 2, 11, 12, 10, 7, 6, 9, 13, 15, 14], 4, 4)
    };
    static ref PUZZLE_4X4_20: (Vec<u8>, usize, usize) = {
        (vec![1, 2, 15, 5, 0, 9, 4, 3, 12, 10, 7, 6, 13, 8, 14, 11], 4, 4)
    };
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group100 = c.benchmark_group("100 samples");
    group100.sample_size(100);

    // 9.8k evaluations
    group100.bench_function("3x3 16 steps optimal", |b| {
        b.iter(|| {
            find_swap_order(
                black_box(&PUZZLE_3X3_16.0),
                black_box(PUZZLE_3X3_16.1),
                black_box(PUZZLE_3X3_16.2),
            )
        })
    });

    // 2.6k evaluations
    group100.bench_function("4x4 10 steps optimal", |b| {
        b.iter(|| {
            find_swap_order(
                black_box(&PUZZLE_4X4_10.0),
                black_box(PUZZLE_4X4_10.1),
                black_box(PUZZLE_4X4_10.2),
            )
        })
    });

    group100.bench_function("3x3 16 steps divide and conquer", |b| {
        b.iter(|| {
            let mut solver = DacPuzzleSolver::new(
                black_box(&PUZZLE_3X3_16.0),
                black_box(PUZZLE_3X3_16.1 as i32),
                black_box(PUZZLE_3X3_16.2 as i32),
            )
            .unwrap();
            solver.solve_puzzle().unwrap();
        });
    });

    group100.bench_function("4x4 10 steps divide and conquer", |b| {
        b.iter(|| {
            let mut solver = DacPuzzleSolver::new(
                black_box(&PUZZLE_4X4_10.0),
                black_box(PUZZLE_4X4_10.1 as i32),
                black_box(PUZZLE_4X4_10.2 as i32),
            )
            .unwrap();
            solver.solve_puzzle().unwrap();
        });
    });
    group100.finish();

    let mut group_slow = c.benchmark_group("10 samples");
    group_slow.sample_size(10);

    // // 642k evaluations
    group_slow.bench_function("4x4_17_steps optimal", |b| {
        b.iter(|| {
            find_swap_order(
                black_box(&PUZZLE_4X4_17.0),
                black_box(PUZZLE_4X4_17.1),
                black_box(PUZZLE_4X4_17.2),
            )
        })
    });

    group_slow.bench_function("4x4 10 steps divide and conquer", |b| {
        b.iter(|| {
            let mut solver = DacPuzzleSolver::new(
                black_box(&PUZZLE_4X4_17.0),
                black_box(PUZZLE_4X4_17.1 as i32),
                black_box(PUZZLE_4X4_17.2 as i32),
            )
            .unwrap();
            solver.solve_puzzle().unwrap();
        });
    });
    group_slow.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
