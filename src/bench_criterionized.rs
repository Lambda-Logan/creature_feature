use crate::ftzrs::{n_slice, n_gram};
use crate::accum_ftzr::Ftzr; // because of the `featurize` method

/// Benchmark #6: n_slice + Vec<&[u8]> (from &str)
pub fn benchmark_for_vec_of_n_length_array_slices(bigstring: &str, n: usize) -> usize {
    let vec_of_n_length_array_slices: Vec<&[u8]> = n_slice(n).featurize(bigstring);
    vec_of_n_length_array_slices.len()
}

/// Benchmark #7: n_gram + Vec<[u8; N]> (from &str)
pub fn benchmark_for_vec_of_n_length_arrays(bigstring: &str, n: usize) -> usize {
    match n {
        2 => {
            let vec_of_n_length_arrays: Vec<[u8;   2]> = n_gram::<2>  ().featurize(bigstring);
            vec_of_n_length_arrays.len()
        },
        4 => {
            let vec_of_n_length_arrays: Vec<[u8;   4]> = n_gram::<4>  ().featurize(bigstring);
            vec_of_n_length_arrays.len()
        },
        8 => {
            let vec_of_n_length_arrays: Vec<[u8;   8]> = n_gram::<8>  ().featurize(bigstring);
            vec_of_n_length_arrays.len()
        },
        16 => {
            let vec_of_n_length_arrays: Vec<[u8;  16]> = n_gram::<16> ().featurize(bigstring);
            vec_of_n_length_arrays.len()
        },
        32 => {
            let vec_of_n_length_arrays: Vec<[u8;  32]> = n_gram::<32> ().featurize(bigstring);
            vec_of_n_length_arrays.len()
        },
        64 => {
            let vec_of_n_length_arrays: Vec<[u8;  64]> = n_gram::<64> ().featurize(bigstring);
            vec_of_n_length_arrays.len()
        },
        128 => {
            let vec_of_n_length_arrays: Vec<[u8; 128]> = n_gram::<128>().featurize(bigstring);
            vec_of_n_length_arrays.len()
        }
        _ => panic!("No matching")
    }
}
