use crate::convert::Output;
use std::str::from_utf8;

///The trait for features that can be made from another type. It's largely self-explanatory and very similar to [`std::convert::From`], only more accommodating. It's used in `Ftzr::featurize` and `Ftzr::push_tokens_from`.
/// ```
///use creature_feature::traits::FeatureFrom;
///use creature_feature::HashedAs;
///
///let data: &[u8] = &[98, 97, 99, 111, 110];
///
///let out1: String = FeatureFrom::from(data);
///
///let out2: HashedAs<u64> = FeatureFrom::from(data);
///
///println!("{:?} can be {:?}", out1, out2);
///
/// //>> "bacon" can be HashedAs(13832527907573695876)
/// ```
pub trait FeatureFrom<T> {
    #[allow(missing_docs)]
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

const UTF_ERR_MSG: &'static str = &"Featurizing into &str is only supported for ASCII, not unicode. Please first convert your input data to a Vec<char>. This is noted on the first page of the docs, at the bottom.";

impl<const N: usize> FeatureFrom<[u8; N]> for String {
    fn from(token_group: [u8; N]) -> Self {
        from_utf8(&token_group).expect(UTF_ERR_MSG).to_owned()
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
        from_utf8(token_group).expect(UTF_ERR_MSG)
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

impl<A: Copy, B: FeatureFrom<A>> FeatureFrom<[A; 1]> for Output<B> {
    fn from(token_group: [A; 1]) -> Self {
        Output(FeatureFrom::from(token_group[0]))
    }
}

impl FeatureFrom<String> for String {
    fn from(token_group: String) -> Self {
        token_group
    }
}

impl<'a> FeatureFrom<&'a [u8]> for String {
    fn from(token_group: &'a [u8]) -> Self {
        from_utf8(token_group).expect(UTF_ERR_MSG).to_owned()
    }
}

impl<A, B, C> FeatureFrom<Result<A, B>> for Output<C>
where
    C: FeatureFrom<A> + FeatureFrom<B>,
{
    fn from(r: Result<A, B>) -> Output<C> {
        Output(match r {
            Err(x) => FeatureFrom::from(x),
            Ok(x) => FeatureFrom::from(x),
        })
    }
}
