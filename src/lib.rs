#![deny(
    unsafe_code,
    unstable_features,
    trivial_casts,
    missing_debug_implementations,
    missing_copy_implementations,
    //trivial_numeric_casts,
    unused_import_braces,
    unused_qualifications,
    missing_docs
)]
#![allow(unused)]

//! # CREATURE FEATUR(ization)
//! A crate for polymorphic ML & NLP featurization that leverages zero-cost abstraction. It provides composable n-gram combinators that are ergonomic and bare-metal fast. Although created with NLP in mind, it's very general and can be applied in a plethera of domains such as computer vision.
//!
//! There are many n-gram crates, but the majority force heap allocation or lock you into a concrete type that doesn’t fit your use-case or performance needs. In most benchmarks, `creature_feature` is anywhere between 4x - 60x faster.
//!
//!
//! ![Image](https://raw.githubusercontent.com/Lambda-Logan/creature_feature/master/godzilla_featurization.jpg)
//!
//! # See a live demo
//!
//! [Here](https://ecstatic-leakey-c7a4fc.netlify.app/)  is a live demo of creature_feature using WASM
//!
//!
//! # A Swiss Army Knife of Featurization
//! ```rust
//!use creature_feature::traits::Ftzr;
//!use creature_feature::ftzrs::{bigram, bislice, for_each, whole};
//!use creature_feature::HashedAs;
//!use creature_feature::convert::Bag;
//!use std::collections::{HashMap, HashSet, BTreeMap, LinkedList, BTreeSet, BinaryHeap, VecDeque};
//!
//!let int_data = &[1, 2, 3, 4, 5];
//!let str_data = "one fish two fish red fish blue fish";
//!
//! // notice how the left-hand side remains almost unchanged.
//!
//! // we're using 'bislice' right now (which is a 2-gram of referenced data), 'ftzrs::bigram' would yield owned data instead of references
//!
//! let ref_feats: Vec<&str>                  = bislice().featurize(str_data);
//!let ref_feats: LinkedList<&[u8]>          = bislice().featurize(str_data);
//!let ref_bag:   Bag<HashMap<&[usize], u8>> = bislice().featurize(int_data);
//!let ref_trigram_bag:   Bag<BTreeMap<&str, i16>>   = for_each(whole()).featurize(str_data.split_ascii_whitespace());
//!let hashed_trigrams: BTreeSet<HashedAs<u64>> = trislice().featurize(int_data);
//!```
//!The above five will have the following values, respectively.
//! ```
//! ["on", "ne", "e ", " f", "fi", "is", "sh", "h ", " t", "tw", "wo", "o ", " f", "fi", "is", "sh", "h ", " r", "re", "ed", "d ", " f", "fi", "is", "sh", "h ", " b", "bl", "lu", "ue", "e ", " f", "fi", "is", "sh"]
//!
//! [[111, 110], [110, 101], [101, 32], [32, 102], [102, 105], [105, 115], [115, 104], [104, 32], [32, 116], [116, 119], [119, 111], [111, 32], [32, 102], [102, 105], [105, 115], [115, 104], [104, 32], [32, 114], [114, 101], [101, 100], [100, 32], [32, 102], [102, 105], [105, 115], [115, 104], [104, 32], [32, 98], [98, 108], [108, 117], [117, 101], [101, 32], [32, 102], [102, 105], [105, 115], [115, 104]]
//!
//! Bag({[2, 3, 4]: 1, [3, 4, 5]: 1, [1, 2, 3]: 1})
//!
//! Bag({"blue": 1, "fish": 4, "one": 1, "red": 1, "two": 1})
//!
//! {HashedAs(3939941806544028562), HashedAs(7191405660579021101), HashedAs(16403185381100005216)}
//! ```
//!
//! Here are more examples of what's possible:
//!  
//!```rust
//! // let's now switch to 'bigram'
//!let owned_feats: BTreeSet<[u8; 2]>        = bigram().featurize(str_data);
//!let owned_feats: Vec<String>              = bigram().featurize(str_data);
//!let owned_feats: HashSet<Vec<usize>>      = bigram().featurize(int_data);
//!let owned_bag:   Bag<HashMap<Vec<usize>, u16>>      = bigram().featurize(int_data);
//!
//!let hashed_feats: BinaryHeap<HashedAs<u32>> = bislice().featurize(str_data);
//!let hashed_feats: VecDeque<HashedAs<u64>>   =  bigram().featurize(int_data);
//!
//!let sentence = str_data.split_ascii_whitespace();
//!let bag_of_words: Bag<HashMap<String, u128>> = for_each(bigram()) .featurize(sentence.clone());
//!let bag_of_words: Bag<HashMap<&str, u8>>     = for_each(bislice()).featurize(sentence.clone());
//!
//! // and many, MANY more posibilities
//! ```
//!
//! ### We can even produce multiple outputs while still only featurizing the input once
//! ```rust
//! let (set, list): (BTreeSet<HashedAs<u64>>, Vec<&str>) = bislice().featurize_x2(str_data);
//! ```
//!
//!
//! # `creature_feature` provides three general flavors of featurizers:
//!
//!1) `NGram<const N: usize>` provides n-grams over copied data and produces owned data or multiple `[T;N]`. Examples include [`ftzrs::n_gram`], [`ftzrs::bigram`] and [`ftzrs::trigram`].
//!
//!2) `SliceGram` provides n-grams over referenced data and produces owned data or multiple &[T]. Examples include [`ftzrs::n_slice`], [`ftzrs::bislice`] and [`ftzrs::trislice`].
//!
//!3) Combinators that compose one or more featurizers and return a new featurizer with different behavior. Examples include [`ftzrs::for_each`], [`ftzrs::gap_gram`], [`featurizers!`] and [`ftzrs::bookends`].
//!
//!
//! # WHY POLYMORPHISM == PERFORMANCE
//! **Here is a small quiz to show why polymorphic featurization and _FAST_ featurization go hand-in-hand.**
//!
//!
//! Here are four different ways to featurize a string that are basically equivalent. But, which one of the four is fastest? By how much?
//!
//! ```rust
//! let sentence = "It is a truth universally acknowledged that Jane Austin must be used in nlp examples";
//!
//! let one:   Vec<String> = trigram().featurize(sentence);
//! let two:   Vec<[u8;3]> = trigram().featurize(sentence);
//! let three: Vec<&str>   = trislice().featurize(sentence); // same performance as &[u8]
//! let four:  Vec<HashedAs<u64>> = trislice().featurize(sentence); // could have used trigram
//! ```
//!
//! Trigrams of `String`, [`HashedAs<u64>`], `&str` and `[u8; 3]` each have their place depending on your use-case. But there can be roughly _two orders of magnitude_ of difference in performance between the fastest and the slowest. If you choose the wrong one for your needs (or use a less polymorphic crate), you're losing out on speed!
//!
//! # What type should I use to represent my features?
//! * use `Collection<[T; N]>` via [`ftzrs::n_gram`] if both `T` and `N` are small. This is most of the time.
//!
//! * use `Collection<&[T]>` (or `Collection<&str>`) via [`ftzrs::n_slice`] if `[T; N]` would be larger (in bytes) than `(usize, usize)`. This is more common if `N` is large or you're using `char` instead of `u8`. This is also depends on the lifetime of the original data vs the lifetime of the features produced.
//!
//! * [`HashedAs<u64>`] has the opposite time complexity as `&[T]`, linear creation and O(1) equality. If you're ok with one-in-a-millionish hash collisions, this can be a great compromise.
//!
//! * Never use `Collection<String>` or `Collection<Vec<T>>` in a performance critical section.
//!
//! # Where does `creature_feature` fit in with other tokenizers?
//!
//! `creature_feature` is very flexible, and [`traits::Ftzr`]/[`traits::IterFtzr`] can be easily implemented with a newtype for whatever other tokenizer/featurizer you please. Anything could be featurized: images, documents, time-series data, etc.
//!
//! # Example: Featurizing books
//! Consider a custom struct to represent a book
//! ```rust
//!struct Book {
//!    author: String,
//!    genre: Genre,
//!    sub_genre: SubGenre,
//!    year: u16,
//!}
//!
//!#[derive(Hash)]
//!enum Genre {
//!    Fiction,
//!    NonFiction,
//!    Religion,
//!}
//!
//!#[derive(Hash)]
//!enum SubGenre {
//!    Romance,
//!    History,
//!    DataScience,
//!}
//!
//!impl Book {
//!    fn decade(&self) -> u8 {
//!        unimplemented!()
//!    }
//!}
//! ```
//! We can easily make a custom featurizer for `Book` by visitation with [`traits::Ftzr`].
//! ```rust
//!use creature_feature::ftzrs::whole;
//!use creature_feature::traits::*;
//!use creature_feature::HashedAs;
//!
//!struct BookFtzr;
//!
//!impl<'a> Ftzr<&'a Book> for BookFtzr {
//!    type TokenGroup = HashedAs<u64>;
//!    fn push_tokens<Push: FnMut(Self::TokenGroup)>(&self, book: &'a Book, push: &mut Push) {
//!        whole().push_tokens_from(&book.author, push);
//!        push(FeatureFrom::from(&book.genre));
//!        push(FeatureFrom::from(&book.sub_genre));
//!        push(FeatureFrom::from(book.year));
//!        push(FeatureFrom::from(book.decade()));
//!    }
//!}
//! ```
//! Now we could easily implement a similarity metric for `Book` via `Vec<HashedAs<u64>>`, like cosine or jaccard.
//!
//! # Usage notes
//! * No bounds checking is performed. This is the responsibility of the user.
//! * To handle unicode, convert to `Vec<char>`
//!
//!
//! # YOU CAN HELP
//!
//! I'm actually an English teacher, not a dev. So any PRs, observations or feedback is very welcome. I've done my best to document everything well, but if you have any questions feel free to reach out. Your help with any of the small number of [current issues](https://github.com/Lambda-Logan/creature_feature/issues) would be VERY much welcome :)
//!
mod internal;

