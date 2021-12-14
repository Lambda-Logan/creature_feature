use crate::from_token::FromToken;
use crate::gap_gram::GapPair;
use crate::tokengroup::Token;
use fxhash::FxHasher64;
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;

#[derive(Hash, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Debug)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct Feature64(pub(crate) u64);

impl<A: Hash> FromIterator<A> for Feature64 {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = A>,
    {
        let mut h = FxHasher64::default();
        for t in iter.into_iter() {
            t.hash(&mut h);
        }
        Feature64(h.finish())
    }
}

impl<T: Hash> From<Token<T>> for Feature64 {
    fn from(token_group: Token<T>) -> Self {
        let mut h = FxHasher64::default();
        token_group.0.hash(&mut h);
        Feature64(h.finish())
    }
}

impl<T: Hash> FromToken<T> for Feature64 {
    fn from(token_group: T) -> Self {
        let mut h = FxHasher64::default();
        token_group.hash(&mut h);
        Feature64(h.finish())
    }
}

impl<T: Hash, V: Hash> From<GapPair<T, V>> for Feature64 {
    fn from(x: GapPair<T, V>) -> Self {
        let mut h = FxHasher64::default();
        x.0.hash(&mut h);
        [x.2, 4567].hash(&mut h);
        x.0.hash(&mut h);
        Feature64(h.finish())
    }
}

#[derive(Hash, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Debug)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct HashedFeature<U>(pub U);

type FeatBits = u16;
impl<A: Hash> FromIterator<A> for HashedFeature<FeatBits> {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = A>,
    {
        let mut h = FxHasher64::default();
        for t in iter.into_iter() {
            t.hash(&mut h);
        }
        HashedFeature(h.finish() as FeatBits)
    }
}
