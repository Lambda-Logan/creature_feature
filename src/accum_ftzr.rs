use crate::feature_from::FeatureFrom;
use crate::multiftzr::MultiFtzr;
use std::collections::{
    BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, LinkedList, VecDeque,
};
use std::hash::{BuildHasher, Hash};
use std::ops::AddAssign;
use std::rc::Rc;
use std::sync::Arc;

/// A featurizer that can iterate over the original data.
/// This is implemented for all featurizers in creature_feature for all of the following: `IterFtzr<&str>`,`IterFtzr<&String>`, `IterFtzr<&[T]>`,`IterFtzr<&Vec<T>>`,`IterFtzr<&[T; N]>`,
///
/// # Examples
/// ```
///use creature_feature::ftzrs::bigram;
///use creature_feature::traits::*;
///
///let data = &[1, 2, 3, 4, 5];
///
///for bg in bigram().iterate_features(data) {
///    println!("{:?}", bg)
///}
/// //[1, 2]
/// //[2, 3]
/// //[3, 4]
/// //[4, 5]
/// ```
///
/// Given an instance of `IterFtzr<InputData>`, it is very easy to implement the more general `Ftzr<InputData>`
/// ```
///use creature_feature::traits::IterFtzr;
///struct MyIterFtzr;
///
///impl<InputData> Ftzr<InputData> for MyIterFtzr
///where
///    MyIterFtzr: IterFtzr<InputData>,
///{
///    type TokenGroup = <Self as IterFtzr<InputData>>::TokenGroup;
///    fn push_tokens<Push>(&self, input: InputData, push: &mut Push)
///    where
///        Push: FnMut(Self::TokenGroup) -> (),
///    {
///        for t in self.iterate_features(input) {
///            push(t)
///        }
///    }
///}
/// ```
pub trait IterFtzr<InputData> {
    type TokenGroup;
    type Iter: Iterator<Item = Self::TokenGroup>;
    fn iterate_features(&self, input: InputData) -> Self::Iter;
}

/// A highly polymorphic featurizer that's defined by it's ability to visit each group of tokens in the input data.
///
/// Even with some boilerplate, implementing a custom featurizer by visitation can be much simpler that by iteration.
/// # Example of a length-based featurizer
/// ```
///use creature_feature::ftzrs::{for_each, n_slice};
///use creature_feature::traits::Ftzr;
///
///struct MyLengthBasedFtzr;
///
///impl<'a> Ftzr<&'a str> for MyLengthBasedFtzr {
///
///    type TokenGroup = &'a [u8];
///
///    fn push_tokens<Push: FnMut(Self::TokenGroup)>(&self, input: &'a str, push_token_group: &mut Push)
///    {
///        if input.len() <= 2 {
///            push_token_group(input.as_bytes());
///        }
///        if input.len() > 2 {
///            n_slice(3).push_tokens(input, push_token_group);
///        }
///    }
///}
///let sentence = "It is a truth universally acknowledged \
///    that Jane Austin must be used in nlp examples"
///    .split_ascii_whitespace();
///    
///let feats: Vec<&str> = for_each(MyLengthBasedFtzr).featurize(sentence);
///println!("{:?}", feats);
///
/// //>>> ["it", "is", "a", "tru", "rut", "uth", "uni", "niv", "ive", "ver", "ers", "rsa", "sal", "all", "lly", ... "use", "sed", "in", "nlp"]
///```
pub trait Ftzr<InputData> {
    type TokenGroup;

    ///The main method to implement for the `Ftzr<InputData>` trait. Each group of tokens is visited by the `FnMut(Self::TokenGroup)` function.
    ///
    /// # Example
    ///```
    ///use creature_feature::ftzrs::trigram;
    ///use creature_feature::traits::*;
    ///
    ///let mut print_triplet = |x: [isize; 3]| println!("{:?}", x);
    ///
    ///let data = &[0, 2, 4, 6, 8];
    ///
    ///trigram().push_tokens(data, &mut print_triplet);
    /// // [0, 2, 4]
    /// // [2, 4, 6]
    /// // [4, 6, 8]
    /// ```
    fn push_tokens<Push: FnMut(Self::TokenGroup)>(&self, input: InputData, push: &mut Push);

