use criterion::{black_box, criterion_group, criterion_main, Criterion};
use creature_feature::bench_criterionized::{benchmark_for_vec_of_2_length_array_slices, benchmark_for_vec_of_2_length_arrays};

fn criterion_benchmark(c: &mut Criterion) {
    let az = "abcdefghijklmnopqrstuvuxyz1$n34567890!@#$%^&*()";
    let mut bigstring = "".to_owned();
    for _ in 0..100 {
        bigstring.push_str(&az);
    }
    let bigstring = bigstring.as_str();

    c.bench_function(
        "benchmark_for_vec_of_2_length_array_slices()",
        |b| b.iter(
                    || benchmark_for_vec_of_2_length_array_slices(black_box(bigstring))
                  )
    );

    c.bench_function(
        "benchmark_for_vec_of_2_length_arrays()",
        |b| b.iter(
                    || benchmark_for_vec_of_2_length_arrays(black_box(bigstring))
                  )
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