mod feature_from;
//use feature_from::FeatureFrom;
mod skip_schema;
mod tokengroup;

/*
pub mod utils {
    pub use super::tokengroup::chars_of;
}*/

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

/// All traits contained in the crate. The most imminently useful are [`traits::Ftzr`] and [`traits::FeatureFrom`]
pub mod traits {
    pub use super::accum_ftzr::{Accumulates, Ftzr, IterFtzr, LinearFixed};
    pub use super::feature_from::FeatureFrom;
}

/// newtype wrappers to supplement [`traits::FeatureFrom`]
pub mod convert;

/// featurizers & combinators
pub mod ftzrs {

    /// A macro for general composition of featurizers. `featurizers!(ftzr_a, ftzr_b, ... ftzr_z)` will featurize input data exactly `z` times. Must use `Merged` in the output.
    ///
    /// ```
    /// let my_ftzr = featurizers!(ftzr1,
    ///                            ftzr2,
    ///                            ftzr3,
    ///                            ftzr_n);
    /// let feats: HashSet<Merged<&str>> = my_ftzr.featurize(your_data);
    /// ```
    #[macro_export]
    macro_rules! featurizers {
        ($a:expr) => {
            $a
        };
        ($a:expr $(, $tail:expr)*) => {{
            use creature_feature::ftzrs::utils::MultiFtzr;
            MultiFtzr($a, featurizers!($($tail), *),
        )
        }};
    }

