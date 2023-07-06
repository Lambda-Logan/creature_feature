use crate::convert::{Bag, Collisions};
use crate::feature_from::FeatureFrom;
use crate::multiftzr::MultiFtzr;
use std::cmp;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};
use std::hash::{BuildHasher, Hash};
use std::ops::AddAssign;
use std::rc::Rc;
use std::sync::Arc;

#[cfg(feature = "heapless")]
use heapless::binary_heap::{self, Kind};

/// The crate's main trait: a highly polymorphic featurizer that's defined by it's ability to visit each group of tokens in the input data.
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
    /// Type of tokens visited by the featurizer. Usually something like `&'a [T]` (for one dimensional data)
    type TokenGroup;

    ///The main method to implement for the [`Ftzr<InputData>`] trait. Each group of tokens is visited by the `FnMut(Self::TokenGroup)` function.
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

    ///Identical to [`Ftzr::push_tokens`], except with implicit coercion via [`FeatureFrom<Self::TokenGroup>`]
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
    /// # A Smattering of [`Ftzr::featurize`]
    /// ```
    ///use creature_feature::traits::*;
    ///use creature_feature::ftzrs::{bigram, bislice, for_each};
    ///use std::collections::{HashMap, HashSet, BTreeMap};
    ///
    ///let int_data = &[1, 2, 3, 4, 5];
    ///let str_data = "one fish two fish red fish blue fish";
    ///
    ///let owned_feats: Vec<String>         = bigram().featurize(str_data);
    ///let owned_feats: HashSet<Vec<usize>> = bigram().featurize(int_data);
    ///let owned_bag:   HashSet<Vec<usize>> = bigram().featurize(int_data);
    ///
    ///let ref_feats: Vec<&str>                  = bislice().featurize(str_data);
    ///let ref_bag:   Bag<HashMap<&[usize], u8>> = bislice().featurize(int_data);
    ///let ref_bag:   Bag<BTreeMap<&str, i16>>   = for_each(bislice()).featurize(str_data.split_ascii_whitespace());
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

    ///Identical to [`Ftzr::featurize`], but produces two outputs while still only featurizing the input once. Pretty cool!
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
    ///let (set, list): (HashSet<(&str, &str)>, LinkedList<HashedAs<u64>>) = ftzr.featurize_x2(data);
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

    /*
    fn collisions<Heap, V, A>(&self, input: InputData) -> A
    where
        V: FeatureFrom<Self::TokenGroup>,
        Collisions<Heap, V, A>: Accumulates<Self::TokenGroup>,
    {
        self.featurize::<_, Collisions<Heap, V, A>>(input).0
        //unimplemented!()
    }*/

    #[allow(missing_docs)]
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

/// A featurizer that can only iterate over the original data.
/// This is implemented for all featurizers in creature_feature for all of the following: `IterFtzr<&str>`,`IterFtzr<&String>`, `IterFtzr<&[T]>`,`IterFtzr<&Vec<T>>`,`IterFtzr<&[T; N]>`.
///
/// # Examples
/// ```
///use creature_feature::ftzrs::bigram;
///use creature_feature::traits::Ftzr;
///use creature_feature::convert::Bag;
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
/// Given an instance of [`IterFtzr<InputData>`], it is very easy to implement the more general [`Ftzr<InputData>`]
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
    /// Type of tokens produced by the featurizer. Usually something like `&'a [T]` (for one dimensional data)
    type TokenGroup;

    /// The associated iterator. For example: `NGramIter`, `MultiFtzrIter`, etc
    type Iter: Iterator<Item = Self::TokenGroup>;

    /// Similar to `into_iter`, but for featurizers.
    fn iterate_features(&self, input: InputData) -> Self::Iter;
}

/// A trait for featurizers that process their input from left-to-right, exactly once and with a constant size.
/// Examples include `NGram<N>`, `SliceGram` and `GapGram<A,B>`
pub trait LinearFixed {
    /// must be equal to the 'n' of an n-gram, or the total stretch of a `GapGram` (including both groups and the space between).
    fn chunk_size(&self) -> usize;
}

/// A trait (inspired by `std::hash::Hasher`) that is the featurizer equivalent of `FromIterator`
///
/// However, there are two key differences:
///  - Something that `Accumulates<Token>` can be built by either iterating tokens, OR visiting tokens.
///  - The `std::FromIterator` instance of `HashMap` does not create a true bag (or multiset)
///
/// `Accumulates` powers `Ftzr::featurize` and seldom needs to be used directly.
pub trait Accumulates<Token> {
    /// The initial State to start accumulating tokens
    type State: Default;

    /// Visits one Token/TokenGroup. Isomorphic to a mutable continuation.
    fn accum_token(state: &mut Self::State, token: Token);

    /// Produces the final result
    fn finish(state: Self::State) -> Self;

    ////////////////////////////////////
    ////////////////////////////////////
}

