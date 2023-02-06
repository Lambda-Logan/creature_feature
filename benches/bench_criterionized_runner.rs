use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};

use creature_feature::HashedAs;
use creature_feature::ftzrs::{n_slice, n_gram};
use creature_feature::traits::Ftzr; // because of the `featurize` method
use creature_feature::tokengroup::chars_of;


fn benchmarks_on_stringslice_input(c_manager: &mut Criterion) {
    let az = "abcdefghijklmnopqrstuvuxyz1$n34567890!@#$%^&*()";
    let mut bigstring = "".to_owned();
    for _ in 0..100 {
        bigstring.push_str(&az);
    }
    let big_strslice = bigstring.as_str();

    let mut comparison_group_strslice_input = c_manager.benchmark_group("Comparison group of ﹠str-input");

    // Note also the ampersand display problem: ASCII ampersand (&, i.e. U+0026) not appearing on plot diagram labels.
    // Thus, at each `BechmarkId` label, I use the `small ampersand' ﹠(U+FE60) instead!
    macro_rules! assemble_benchmarks_on_stringslice_input {
        ($n: expr) => {{
            // The refactored benches out of the old benchmark module `bench.rs`:
            comparison_group_strslice_input.bench_with_input( // from old bench #1
                BenchmarkId::new("n_slice + Vec<﹠str> (from ﹠str)", $n),
                &$n,
                |b_timer, ref_n| b_timer.iter(
                    || {
                        let vec_of_string_slices: Vec<&str> = n_slice(*ref_n).featurize(black_box(big_strslice));
                        vec_of_string_slices.len()
                    }
                )
            );
            comparison_group_strslice_input.bench_with_input( // from old bench #2
                BenchmarkId::new("n_slice + Vec<HashedAs<u64>> (from ﹠str)", $n), // note the ampersand
                &$n,
                |b_timer, ref_n| b_timer.iter(
                    || {
                        let vec_of_hashes: Vec<HashedAs<u64>> = n_slice(*ref_n).featurize(black_box(big_strslice));
                        vec_of_hashes.len()
                    }
                )
            );
            comparison_group_strslice_input.bench_with_input( // from old bench #3
                BenchmarkId::new("n_gram + Vec<HashedAs<u64>> (from ﹠str)", $n), // note the ampersand
                &$n,
                |b_timer, _| b_timer.iter(
                    || {
                        let vec_of_hashes: Vec<HashedAs<u64>> = n_gram::<$n>().featurize(black_box(big_strslice));
                        vec_of_hashes.len()
                    }
                )
            );
            comparison_group_strslice_input.bench_with_input( // from old bench #4
                BenchmarkId::new("n_slice + Vec<HashedAs<u16>> (from ﹠str)", $n), // note the ampersand
                &$n,
                |b_timer, ref_n| b_timer.iter(
                    || {
                        let vec_of_hashes: Vec<HashedAs<u16>> = n_slice(*ref_n).featurize(black_box(big_strslice));
                        vec_of_hashes.len()
                    }
                )
            );
            comparison_group_strslice_input.bench_with_input( // from old bench #5
                BenchmarkId::new("n_gram + Vec<HashedAs<u16>> (from ﹠str)", $n), // note the ampersand
                &$n,
                |b_timer, _| b_timer.iter(
                    || {
                        let vec_of_hashes: Vec<HashedAs<u16>> = n_gram::<$n>().featurize(black_box(big_strslice));
                        vec_of_hashes.len()
                    }
                )
            );
            comparison_group_strslice_input.bench_with_input( // from old bench #6
                BenchmarkId::new("n_slice + Vec<﹠[u8]> (from ﹠str)", $n), // note the ampersand
                &$n,
                |b_timer, ref_n| b_timer.iter(
                    || {
                        let vec_of_n_length_array_slices: Vec<&[u8]> = n_slice(*ref_n).featurize(black_box(big_strslice));
                        vec_of_n_length_array_slices.len()
                    }
                )
            );
            comparison_group_strslice_input.bench_with_input( // from old bench #7
                BenchmarkId::new("n_gram + Vec<[u8; N]> (from ﹠str)", $n), // note the ampersand
                &$n,
                |b_timer, _| b_timer.iter(
                    || {
                        let vec_of_n_length_arrays: Vec<[u8; $n]> = n_gram::<$n>().featurize(black_box(big_strslice));
                        vec_of_n_length_arrays.len()
                    }
                )
            );


        }}
    }

    assemble_benchmarks_on_stringslice_input!(  2);
    assemble_benchmarks_on_stringslice_input!(  4);
    assemble_benchmarks_on_stringslice_input!(  8);
    assemble_benchmarks_on_stringslice_input!( 16);
    assemble_benchmarks_on_stringslice_input!( 32);
    assemble_benchmarks_on_stringslice_input!( 64);
    assemble_benchmarks_on_stringslice_input!(128);

    comparison_group_strslice_input.finish();
}


