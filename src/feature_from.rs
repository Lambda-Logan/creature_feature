use crate::tokengroup::Token;
use std::str::from_utf8;
pub trait FeatureFrom<T> {
    fn from(t: T) -> Self;
}

impl<'a, T> FeatureFrom<&'a [T]> for &'a [T] {
    fn from(token_group: &'a [T]) -> Self {
        token_group
    }
}

impl<T, const N: usize> FeatureFrom<[T; N]> for [T; N] {
    fn from(token_group: [T; N]) -> Self {
        token_group
    }
}

impl<T: Clone, const N: usize> FeatureFrom<[T; N]> for Vec<T> {
    fn from(token_group: [T; N]) -> Self {
        token_group.to_vec()
    }
}

impl<const N: usize> FeatureFrom<[u8; N]> for String {
    fn from(token_group: [u8; N]) -> Self {
        from_utf8(&token_group).unwrap().to_owned()
    }
}
impl<const N: usize> FeatureFrom<[char; N]> for String {
    fn from(token_group: [char; N]) -> Self {
        Iterator::collect(token_group.iter())
    }
}

impl<'a> FeatureFrom<&'a [char]> for String {
    fn from(token_group: &'a [char]) -> Self {
        let mut s = String::default();
        for c in token_group {
            s.push(*c);
        }
        s
    }
}

impl<'a> FeatureFrom<&'a [u8]> for &'a str {
    fn from(token_group: &'a [u8]) -> Self {
        from_utf8(token_group).unwrap()
    }
}
impl<'a> FeatureFrom<&'a str> for &'a str {
    fn from(token_group: &'a str) -> Self {
        token_group
    }
}

impl<'a> FeatureFrom<&'a str> for String {
    fn from(token_group: &'a str) -> Self {
        token_group.to_string()
    }
}

impl<A: Copy, B: FeatureFrom<A>> FeatureFrom<[A; 1]> for Token<B> {
    fn from(token_group: [A; 1]) -> Self {
        Token(FeatureFrom::from(token_group[0]))
    }
}

impl FeatureFrom<String> for String {
    fn from(token_group: String) -> Self {
        token_group
    }
}

impl<'a> FeatureFrom<&'a [u8]> for String {
    fn from(token_group: &'a [u8]) -> Self {
        from_utf8(token_group).unwrap().to_owned()
    }
}

impl<A, B, C> FeatureFrom<Result<A, B>> for Token<C>
where
    C: FeatureFrom<A> + FeatureFrom<B>,
{
    fn from(r: Result<A, B>) -> Token<C> {
        Token(match r {
            Err(x) => FeatureFrom::from(x),
            Ok(x) => FeatureFrom::from(x),
        })
    }
}
