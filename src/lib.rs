#![deny(
    unsafe_code,
    unstable_features,
    trivial_casts,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_numeric_casts,
    unused_import_braces,
    unused_qualifications
    //missing_docs
)]
#![allow(unused)]

mod internal;

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
mod whole_empty;

pub mod traits {
    pub use super::accum_ftzr::{Accumulates, IterFtzr, LinearFixed};
    pub use super::token_from::TokenFrom;
}

pub mod ftzrs {
    pub use super::bookends::bookends;
    pub use super::gap_gram::gap_gram;
    pub use super::multiftzr::featurizers;
    pub use super::n_gram::n_gram;
    pub use super::slice_gram::slice_gram;
    pub use super::whole_empty::{empty, whole};
    pub mod utils {
        pub use super::super::bookends::{BookEnds, BookEndsIter};
        pub use super::super::gap_gram::{GapGram, GapGramIter, GapPair};
        pub use super::super::multiftzr::{EitherGroup, MultiFtzr, MultiFtzrIter};
        pub use super::super::n_gram::{NGram, NGramIter};
        pub use super::super::slice_gram::{SliceGram, SliceGramIter};
        pub use super::super::whole_empty::{Empty, EmptyAtom, Whole, WholeAtom};
    }
}