fn benchmarks_on_charvec_input(c_manager: &mut Criterion) {
    let az = "abcdefghijklmnopqrstuvuxyz1$n34567890!@#$%^&*()";
    let mut bigstring = "".to_owned();
    for _ in 0..100 {
        bigstring.push_str(&az);
    }
    let big_strslice = bigstring.as_str();
    let big_charvec = chars_of(big_strslice);

    let mut comparison_group_charvec_input = c_manager.benchmark_group("Comparison group of Vec<char>-input");

    // Note also the ampersand display problem: ASCII ampersand (&, i.e. U+0026) not appearing on plot diagram labels.
    // Thus, at each `BechmarkId` label, I use the `small ampersand' ﹠(U+FE60) instead!
    macro_rules! assemble_benchmarks_on_charvec_input {
        ($n: expr) => {{
            // The refactored benches out of the old benchmark module `bench.rs`:

            comparison_group_charvec_input.bench_with_input( // from old bench #8
                BenchmarkId::new("n_gram + Vec<[char; N]> (from Vec<char>)", $n),
                &$n,
                |b_timer, _| b_timer.iter(
                    || {
                        let vec_of_n_length_arrays: Vec<[char; $n]> = n_gram::<$n>().featurize(black_box(&big_charvec));
                        vec_of_n_length_arrays.len()
                    }
                )
            );
            comparison_group_charvec_input.bench_with_input( // from old bench #9
                BenchmarkId::new("n_slice + Vec<﹠[char]> (from Vec<char>)", $n), // note the ampersand
                &$n,
                |b_timer, ref_n| b_timer.iter(
                    || {
                        let vec_of_n_length_array_slices: Vec<&[char]> = n_slice(*ref_n).featurize(black_box(&big_charvec));
                        vec_of_n_length_array_slices.len()
                    }
                )
            );
            comparison_group_charvec_input.bench_with_input( // from old bench #10
                BenchmarkId::new("n_gram + Vec<HashedAs<u64>> (from Vec<char>)", $n),
                &$n,
                |b_timer, _| b_timer.iter(
                    || {
                        let vec_of_hashes: Vec<HashedAs<u64>> = n_gram::<$n>().featurize(black_box(&big_charvec));
                        vec_of_hashes.len()
                    }
                )
            );
            comparison_group_charvec_input.bench_with_input( // from old bench #11
                BenchmarkId::new("n_slice + Vec<HashedAs<u64>> (from Vec<char>)", $n), // note the ampersand
                &$n,
                |b_timer, ref_n| b_timer.iter(
                    || {
                        let vec_of_hashes: Vec<HashedAs<u64>> = n_slice(*ref_n).featurize(black_box(&big_charvec));
                        vec_of_hashes.len()
                    }
                )
            );
        }}
    }

    assemble_benchmarks_on_charvec_input!(  2);
    assemble_benchmarks_on_charvec_input!(  4);
    assemble_benchmarks_on_charvec_input!(  8);
    assemble_benchmarks_on_charvec_input!( 16);
    assemble_benchmarks_on_charvec_input!( 32);
    assemble_benchmarks_on_charvec_input!( 64);
    assemble_benchmarks_on_charvec_input!(128);

    comparison_group_charvec_input.finish();
}

