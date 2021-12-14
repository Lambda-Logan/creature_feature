use std::str::from_utf8;

pub trait TokenFrom<T> {
    fn from(t: T) -> Self;
}

impl<'a, T> TokenFrom<&'a [T]> for &'a [T] {
    fn from(token_group: &'a [T]) -> Self {
        token_group
    }
}

impl<T, const N: usize> TokenFrom<[T; N]> for [T; N] {
    fn from(token_group: [T; N]) -> Self {
        token_group
    }
}

impl<T: Clone, const N: usize> TokenFrom<[T; N]> for Vec<T> {
    fn from(token_group: [T; N]) -> Self {
        token_group.to_vec()
    }
}

impl<const N: usize> TokenFrom<[u8; N]> for String {
    fn from(token_group: [u8; N]) -> Self {
        from_utf8(&token_group).unwrap().to_owned()
    }
}
impl<const N: usize> TokenFrom<[char; N]> for String {
    fn from(token_group: [char; N]) -> Self {
        Iterator::collect(token_group.iter())
    }
}

impl<'a> TokenFrom<&'a [char]> for String {
    fn from(token_group: &'a [char]) -> Self {
        let mut s = String::default();
        for c in token_group {
            s.push(*c);
        }
        s
    }
}

impl<'a> TokenFrom<&'a [u8]> for &'a str {
    fn from(token_group: &'a [u8]) -> Self {
        from_utf8(token_group).unwrap()
    }
}

impl<'a> TokenFrom<&'a [u8]> for String {
    fn from(token_group: &'a [u8]) -> Self {
        from_utf8(token_group).unwrap().to_owned()
    }
}