    ///Identical to `push_tokens`, except with implicit coercion via `FeatureFrom<Self::TokenGroup>`
    ///
    /// # Example
    ///
    ///```rust
    ///use creature_feature::ftzrs::n_slice;
    ///use creature_feature::traits::*;
    ///use creature_feature::HashedAs;
    ///
    ///let sentence = "It is a truth universally acknowledged \
    ///    that Jane Austin must be used in nlp examples";
    ///
    ///let mut print_str = |s: &str| println!("{:?}", s);
    ///let mut print_hashed = |s: HashedAs<u32>| println!("{:?}", s);
    ///
    ///  //-----------&str: FeatureFrom<&[u8]>-------------//
    ///n_slice(50).push_tokens_from(sentence, &mut print_str);
    /// //>>>"It is a truth universally acknowledged that Jane A"
    /// //>>>"t is a truth universally acknowledged that Jane Au"
    ///
    /// //-----------HashedAs<u32>: FeatureFrom<&[u8]>----//
    ///n_slice(50).push_tokens_from(sentence, &mut print_hashed);
    /// //>>>HashedAs(3003844930)
    /// //>>>HashedAs(3843215312)
    /// //>>>HashedAs(4188097127)
    ///```
    ///
    ///
    fn push_tokens_from<Push, T>(&self, input: InputData, push: &mut Push)
    where
        Push: FnMut(T),
        T: FeatureFrom<Self::TokenGroup>,
    {
        let mut _push = |t| push(FeatureFrom::from(t));
        self.push_tokens(input, &mut _push)
    }

    /// The most versatile tool in the entire crate.
    /// # A Smattering of Ftzr::featurize
    /// ```
    ///use creature_feature::ftzrs::{bigram, bislice, for_each};
    ///use std::collections::{HashMap, HashSet};
    ///
    ///let int_data = &[1, 2, 3, 4, 5];
    ///let str_data = "one fish two fish red fish blue fish";
    ///
    ///let owned_feats: Vec<String> = bigram().featurize(str_data);
    ///let owned_feats: HashSet<Vec<usize>> = bigram().featurize(int_data);
    ///let owned_bag: HashSet<Vec<usize>> = bigram().featurize(int_data);
    ///
    ///let ref_feats: Vec<&str> = bislice().featurize(str_data);
    ///let ref_bag: HashMap<&[usize], u8> = bislice().featurize(int_data);
    ///let ref_bag: HashMap<&str, i16> = for_each(bislice()).featurize(str_data.split_ascii_whitespace());
    ///
    /// // and many, MANY more posibilities
    /// ```
    fn featurize<Feature, A>(&self, input: InputData) -> A
    where
        Feature: FeatureFrom<Self::TokenGroup>,
        A: Accumulates<Feature>,
    {
        let mut state: A::State = Default::default();
        {
            let mut push = |t: Self::TokenGroup| A::accum_token(&mut state, FeatureFrom::from(t));
            self.push_tokens(input, &mut push);
        }
        //Self::TokenGroup: FeatureFrom<Self::TokenGroup>
        A::finish(state)
    }

    ///Identical to `featurize`, but produces two outputs while still only featurizing the input once. Pretty cool!
    /// # Example
    /// ```
    ///use creature_feature::ftzrs::{gap_gram, trislice};
    ///use creature_feature::traits::Ftzr;
    ///use creature_feature::HashedAs;
    ///use std::collections::{HashSet, LinkedList};
    ///
    ///let ftzr = gap_gram(trislice(), 1, trislice());
    ///
    ///let data = "0123456789";
    ///
    ///let (set, list): (HashSet<&str>, LinkedList<HashedAs<u64>>) = ftzr.featurize_x2(data);
    /// ```
    fn featurize_x2<A1, T1, A2, T2>(&self, input: InputData) -> (A1, A2)
    where
        Self::TokenGroup: Clone,
        A1: Accumulates<T1>,
        A2: Accumulates<T2>,
        T1: FeatureFrom<Self::TokenGroup>,
        T2: FeatureFrom<Self::TokenGroup>,
    {
        let mut state1: A1::State = Default::default();
        let mut state2: A2::State = Default::default();
        {
            let mut push = |t: Self::TokenGroup| {
                A1::accum_token(&mut state1, FeatureFrom::from(t.clone()));
                A2::accum_token(&mut state2, FeatureFrom::from(t));
            };

            self.push_tokens(input, &mut push);
        }
        //Self::TokenGroup: FeatureFrom<Self::TokenGroup>
        (A1::finish(state1), A2::finish(state2))
    }

