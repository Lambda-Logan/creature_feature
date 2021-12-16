use crate::multiftzr::MultiFtzr;
use crate::token_from::TokenFrom;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};
use std::hash::{BuildHasher, Hash};
use std::ops::AddAssign;
use std::rc::Rc;
use std::sync::Arc;

pub trait IterFtzr<Origin> {
    type TokenGroup;
    type Iter: Iterator<Item = Self::TokenGroup>;
    fn extract_tokens(&self, origin: Origin) -> Self::Iter;
    fn chunk_size(&self) -> usize;
}

pub trait Ftzr<Origin> {
    type TokenGroup;

    fn push_tokens<Push>(&self, origin: Origin, push: &mut Push)
    where
        Push: FnMut(Self::TokenGroup) -> ();

    ////////////////////////////////////////////////////////////////////////////
    fn featurize<Feature: TokenFrom<Self::TokenGroup>, A: Accumulates<Feature>>(
        &self,
        origin: Origin,
    ) -> A {
        let mut state: A::State = Default::default();
        {
            let mut push = |t: Self::TokenGroup| A::accum_token(&mut state, TokenFrom::from(t));
            self.push_tokens(origin, &mut push);
        }
        //Self::TokenGroup: TokenFrom<Self::TokenGroup>
        A::finish(state)
    }

    fn as_fn<X: TokenFrom<Self::TokenGroup>>(self) -> Arc<dyn Fn(Origin) -> Vec<X> + Send + Sync>
    where
        Self: Sized + Send + Sync + 'static,
    {
        Arc::new(move |o| self.featurize(o))
    }

    /*
    fn and_then<T: Ftzr<Origin>>(self, ftzr: T) -> MultiFtzr<Self, T>
    where
        Self: Sized,
    {
        MultiFtzr(self, ftzr)
    } */

    //fn derefed<'a>(&'a self) -> DerefedFtzr<'a, Self> {
    //    DerefedFtzr(self)
    //}
}

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
        Feature: TokenFrom<Token>,
        Iter: Iterator<Item = Token>,
    {
        let mut bag: Self = Default::default();
        for t in iter {
            let f: Feature = TokenFrom::from(t);
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
        Feature: TokenFrom<Token>,
        Iter: Iterator<Item = Token>,
    {
        Iterator::collect(iter.into_iter().map(TokenFrom::from))
    }
}

//// BinaryHeap and BTreeSet require cmp
impl<Feature: Ord> Accumulates<Feature> for BTreeSet<Feature> {
    fn accum<Token, Iter>(iter: Iter) -> Self
    where
        Feature: TokenFrom<Token>,
        Iter: Iterator<Item = Token>,
    {
        Iterator::collect(iter.into_iter().map(TokenFrom::from))
    }
}

impl<Feature: Ord> Accumulates<Feature> for BinaryHeap<Feature> {
    fn accum<Token, Iter>(iter: Iter) -> Self
    where
        Feature: TokenFrom<Token>,
        Iter: Iterator<Item = Token>,
    {
        Iterator::collect(iter.into_iter().map(TokenFrom::from))
    }
}

impl<Feature: Ord, N> Accumulates<Feature> for BTreeMap<Feature, N>
where
    Feature: Eq + Hash,
    N: Default + AddAssign + From<u8>,
{
    fn accum<Token, Iter>(iter: Iter) -> Self
    where
        Feature: TokenFrom<Token>,
        Iter: Iterator<Item = Token>,
    {
        let mut bag: Self = Default::default();
        for t in iter {
            let f: Feature = TokenFrom::from(t);
            *bag.entry(f).or_default() += From::from(1);
        }
        bag
    }
}
 */