#[cfg(feature = "heapless")]
impl<Token: Ord, K: Kind, const N: usize> Accumulates<Token>
    for binary_heap::BinaryHeap<Token, K, N>
{
    type State = Self;

    fn accum_token(state: &mut Self::State, token: Token) {
        state.push(token);
    }

    fn finish(state: Self::State) -> Self {
        state
    }
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

impl<Token> Accumulates<Token> for VecDeque<Token> {
    type State = Self;
    fn accum_token(state: &mut Self, token: Token) {
        state.push_back(token);
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

/// # Example: Succinctly implementing MinHash
/// ```
///use creature_feature::ftzrs::bigram;
///use creature_feature::traits::*;
///use creature_feature::HashedAs;
///use std::cmp::Reverse;
///use std::collections::BinaryHeap;
///
/// // jaccard similarity is very fast on two sorted vecs, left as an exercise
///fn min_hash(s: &str, n: usize) -> Vec<HashedAs<u64>> {
///
///    let heap: BinaryHeap<Reverse<HashedAs<u64>>> = bigram().featurize(s);
///
///    heap
///     .into_iter_sorted()
///     .map(|r| r.0)
///     .take(10)
///     .collect()
///}
/// ```
impl<Token: Ord> Accumulates<Token> for BinaryHeap<Token> {
    type State = Self;
    fn accum_token(state: &mut Self, token: Token) {
        state.push(token);
    }
    fn finish(state: Self) -> Self {
        state
    }
}

impl<Token: Ord> Accumulates<Token> for BTreeSet<Token> {
    type State = Self;
    fn accum_token(state: &mut Self, token: Token) {
        state.insert(token);
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

impl<Token, N, S> Accumulates<Token> for Bag<HashMap<Token, N, S>>
where
    Token: Eq + Hash,
    N: Default + AddAssign + From<u8>,
    S: Default + BuildHasher,
{
    type State = Self;
    fn accum_token(state: &mut Self, token: Token) {
        *state.0.entry(token).or_default() += From::from(1);
    }
    fn finish(state: Self) -> Self {
        state
    }
}

impl<Token, K, V, Heap, S> Accumulates<Token> for Collisions<Heap, V, HashMap<K, Heap, S>>
where
    Token: Clone,
    S: Default + BuildHasher,
    K: Eq + Hash + FeatureFrom<Token>,
    V: FeatureFrom<Token>,
    Heap: Accumulates<V>,
{
    type State = HashMap<K, Heap::State, S>;
    fn accum_token(state: &mut Self::State, token: Token) {
        let k: K = FeatureFrom::from(token.clone());
        let mut v_opt = state.get_mut(&k);
        match v_opt {
            Some(v) => Heap::accum_token(v, FeatureFrom::from(token)),
            None => {
                let mut d: Heap::State = Default::default();
                Heap::accum_token(&mut d, FeatureFrom::from(token));
                state.insert(k, d);
            }
        }
    }
    fn finish(state: Self::State) -> Self {
        Collisions::new(
            state
                .into_iter()
                .map(|(k, v)| (k, Heap::finish(v)))
                .collect(),
        )
    }
}

/*
impl<Token, T1, K, V, S> Accumulates<Token> for Collisions<T1, HashMap<K, V, S>>
where
    Token: Clone,
    S: Default + BuildHasher,
    K: Eq + Hash + FeatureFrom<Token>,
    V: Accumulates<T1>,
    T1: FeatureFrom<Token>,
{
    type State = HashMap<K, V::State, S>;
    fn accum_token(state: &mut Self::State, token: Token) {
        let k: K = FeatureFrom::from(token.clone());
        let mut v_opt = state.get_mut(&k);
        match v_opt {
            Some(v) => V::accum_token(v, FeatureFrom::from(token)),
            None => {
                state.insert(k, Default::default());
            }
        }
    }
    fn finish(state: Self::State) -> Self {
        Collisions::new(state.into_iter().map(|(k, v)| (k, V::finish(v))).collect())
    }
}*/

impl<Token: Ord, N: Default + AddAssign + From<u8>> Accumulates<Token> for Bag<BTreeMap<Token, N>> {
    type State = Self;
    fn accum_token(state: &mut Self, token: Token) {
        *state.0.entry(token).or_default() += From::from(1);
    }
    fn finish(state: Self) -> Self {
        state
    }
}

impl<Token, K, V, S> Accumulates<Token> for HashMap<K, V, S>
where
    S: Default + BuildHasher,
    Token: Clone + Eq + Hash,
    K: Eq + Hash + FeatureFrom<Token>,
    V: FeatureFrom<Token>,
{
    type State = Self;
    fn accum_token(state: &mut Self, token: Token) {
        state.insert(
            FeatureFrom::from(token.clone()),
            FeatureFrom::from(token.clone()),
        );
    }
    fn finish(state: Self) -> Self {
        state
    }
}

impl<Token, K, V> Accumulates<Token> for BTreeMap<K, V>
where
    Token: Clone,
    K: Ord + FeatureFrom<Token>,
    V: FeatureFrom<Token>,
{
    type State = Self;
    fn accum_token(state: &mut Self, token: Token) {
        state.insert(
            FeatureFrom::from(token.clone()),
            FeatureFrom::from(token.clone()),
        );
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

impl<'a> Accumulates<&'a str> for String {
    type State = Self;

    #[inline]
    fn accum_token(state: &mut Self::State, tokens: &'a str) {
        state.push_str(tokens);
    }
    #[inline]
    fn finish(state: Self::State) -> Self {
        state
    }
}
