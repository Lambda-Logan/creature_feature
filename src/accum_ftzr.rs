use crate::from_token::FromToken;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};
use std::hash::{BuildHasher, Hash};
use std::ops::AddAssign;
use std::rc::Rc;
use std::sync::Arc;

pub trait Ftzr<Origin> //Self: Sized,
{
    type TokenGroup;
    type Iter: Iterator<Item = Self::TokenGroup>;
    fn extract_tokens(&self, origin: Origin) -> Self::Iter;
    fn chunk_size(&self) -> usize;
    /////////////////////////////
    /////////////////////////////
    fn featurize<Feature: FromToken<Self::TokenGroup>, A: Accumulates<Feature>>(
        &self,
        origin: Origin,
    ) -> A {
        Accumulates::accum(self.extract_tokens(origin))
    }

    fn as_fn<X: FromToken<Self::TokenGroup>>(self) -> Arc<dyn Fn(Origin) -> Vec<X> + Send + Sync>
    where
        Self: Sized + Send + Sync + 'static,
    {
        Arc::new(move |o| self.featurize(o))
    }

    //fn derefed<'a>(&'a self) -> DerefedFtzr<'a, Self> {
    //    DerefedFtzr(self)
    //}
}

pub trait Accumulates<Feature> {
    fn accum<Token, Iter>(iter: Iter) -> Self
    where
        Feature: FromToken<Token>,
        Iter: Iterator<Item = Token>;
}

macro_rules! impl_accum_fromiter {
    ($t:ty) => {
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

        impl<Feature> Accumulates<Feature> for $t {
            fn accum<Token, Iter>(iter: Iter) -> Self
            where
                Feature: FromToken<Token>,
                Iter: Iterator<Item = Token>,
            {
                Iterator::collect(iter.into_iter().map(FromToken::from))
            }
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
        Feature: FromToken<Token>,
        Iter: Iterator<Item = Token>,
    {
        let mut bag: Self = Default::default();
        for t in iter {
            let f: Feature = FromToken::from(t);
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
        Feature: FromToken<Token>,
        Iter: Iterator<Item = Token>,
    {
        Iterator::collect(iter.into_iter().map(FromToken::from))
    }
}

//// BinaryHeap and BTreeSet require cmp
impl<Feature: Ord> Accumulates<Feature> for BTreeSet<Feature> {
    fn accum<Token, Iter>(iter: Iter) -> Self
    where
        Feature: FromToken<Token>,
        Iter: Iterator<Item = Token>,
    {
        Iterator::collect(iter.into_iter().map(FromToken::from))
    }
}

impl<Feature: Ord> Accumulates<Feature> for BinaryHeap<Feature> {
    fn accum<Token, Iter>(iter: Iter) -> Self
    where
        Feature: FromToken<Token>,
        Iter: Iterator<Item = Token>,
    {
        Iterator::collect(iter.into_iter().map(FromToken::from))
    }
}

impl<Feature: Ord, N> Accumulates<Feature> for BTreeMap<Feature, N>
where
    Feature: Eq + Hash,
    N: Default + AddAssign + From<u8>,
{
    fn accum<Token, Iter>(iter: Iter) -> Self
    where
        Feature: FromToken<Token>,
        Iter: Iterator<Item = Token>,
    {
        let mut bag: Self = Default::default();
        for t in iter {
            let f: Feature = FromToken::from(t);
            *bag.entry(f).or_default() += From::from(1);
        }
        bag
    }
}