fn benchmarks_mixed_crosscutting_group(c_manager: &mut Criterion) {
    let az = "abcdefghijklmnopqrstuvuxyz1$n34567890!@#$%^&*()";
    let mut bigstring = "".to_owned();
    for _ in 0..100 {
        bigstring.push_str(&az);
    }
    let big_strslice = bigstring.as_str();
    let big_charvec = chars_of(big_strslice);

    let mut comparison_group_mixed_crosscutting = c_manager.benchmark_group("Comparison group for mixed crosscutting");

    // Note also the ampersand display problem: ASCII ampersand (&, i.e. U+0026) not appearing on plot diagram labels.
    // Thus, at each `BechmarkId` label, I use the `small ampersand' ﹠(U+FE60) instead!
    macro_rules! assemble_benchmarks_mixed_crosscutting {
        ($n: expr) => {{
            // The refactored benches out of the old benchmark module `bench.rs`:

            // Input &str:

            comparison_group_mixed_crosscutting.bench_with_input( // from old bench #1
                BenchmarkId::new("n_slice + Vec<﹠str> (from ﹠str)", $n),
                &$n,
                |b_timer, ref_n| b_timer.iter(
                    || {
                        let vec_of_string_slices: Vec<&str> = n_slice(*ref_n).featurize(black_box(big_strslice));
                        vec_of_string_slices.len()
                    }
                )
            );
            comparison_group_mixed_crosscutting.bench_with_input( // from old bench #6
                BenchmarkId::new("n_slice + Vec<﹠[u8]> (from ﹠str)", $n), // note the ampersand
                &$n,
                |b_timer, ref_n| b_timer.iter(
                    || {
                        let vec_of_n_length_array_slices: Vec<&[u8]> = n_slice(*ref_n).featurize(black_box(big_strslice));
                        vec_of_n_length_array_slices.len()
                    }
                )
            );
            comparison_group_mixed_crosscutting.bench_with_input( // from old bench #5
                BenchmarkId::new("n_gram + Vec<HashedAs<u16>> (from ﹠str)", $n), // note the ampersand
                &$n,
                |b_timer, _| b_timer.iter(
                    || {
                        let vec_of_hashes: Vec<HashedAs<u16>> = n_gram::<$n>().featurize(black_box(big_strslice));
                        vec_of_hashes.len()
                    }
                )
            );
            comparison_group_mixed_crosscutting.bench_with_input( // from old bench #4
                BenchmarkId::new("n_slice + Vec<HashedAs<u16>> (from ﹠str)", $n), // note the ampersand
                &$n,
                |b_timer, ref_n| b_timer.iter(
                    || {
                        let vec_of_hashes: Vec<HashedAs<u16>> = n_slice(*ref_n).featurize(black_box(big_strslice));
                        vec_of_hashes.len()
                    }
                )
            );

            // Input Vec<char>:

            comparison_group_mixed_crosscutting.bench_with_input( // from old bench #10
                BenchmarkId::new("n_gram + Vec<HashedAs<u64>> (from Vec<char>)", $n),
                &$n,
                |b_timer, _| b_timer.iter(
                    || {
                        let vec_of_hashes: Vec<HashedAs<u64>> = n_gram::<$n>().featurize(black_box(&big_charvec));
                        vec_of_hashes.len()
                    }
                )
            );
            comparison_group_mixed_crosscutting.bench_with_input( // from old bench #9
                BenchmarkId::new("n_slice + Vec<﹠[char]> (from Vec<char>)", $n), // note the ampersand
                &$n,
                |b_timer, ref_n| b_timer.iter(
                    || {
                        let vec_of_n_length_array_slices: Vec<&[char]> = n_slice(*ref_n).featurize(black_box(&big_charvec));
                        vec_of_n_length_array_slices.len()
                    }
                )
            );
        }}
    }

    assemble_benchmarks_mixed_crosscutting!(  2);
    assemble_benchmarks_mixed_crosscutting!(  4);
    assemble_benchmarks_mixed_crosscutting!(  8);
    assemble_benchmarks_mixed_crosscutting!( 16);
    assemble_benchmarks_mixed_crosscutting!( 32);
    assemble_benchmarks_mixed_crosscutting!( 64);
    assemble_benchmarks_mixed_crosscutting!(128);

    comparison_group_mixed_crosscutting.finish();
}

criterion_group!(benches, benchmarks_on_stringslice_input, benchmarks_on_charvec_input, benchmarks_mixed_crosscutting_group);
criterion_main!(benches);
