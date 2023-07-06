#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::collections::{LinkedList, VecDeque};
use std::hash::Hash;
use std::iter::FromIterator;
use std::marker::PhantomData;
use std::ops::Deref;
use std::rc::Rc;
use std::str::from_utf8;
use std::sync::Arc;

///A `Bag` is a wrapper for `HashMap` and `BTreeMap` so that they can be treated as a multiset. `Accumulates<Token>` can create either a mapping that only keeps the most recently seen value, or the count of the key (a bag).
///
/// `Bag<SomeMap<K, V>>` must have integer or unsigned integer values `V`, whereas `SomeMap<K, V>` doesn't have this limitation.
/// ```
/// let bag_of_words Bag<HashMap<&str, i8>> = for_each(your_ftzr).featuize(your_sentence);
/// ```
#[derive(Hash, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Bag<T>(pub T);

/// `Merged` is a wrapper to mark a feature as being produced by one or more different types.
///
/// For example: `FeatureFrom<Result<A,B>>` or any featurizer produced by composing two featurizers with different outputs (like `bookends` or `featurizers!`)
/// ```
/// //neither of these will compile without Merged
/// let feats: Vec<Merged<String>> = featurizers!(ftzr1, ftzr2).featurize(your_data);
/// let feats: Vec<Merged<HashedAs<u16>>> = bookends((ftzr1, 3), (ftzr2, 17)).featurize(your_data);
/// ```
#[derive(Hash, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Merged<T>(pub T);

#[derive(Hash, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub(crate) struct Output<T>(pub T);

#[derive(Hash, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub(crate) struct SelfOut<T>(pub T);

#[derive(Hash, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub(crate) struct Collisions<Heap, V, T>(pub T, PhantomData<(Heap, V)>);

impl<Heap, V, T> Collisions<Heap, V, T> {
    pub(crate) fn new(t: T) -> Self {
        Collisions(t, PhantomData::default())
    }
}

macro_rules! impl_deref {
    ($t:ty) => {
        impl<T> Deref for $t {
            type Target = T;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}

impl_deref!(Output<T>);
impl_deref!(Bag<T>);
impl_deref!(Merged<T>);
impl_deref!(SelfOut<T>);

const UNICODE_ERR_MSG: &str = 
    "Featurizing as a &str is only supported for ascii. Please use &[u8] or Vec<char>. (This is at the bottom of the first page of the docs.)";

impl<'a> From<Output<&'a [u8]>> for &'a str {
    fn from(token_group: Output<&'a [u8]>) -> Self {
        from_utf8(&*token_group).expect(UNICODE_ERR_MSG)
    }
}

impl<'a, T> From<Output<&'a [T]>> for &'a [T] {
    fn from(token_group: Output<&'a [T]>) -> Self {
        &*token_group
    }
}

impl<T, const N: usize> From<Output<[T; N]>> for [T; N] {
    fn from(token_group: Output<[T; N]>) -> Self {
        token_group.0
    }
}

impl<const N: usize> From<Output<[u8; N]>> for String {
    fn from(token_group: Output<[u8; N]>) -> Self {
        from_utf8(&*token_group).expect(UNICODE_ERR_MSG).to_owned()
    }
}
impl<const N: usize> From<Output<[char; N]>> for String {
    fn from(token_group: Output<[char; N]>) -> Self {
        Iterator::collect(token_group.0.iter())
    }
}

impl<'a> From<Output<&'a [char]>> for String {
    fn from(token_group: Output<&'a [char]>) -> Self {
        let mut s = String::default();
        for c in token_group.0 {
            s.push(*c);
        }
        s
    }
}

macro_rules! group_from_iter {
    ($iter:ty) => {
        impl<Feature1: Copy, Feature2: From<Feature1>, const N: usize> From<Output<[Feature1; N]>>
            for $iter
        {
            fn from(token_group: Output<[Feature1; N]>) -> Self {
                Self::from_iter(token_group.0.iter().map(|x| From::from(*x)))
            }
        }
    };
}

group_from_iter!(Box<[Feature2]>);
group_from_iter!(Rc<[Feature2]>);
group_from_iter!(Arc<[Feature2]>);
group_from_iter!(VecDeque<Feature2>);
//group_from_iter!(BTreeMap);
group_from_iter!(Vec<Feature2>);
group_from_iter!(LinkedList<Feature2>);
//TODO make hashset work for any hasher
//group_from_iter!(HashSet<Feature>);

//requires cmp
//group_from_iter!(BinaryHeap<Feature>);
//group_from_iter!(BTreeSet<Feature>);
