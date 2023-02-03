use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};

use creature_feature::HashedAs;
use creature_feature::ftzrs::{n_slice, n_gram};
use creature_feature::traits::Ftzr; // because of the `featurize` method
use creature_feature::tokengroup::chars_of;


fn big_comparison_benchmark(c_manager: &mut Criterion) {
    let az = "abcdefghijklmnopqrstuvuxyz1$n34567890!@#$%^&*()";
    let mut bigstring = "".to_owned();
    for _ in 0..100 {
        bigstring.push_str(&az);
    }
    let bigstring = bigstring.as_str();

    let mut comparison_group = c_manager.benchmark_group("Comparison group");

    // Note also the ampersand display problem: ASCII ampersand (&, i.e. U+0026) not appearing on plot diagram labels.
    // Thus, at each `BechmarkId` label, I use the `small ampersand' ﹠(U+FE60) instead!
    macro_rules! comparison_item_benchmark {
        ($n: expr) => {{
            // The refactored benches out of the old benchmark module `bench.rs`:
            comparison_group.bench_with_input( // from old bench #1
                BenchmarkId::new("n_slice + Vec<﹠str> (from ﹠str)", $n),
                &$n,
                |b_timer, ref_n| b_timer.iter(
                    || {
                        let vec_of_string_slices: Vec<&str> = n_slice(*ref_n).featurize(black_box(bigstring));
                        vec_of_string_slices.len()
                    }
                )
            );
            comparison_group.bench_with_input( // from old bench #2
                BenchmarkId::new("n_slice + Vec<HashedAs<u64>> (from ﹠str)", $n), // note the ampersand
                &$n,
                |b_timer, ref_n| b_timer.iter(
                    || {
                        let vec_of_hashes: Vec<HashedAs<u64>> = n_slice(*ref_n).featurize(black_box(bigstring));
                        vec_of_hashes.len()
                    }
                )
            );
            comparison_group.bench_with_input( // from old bench #3
                BenchmarkId::new("n_gram + Vec<HashedAs<u64>> (from ﹠str)", $n), // note the ampersand
                &$n,
                |b_timer, _| b_timer.iter(
                    || {
                        let vec_of_hashes: Vec<HashedAs<u64>> = n_gram::<$n>().featurize(black_box(bigstring));
                        vec_of_hashes.len()
                    }
                )
            );
            comparison_group.bench_with_input( // from old bench #4
                BenchmarkId::new("n_slice + Vec<HashedAs<u16>> (from ﹠str)", $n), // note the ampersand
                &$n,
                |b_timer, ref_n| b_timer.iter(
                    || {
                        let vec_of_hashes: Vec<HashedAs<u16>> = n_slice(*ref_n).featurize(black_box(bigstring));
                        vec_of_hashes.len()
                    }
                )
            );
            comparison_group.bench_with_input( // from old bench #5
                BenchmarkId::new("n_gram + Vec<HashedAs<u16>> (from ﹠str)", $n), // note the ampersand
                &$n,
                |b_timer, _| b_timer.iter(
                    || {
                        let vec_of_hashes: Vec<HashedAs<u16>> = n_gram::<$n>().featurize(black_box(bigstring));
                        vec_of_hashes.len()
                    }
                )
            );
            comparison_group.bench_with_input( // from old bench #6
                BenchmarkId::new("n_slice + Vec<﹠[u8]> (from ﹠str)", $n), // note the ampersand
                &$n,
                |b_timer, ref_n| b_timer.iter(
                    || {
                        let vec_of_n_length_array_slices: Vec<&[u8]> = n_slice(*ref_n).featurize(black_box(bigstring));
                        vec_of_n_length_array_slices.len()
                    }
                )
            );
            comparison_group.bench_with_input( // from old bench #7
                BenchmarkId::new("n_gram + Vec<[u8; N]> (from ﹠str)", $n), // note the ampersand
                &$n,
                |b_timer, _| b_timer.iter(
                    || {
                        let vec_of_n_length_arrays: Vec<[u8; $n]> = n_gram::<$n>().featurize(black_box(bigstring));
                        vec_of_n_length_arrays.len()
                    }
                )
            );

            let chars = chars_of(bigstring);

            comparison_group.bench_with_input( // from old bench #8
                BenchmarkId::new("n_gram + Vec<[char; N]> (from Vec<char>)", $n),
                &$n,
                |b_timer, _| b_timer.iter(
                    || {
                        let vec_of_n_length_arrays: Vec<[char; $n]> = n_gram::<$n>().featurize(black_box(&chars));
                        vec_of_n_length_arrays.len()
                    }
                )
            );
            comparison_group.bench_with_input( // from old bench #9
                BenchmarkId::new("n_slice + Vec<﹠[char]> (from Vec<char>)", $n), // note the ampersand
                &$n,
                |b_timer, ref_n| b_timer.iter(
                    || {
                        let vec_of_n_length_array_slices: Vec<&[char]> = n_slice(*ref_n).featurize(black_box(&chars));
                        vec_of_n_length_array_slices.len()
                    }
                )
            );
            comparison_group.bench_with_input( // from old bench #10
                BenchmarkId::new("n_gram + Vec<HashedAs<u64>> (from Vec<char>)", $n),
                &$n,
                |b_timer, _| b_timer.iter(
                    || {
                        let vec_of_hashes: Vec<HashedAs<u64>> = n_gram::<$n>().featurize(black_box(&chars));
                        vec_of_hashes.len()
                    }
                )
            );
            comparison_group.bench_with_input( // from old bench #11
                BenchmarkId::new("n_slice + Vec<HashedAs<u64>> (from Vec<char>)", $n), // note the ampersand
                &$n,
                |b_timer, ref_n| b_timer.iter(
                    || {
                        let vec_of_hashes: Vec<HashedAs<u64>> = n_slice(*ref_n).featurize(black_box(&chars));
                        vec_of_hashes.len()
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
