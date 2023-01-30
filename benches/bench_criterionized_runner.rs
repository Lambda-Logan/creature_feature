use criterion::{black_box, criterion_group, criterion_main, Criterion};
use creature_feature::bench_criterionized::{benchmark_for_vec_of_2_length_array_slices, benchmark_for_vec_of_2_length_arrays};

fn big_comparison_benchmark(c_manager: &mut Criterion) {
    let az = "abcdefghijklmnopqrstuvuxyz1$n34567890!@#$%^&*()";
    let mut bigstring = "".to_owned();
    for _ in 0..100 {
        bigstring.push_str(&az);
    }
    let bigstring = bigstring.as_str();

    let mut comparison_group = c_manager.benchmark_group("Comparison group");

    comparison_group.bench_function(
        "benchmark_for_vec_of_2_length_array_slices()",
        |b_timer| b_timer.iter(
            || benchmark_for_vec_of_2_length_array_slices(black_box(bigstring))
        )
    );

    comparison_group.bench_function(
        "benchmark_for_vec_of_2_length_arrays()",
        |b_timer| b_timer.iter(
            || benchmark_for_vec_of_2_length_arrays(black_box(bigstring))
        )
    );

    comparison_group.finish();
}

criterion_group!(benches, big_comparison_benchmark);
criterion_main!(benches);
