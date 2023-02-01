use crate::ftzrs::{n_slice, n_gram};
use crate::accum_ftzr::Ftzr; // because of the `featurize` method

/// Benchmark #6: n_slice + Vec<&[u8]> (from &str)
pub fn touch_example_for_vec_of_n_length_array_slices(bigstring: &str, n: usize) -> usize {
    let vec_of_n_length_array_slices: Vec<&[u8]> = n_slice(n).featurize(bigstring);
    vec_of_n_length_array_slices.len()
}

/// Benchmark #7: n_gram + Vec<[u8; N]> (from &str)
pub fn touch_example_for_vec_of_n_length_arrays(bigstring: &str, n: usize) -> usize {
    macro_rules! assemble_example_for_vec_of_n_length_arrays {
        ($n:expr) => {{
            let vec_of_n_length_arrays: Vec<[u8; $n]> = n_gram::<$n>().featurize(bigstring);
            vec_of_n_length_arrays.len()
        }}
    }

    match n {
          2 => assemble_example_for_vec_of_n_length_arrays!(  2),
          4 => assemble_example_for_vec_of_n_length_arrays!(  4),
          8 => assemble_example_for_vec_of_n_length_arrays!(  8),
         16 => assemble_example_for_vec_of_n_length_arrays!( 16),
         32 => assemble_example_for_vec_of_n_length_arrays!( 32),
         64 => assemble_example_for_vec_of_n_length_arrays!( 64),
        128 => assemble_example_for_vec_of_n_length_arrays!(128),
          _ => todo!()
    }
}
