use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use creature_feature::ftzrs::{n_slice, n_gram};
use creature_feature::traits::Ftzr; // because of the `featurize` method


fn big_comparison_benchmark(c_manager: &mut Criterion) {
    let az = "abcdefghijklmnopqrstuvuxyz1$n34567890!@#$%^&*()";
    let mut bigstring = "".to_owned();
    for _ in 0..100 {
        bigstring.push_str(&az);
    }
    let bigstring = bigstring.as_str();

    let mut comparison_group = c_manager.benchmark_group("Comparison group");

    macro_rules! comparison_item_benchmark { // Note also the ampersand display problem, see comments below:
        ($n: expr) => {{
            comparison_group.bench_with_input(
                BenchmarkId::new("n_slice + Vec<﹠[u8]> (from ﹠str)", $n), // the `small ampersand' ﹠(U+FE60), instead of the ASCII & (U+0026)
                &$n,
                |b_timer, ref_n| b_timer.iter(
                    || {
                        let vec_of_n_length_array_slices: Vec<&[u8]> = n_slice(*ref_n).featurize(black_box(bigstring));
                        vec_of_n_length_array_slices.len()
                    }
                )
            );
            comparison_group.bench_with_input(
                BenchmarkId::new("n_gram + Vec<[u8,N]> (from ﹠str)", $n), // the `small ampersand' ﹠(U+FE60), instead of the ASCII & (U+0026)
                &$n,
                |b_timer, _| b_timer.iter(
                    || {
                        let vec_of_n_length_arrays: Vec<[u8; $n]> = n_gram::<$n>().featurize(black_box(bigstring));
                        vec_of_n_length_arrays.len()
                    }
                )
            );
        }}
    }

    comparison_item_benchmark!(  2);
    comparison_item_benchmark!(  4);
    comparison_item_benchmark!(  8);
    comparison_item_benchmark!( 16);
    comparison_item_benchmark!( 32);
    comparison_item_benchmark!( 64);
    comparison_item_benchmark!(128);

    comparison_group.finish();
}

criterion_group!(benches, big_comparison_benchmark);
criterion_main!(benches);