    fn as_fn<X: FeatureFrom<Self::TokenGroup>>(
        self,
    ) -> Arc<dyn Fn(InputData) -> Vec<X> + Send + Sync>
    where
        Self: Sized + Send + Sync + 'static,
    {
        Arc::new(move |o| self.featurize(o))
    }

    /*
    fn and_then<T: Ftzr<InputData>>(self, ftzr: T) -> MultiFtzr<Self, T>
    where
        Self: Sized,
    {
        MultiFtzr(self, ftzr)
    } */

    //fn derefed<'a>(&'a self) -> DerefedFtzr<'a, Self> {
    //    DerefedFtzr(self)
    //}
}

/// A trait for featurizers that process their input from left-to-right, exactly once and with a constant size.
/// Examples include `NGram<N>`, `SliceGram` and `GapGram<A,B>`
pub trait LinearFixed {
    // this must be equal to the 'n' of an n-gram, or the total stretch of a GapGram (including both groups and the space between).
    fn chunk_size(&self) -> usize;
}

/// A trait (inspired by `std::hash::Hasher`) that is the featurizer equivalent of `FromIterator`, with two key differences:
///  - Something that `Accumulates<Token>` can be built by either iterating tokens, OR visiting tokens.
///  - The FromIterator instance of `HashMap` does not create a true bag (or multiset)
pub trait Accumulates<Token> {
    type State: Default;
    fn accum_token(state: &mut Self::State, token: Token);

    fn finish(state: Self::State) -> Self;

    ////////////////////////////////////
    ////////////////////////////////////
}

impl<Token, A, B> Accumulates<Token> for (A, B)
where
    Token: Clone,
    A: Accumulates<Token>,
    B: Accumulates<Token>,
{
    type State = (A::State, B::State);
    fn accum_token(state: &mut Self::State, token: Token) {
        A::accum_token(&mut state.0, token.clone());
        B::accum_token(&mut state.1, token);
    }

    fn finish(state: Self::State) -> Self {
        (A::finish(state.0), B::finish(state.1))
    }

    ////////////////////////////////////
    ////////////////////////////////////
}

impl<Token> Accumulates<Token> for Vec<Token> {
    type State = Self;
    fn accum_token(state: &mut Self, token: Token) {
        state.push(token);
    }
    fn finish(state: Self) -> Self {
        state
    }
}

impl<Token> Accumulates<Token> for LinkedList<Token> {
    type State = Self;
    fn accum_token(state: &mut Self, token: Token) {
        state.push_back(token);
    }
    fn finish(state: Self) -> Self {
        state
    }
}

impl<Token: Eq + Hash, S: Default + BuildHasher> Accumulates<Token> for HashSet<Token, S> {
    type State = Self;
    fn accum_token(state: &mut Self, token: Token) {
        state.insert(token);
    }
    fn finish(state: Self) -> Self {
        state
    }
}

impl<Token, N, S> Accumulates<Token> for HashMap<Token, N, S>
where
    Token: Eq + Hash,
    N: Default + AddAssign + From<u8>,
    S: Default + BuildHasher,
{
    type State = Self;
    fn accum_token(state: &mut Self, token: Token) {
        *state.entry(token).or_default() += From::from(1);
    }
    fn finish(state: Self) -> Self {
        state
    }
}

impl Accumulates<&[&str]> for String {
    type State = Self;

    #[inline]
    fn accum_token(state: &mut Self::State, tokens: &[&str]) {
        let mut x = 0;
        let len = tokens.len();
        for t in tokens.iter() {
            state.push_str(*t);
            if x != len - 1 {
                state.push_str("  ");
            }
            x += 1
        }
    }
    #[inline]
    fn finish(state: Self::State) -> Self {
        state
    }
}

impl<'a> Accumulates<&'a [char]> for String {
    type State = Self;

