#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::collections::{LinkedList, VecDeque};
use std::hash::Hash;
use std::iter::FromIterator;
use std::ops::Deref;
use std::rc::Rc;
use std::str::from_utf8;
use std::sync::Arc;

#[derive(Hash, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Token<T>(pub T);

impl<T> Deref for Token<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn chars_of(s: &str) -> Vec<char> {
    Iterator::collect(s.chars())
}

impl<'a> From<Token<&'a [u8]>> for &'a str {
    fn from(token_group: Token<&'a [u8]>) -> Self {
        from_utf8(&*token_group).unwrap()
    }
}

impl<'a, T> From<Token<&'a [T]>> for &'a [T] {
    fn from(token_group: Token<&'a [T]>) -> Self {
        &*token_group
    }
}

impl<T, const N: usize> From<Token<[T; N]>> for [T; N] {
    fn from(token_group: Token<[T; N]>) -> Self {
        token_group.0
    }
}

impl<const N: usize> From<Token<[u8; N]>> for String {
    fn from(token_group: Token<[u8; N]>) -> Self {
        from_utf8(&*token_group).unwrap().to_owned()
    }
}
impl<const N: usize> From<Token<[char; N]>> for String {
    fn from(token_group: Token<[char; N]>) -> Self {
        Iterator::collect(token_group.0.iter())
    }
}

impl<'a> From<Token<&'a [char]>> for String {
    fn from(token_group: Token<&'a [char]>) -> Self {
        let mut s = String::default();
        for c in token_group.0 {
            s.push(*c);
        }
        s
    }
}
macro_rules! group_from_iter {
    ($iter:ty) => {
        /*impl<Feature1, Feature2: From<Feature1>, Q: IntoIterator<Item = Feature1>>
            From<Token<Q>> for $iter
        {
            fn from(token_group: Token<Q>) -> Self {
                Self::from_iter(&*token_group.into_iter().map(From::from))
            }
        } */

        impl<Feature1: Copy, Feature2: From<Feature1>, const N: usize> From<Token<[Feature1; N]>>
            for $iter
        {
            fn from(token_group: Token<[Feature1; N]>) -> Self {
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
