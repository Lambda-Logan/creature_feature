use crate::convert::{Output, SelfOut};
use crate::feature_from::FeatureFrom;
use crate::gap_gram::GapPair;
use fxhash::FxHasher64;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::cmp::Reverse;
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;

///[`HashedAs<U>`] can encode any feature that's hashable. Here, `U` can be `u8`, `u16`, `u32` or `u64`. Hash collisions are usually not a big problem for most uses (especially with `HashedAs<u64>`)
///`HashedAs` can really speed things up where you need to do a lot of equality comparisons and your feature is longer that `U`. It can also provide more balanced nodes in a BTree. Currently uses `FxHash`.
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
#[derive(Hash, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HashedAs<T>(pub(crate) T);

macro_rules! impl_hashed {
    ($u_type:ty) => {
        impl<A: Hash> FromIterator<A> for HashedAs<$u_type> {
            fn from_iter<T>(iter: T) -> Self
            where
                T: IntoIterator<Item = A>,
            {
                let mut h = FxHasher64::default();
                for t in iter.into_iter() {
                    t.hash(&mut h);
                }
                HashedAs(h.finish() as $u_type)
            }
        }
        /*impl<T: Hash> From<Output<T>> for HashedAs<$u_type> {
            fn from(token_group: Output<T>) -> Self {
                let mut h = FxHasher64::default();
                token_group.0.hash(&mut h);
                HashedAs(h.finish() as $u_type)
            }
        } */
        impl<T: Hash> FeatureFrom<T> for HashedAs<$u_type> {
            fn from(token_group: T) -> Self {
                let mut h = FxHasher64::default();
                token_group.hash(&mut h);
                HashedAs(h.finish() as $u_type)
            }
        }
        impl<T: Hash, V: Hash> From<GapPair<T, V>> for HashedAs<$u_type> {
            fn from(x: GapPair<T, V>) -> Self {
                let mut h = FxHasher64::default();
                x.0.hash(&mut h);
                [x.2, 4567].hash(&mut h);
                x.0.hash(&mut h);
                HashedAs(h.finish() as $u_type)
            }
        }
    };
}

impl<A, B> FeatureFrom<A> for Reverse<HashedAs<B>>
where
    HashedAs<B>: FeatureFrom<A>,
{
    fn from(token_group: A) -> Self {
        Reverse(FeatureFrom::from(token_group))
    }
}
impl_hashed!(u8);
impl_hashed!(u16);
impl_hashed!(u32);
impl_hashed!(u64);

pub(crate) type Feature64 = HashedAs<u64>;