    pub use featurizers;

    pub use super::bookends::bookends;
    pub use super::gap_gram::gap_gram;
    //pub use super::multiftzr::featurizers;
    pub use super::for_each::for_each;
    pub use super::n_gram::{bigram, n_gram, trigram};
    pub use super::n_slice::{bislice, n_slice, trislice};
    pub use super::whole_empty::{empty, whole};

    /// All associated types for the featurizers in `creature_feature::ftzrs`. Most featurizers have an an associated iterator, and possibly a specialized type for `Self::TokenGroup`.
    pub mod misc {
        pub use super::super::bookends::{BookEnds, BookEndsIter, FrontBack};
        pub use super::super::for_each::ForEach;
        pub use super::super::gap_gram::{GapGram, GapGramIter, GapPair};
        pub use super::super::multiftzr::{EitherGroup, MultiFtzr, MultiFtzrIter};
        pub use super::super::n_gram::{NGram, NGramIter};
        pub use super::super::n_slice::{SliceGram, SliceGramIter};
        pub use super::super::whole_empty::{Empty, EmptyAtom, Whole};
    }
}


/// The custom benchmark (`src/bench.rs`) is being rewritten for Criterion: `src/bench_criterionized.rs`.
/// It is run by `bench/bench_criterionized_runner.rs`.
pub mod bench_criterionized;


#[cfg(test)]
mod tests {
    use crate::ftzrs::{bislice, bigram};
    use crate::accum_ftzr::Ftzr; // because of the `featurize` method

    #[test]
    fn test_for_vec_of_str_slices() {
        let vec_of_str_slices: Vec<&str> = bislice().featurize("one fish");
        assert_eq!(vec_of_str_slices, vec!["on", "ne", "e ", " f", "fi", "is", "sh"]);
    }

    // n = 2 @ creature_feature::slice_gram + Vec<&[u8]> (from &str):
    #[test]
    fn test_for_vec_of_2_length_array_slices() {
        let vec_of_2_length_array_slices: Vec<&[u8]> = bislice().featurize("one fish");
        assert_eq!(vec_of_2_length_array_slices, vec![&[b'o', b'n'], &[b'n', b'e'], &[b'e', b' '], &[b' ', b'f'], &[b'f', b'i'], &[b'i', b's'], &[b's', b'h']]);
    }

    // n = 2 @ creature_feature::n_gram + Vec<[u8; N]> (from &str):
    #[test]
    fn test_for_vec_of_2_length_arrays() {
        let vec_of_2_length_arrays: Vec<[u8; 2]> = bigram().featurize("one fish");
        assert_eq!(vec_of_2_length_arrays, vec![[b'o', b'n'], [b'n', b'e'], [b'e', b' '], [b' ', b'f'], [b'f', b'i'], [b'i', b's'], [b's', b'h']]);
    }
}
