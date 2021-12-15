/*#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]*/

mod token_from;
//use token_from::TokenFrom;

mod tokengroup;

pub mod utils {
    pub use super::tokengroup::{chars_of, Token};
}

mod hashedfeature;
pub use hashedfeature::{Feature64, HashedFeature};

mod accum_ftzr;
mod bookends;
mod gap_gram;
mod multiftzr;
mod n_gram;
mod slice_gram;

pub mod traits {
    pub use super::accum_ftzr::{Accumulates, Ftzr};
    pub use super::token_from::TokenFrom;
}

pub mod ftzrs {
    pub use super::bookends::bookends;
    pub use super::gap_gram::gap_gram;
    pub use super::n_gram::n_gram;
    pub use super::slice_gram::slice_gram;
    pub mod utils {
        pub use super::super::bookends::{BookEnds, BookEndsIter};
        pub use super::super::gap_gram::{GapGram, GapGramIter, GapPair};
        pub use super::super::multiftzr::{EitherGroup, MultiFtzr, MultiFtzrIter};
        pub use super::super::n_gram::{NGram, NGramIter};
        pub use super::super::slice_gram::{SliceGram, SliceGramIter};
    }
}
