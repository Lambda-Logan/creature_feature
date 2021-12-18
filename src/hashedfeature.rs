use crate::gap_gram::GapPair;
use crate::token_from::TokenFrom;
use crate::tokengroup::Token;
use fxhash::FxHasher64;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;
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
        impl<T: Hash> From<Token<T>> for HashedAs<$u_type> {
            fn from(token_group: Token<T>) -> Self {
                let mut h = FxHasher64::default();
                token_group.0.hash(&mut h);
                HashedAs(h.finish() as $u_type)
            }
        }
        impl<T: Hash> TokenFrom<T> for HashedAs<$u_type> {
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

impl_hashed!(u16);
impl_hashed!(u32);
impl_hashed!(u64);

pub type Feature64 = HashedAs<u64>;
