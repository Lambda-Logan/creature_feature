use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use creature_feature::bench_criterionized::{benchmark_for_vec_of_n_length_array_slices, benchmark_for_vec_of_n_length_arrays};

fn big_comparison_benchmark(c_manager: &mut Criterion) {
    let az = "abcdefghijklmnopqrstuvuxyz1$n34567890!@#$%^&*()";
    let mut bigstring = "".to_owned();
    for _ in 0..100 {
        bigstring.push_str(&az);
    }
    let bigstring = bigstring.as_str();

    let mut comparison_group = c_manager.benchmark_group("Comparison group");

    for ref_n in [2, 4, 8, 16, 32, 64, 128].iter() {
        comparison_group.bench_with_input(
            BenchmarkId::new("benchmark_for_vec_of_n_length_array_slices", *ref_n),
            ref_n,
            |b_timer, ref_n| b_timer.iter(
                || benchmark_for_vec_of_n_length_array_slices(black_box(bigstring), *ref_n)
            )
        );

        comparison_group.bench_with_input(
            BenchmarkId::new("benchmark_for_vec_of_n_length_arrays", *ref_n),
            ref_n,
            |b_timer, ref_n| b_timer.iter(
                || benchmark_for_vec_of_n_length_arrays(black_box(bigstring), *ref_n)
            )
        );
    }

    comparison_group.finish();
}

criterion_group!(benches, big_comparison_benchmark);
criterion_main!(benches);
