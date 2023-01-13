use crate::ftzrs::{bislice, bigram};
use crate::accum_ftzr::Ftzr; // because of the `featurize` method

/// Benchmark #6: n_slice + Vec<&[u8]> (from &str)
pub fn benchmark_for_vec_of_2_length_array_slices(bigstring: &str) -> usize {
    let vec_of_2_length_array_slices: Vec<&[u8]> = bislice().featurize(bigstring);
    vec_of_2_length_array_slices.len()
}

/// Benchmark #7: n_gram + Vec<[u8; N]> (from &str)
pub fn benchmark_for_vec_of_2_length_arrays(bigstring: &str) -> usize {
    let vec_of_2_length_arrays: Vec<[u8; 2]> = bigram().featurize(bigstring);
    vec_of_2_length_arrays.len()
}
