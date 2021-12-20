#![deny(
    unsafe_code,
    unstable_features,
    trivial_casts,
    missing_debug_implementations,
    missing_copy_implementations,
    //trivial_numeric_casts,
    unused_import_braces,
    unused_qualifications
    //missing_docs
)]
#![allow(unused)]

mod internal;

mod feature_from;
//use feature_from::FeatureFrom;
mod skip_schema;
mod tokengroup;

pub mod utils {
    pub use super::tokengroup::{chars_of, Token};
}

mod hashedfeature;
pub use hashedfeature::HashedAs;

mod accum_ftzr;
mod bookends;
mod for_each;
mod gap_gram;
mod multiftzr;
mod n_gram;
mod n_slice;
mod whole_empty;

pub mod traits {
    pub use super::accum_ftzr::{Accumulates, Ftzr, IterFtzr, LinearFixed};
    pub use super::feature_from::FeatureFrom;
}

pub mod ftzrs {
    /// look for doc comments '///'
    /// TODO size hints for pre-allocation
    /*
            let ftzr = featurizers!(
            n_gram::<2>(),
            n_gram::<3>(),
            bookends((2, n_gram::<2>()), (n_gram::<2>(), 2))
        );

        push_tokens(str)
    */

    macro_rules! featurizers {
        [$a:expr] => {
            $a
        };
        [$a:expr $(, $tail:expr)*] => {{
            use creature_feature::ftzrs::utils::MultiFtzr;
            MultiFtzr($a, featurizers!($($tail), *),
        )
        }};
    }

    pub use super::bookends::bookends;
    pub use super::gap_gram::gap_gram;
    //pub use super::multiftzr::featurizers;
    pub use super::for_each::for_each;
    pub use super::n_gram::{bigram, n_gram, trigram};
    pub use super::n_slice::{bislice, n_slice, trislice};
    pub use super::whole_empty::{empty, whole};
    pub mod utils {
        pub use super::super::bookends::{BookEnds, BookEndsIter};
        pub use super::super::for_each::ForEach;
        pub use super::super::gap_gram::{GapGram, GapGramIter, GapPair};
        pub use super::super::multiftzr::{EitherGroup, MultiFtzr, MultiFtzrIter};
        pub use super::super::n_gram::{NGram, NGramIter};
        pub use super::super::n_slice::{SliceGram, SliceGramIter};
        pub use super::super::whole_empty::{Empty, EmptyAtom, Whole, WholeAtom};
    }
}