    #[inline]
    fn accum_token(state: &mut Self::State, tokens: &'a [char]) {
        state.extend(tokens.iter())
    }
    #[inline]
    fn finish(state: Self::State) -> Self {
        state
    }
}

impl<'a> Accumulates<&'a [u8]> for String {
    type State = Self;

    #[inline]
    fn accum_token(state: &mut Self::State, tokens: &'a [u8]) {
        state.extend(tokens.iter().map(|token| {
            let chr: char = From::from(*token);
            chr
        }))
    }
    #[inline]
    fn finish(state: Self::State) -> Self {
        state
    }
}

/*
macro_rules! impl_accum_fromiter {
    ($t:ty, $state:ty) => {
        /*
        impl<Feature> Accumulates<Feature> for $t {
            fn accum<Token, Iter>(iter: Iter) -> Self
            where
                Feature: From<Token>,
                Iter: Iterator<Item = Token>,
            {
                Iterator::collect(iter.map(From::from))
            }
        } */
        pub trait Accumulates<Feature> {
            type State = $state;
            fn accum_token(state: &mut Self::State, token: Token) {

            }
            fn finish(state: Self::State) -> Self;
            ////////////////////////////////////
            ////////////////////////////////////
        }

    };
}
impl_accum_fromiter!(Box<[Feature]>);
impl_accum_fromiter!(Rc<[Feature]>);
impl_accum_fromiter!(Arc<[Feature]>);
impl_accum_fromiter!(VecDeque<Feature>);
impl_accum_fromiter!(Vec<Feature>);
impl_accum_fromiter!(LinkedList<Feature>);

///HashMap + HashSet need to work for any BuildHasher
///Also, HashMap here is a Bag and has different semantics than
///the normal FromIterator for HashMap.... This is a large reason why
/// the Accumulates trait exists instead of using FromIterator
impl<Feature, N, S> Accumulates<Feature> for HashMap<Feature, N, S>
where
    Feature: Eq + Hash,
    N: Default + AddAssign + From<u8>,
    S: Default + BuildHasher,
{
    fn accum<Token, Iter>(iter: Iter) -> Self
    where
        Feature: FeatureFrom<Token>,
        Iter: Iterator<Item = Token>,
    {
        let mut bag: Self = Default::default();
        for t in iter {
            let f: Feature = FeatureFrom::from(t);
            *bag.entry(f).or_default() += From::from(1);
        }
        bag
    }
}

impl<Feature, S> Accumulates<Feature> for HashSet<Feature, S>
where
    Feature: Eq + Hash,
    S: Default + BuildHasher,
{
    fn accum<Token, Iter>(iter: Iter) -> Self
    where
        Feature: FeatureFrom<Token>,
        Iter: Iterator<Item = Token>,
    {
        Iterator::collect(iter.into_iter().map(FeatureFrom::from))
    }
}

//// BinaryHeap and BTreeSet require cmp
impl<Feature: Ord> Accumulates<Feature> for BTreeSet<Feature> {
    fn accum<Token, Iter>(iter: Iter) -> Self
    where
        Feature: FeatureFrom<Token>,
        Iter: Iterator<Item = Token>,
    {
        Iterator::collect(iter.into_iter().map(FeatureFrom::from))
    }
}

impl<Feature: Ord> Accumulates<Feature> for BinaryHeap<Feature> {
    fn accum<Token, Iter>(iter: Iter) -> Self
    where
        Feature: FeatureFrom<Token>,
        Iter: Iterator<Item = Token>,
    {
        Iterator::collect(iter.into_iter().map(FeatureFrom::from))
    }
}

impl<Feature: Ord, N> Accumulates<Feature> for BTreeMap<Feature, N>
where
    Feature: Eq + Hash,
    N: Default + AddAssign + From<u8>,
{
    fn accum<Token, Iter>(iter: Iter) -> Self
    where
        Feature: FeatureFrom<Token>,
        Iter: Iterator<Item = Token>,
    {
        let mut bag: Self = Default::default();
        for t in iter {
            let f: Feature = FeatureFrom::from(t);
            *bag.entry(f).or_default() += From::from(1);
        }
        bag
    }
}
 */
