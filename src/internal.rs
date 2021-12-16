use crate::token_from::TokenFrom;
use std::ops::Deref;

#[macro_use]
macro_rules! impl_ftrzs {
    ($self:ty) => {
        impl<'a, T> IterFtzr<&'a Vec<T>> for $self
        where
            Self: IterFtzr<&'a [T]>,
        {
            type TokenGroup = <Self as IterFtzr<&'a [T]>>::TokenGroup;
            type Iter = <Self as IterFtzr<&'a [T]>>::Iter;
            fn chunk_size(&self) -> usize {
                unimplemented!() //<self as IterFtzr<&'a [T]>>.chunk_size()
            }
            fn extract_tokens(&self, origin: &'a Vec<T>) -> Self::Iter {
                self.extract_tokens(origin.as_slice())
            }
        }
        impl<'a, T, const N: usize> IterFtzr<&'a [T; N]> for $self
        where
            Self: IterFtzr<&'a [T]>,
        {
            type TokenGroup = <Self as IterFtzr<&'a [T]>>::TokenGroup;
            type Iter = <Self as IterFtzr<&'a [T]>>::Iter;
            fn chunk_size(&self) -> usize {
                unimplemented!() //<self as IterFtzr<&'a [T]>>.chunk_size()
            }
            fn extract_tokens(&self, origin: &'a [T; N]) -> Self::Iter {
                self.extract_tokens(&origin[..])
            }
        }
        impl<'a> IterFtzr<&'a str> for $self
        where
            Self: IterFtzr<&'a [u8]>,
        {
            type TokenGroup = <Self as IterFtzr<&'a [u8]>>::TokenGroup;
            type Iter = <Self as IterFtzr<&'a [u8]>>::Iter;
            fn chunk_size(&self) -> usize {
                unimplemented!() //<self as IterFtzr<&'a [T]>>.chunk_size()
            }
            fn extract_tokens(&self, origin: &'a str) -> Self::Iter {
                self.extract_tokens(origin.as_bytes())
            }
        }

        impl<'a> IterFtzr<&'a String> for $self
        where
            Self: IterFtzr<&'a [u8]>,
        {
            type TokenGroup = <Self as IterFtzr<&'a [u8]>>::TokenGroup;
            type Iter = <Self as IterFtzr<&'a [u8]>>::Iter;
            fn chunk_size(&self) -> usize {
                unimplemented!() //<self as IterFtzr<&'a [T]>>.chunk_size()
            }
            fn extract_tokens(&self, origin: &'a String) -> Self::Iter {
                self.extract_tokens(origin.as_str().as_bytes())
            }
        }

        impl<Origin> Ftzr<Origin> for $self
        where
            Self: IterFtzr<Origin>,
        {
            type TokenGroup = <Self as IterFtzr<Origin>>::TokenGroup;
            fn push_tokens<Push>(&self, origin: Origin, push: &mut Push)
            where
                Push: FnMut(Self::TokenGroup) -> (),
            {
                for t in self.extract_tokens(origin) {
                    push(t)
                }
            }
        }
    };
}
pub(crate) use impl_ftrzs;

macro_rules! impl_ftrzs_2 {
    ($self:ty) => {
        impl<'a, Y, X, T> IterFtzr<&'a Vec<T>> for $self
        where
            Self: IterFtzr<&'a [T]>,
        {
            type TokenGroup = <Self as IterFtzr<&'a [T]>>::TokenGroup;
            type Iter = <Self as IterFtzr<&'a [T]>>::Iter;
            fn chunk_size(&self) -> usize {
                unimplemented!() //<self as IterFtzr<&'a [T]>>.chunk_size()
            }
            fn extract_tokens(&self, origin: &'a Vec<T>) -> Self::Iter {
                self.extract_tokens(origin.as_slice())
            }
        }
        impl<'a, T, Y, X, const N: usize> IterFtzr<&'a [T; N]> for $self
        where
            Self: IterFtzr<&'a [T]>,
        {
            type TokenGroup = <Self as IterFtzr<&'a [T]>>::TokenGroup;
            type Iter = <Self as IterFtzr<&'a [T]>>::Iter;
            fn chunk_size(&self) -> usize {
                unimplemented!() //<self as IterFtzr<&'a [T]>>.chunk_size()
            }
            fn extract_tokens(&self, origin: &'a [T; N]) -> Self::Iter {
                self.extract_tokens(&origin[..])
            }
        }
        impl<'a, Y, X> IterFtzr<&'a str> for $self
        where
            Self: IterFtzr<&'a [u8]>,
        {
            type TokenGroup = <Self as IterFtzr<&'a [u8]>>::TokenGroup;
            type Iter = <Self as IterFtzr<&'a [u8]>>::Iter;
            fn chunk_size(&self) -> usize {
                unimplemented!() //<self as IterFtzr<&'a [T]>>.chunk_size()
            }
            fn extract_tokens(&self, origin: &'a str) -> Self::Iter {
                self.extract_tokens(origin.as_bytes())
            }
        }

        impl<'a, Y, X> IterFtzr<&'a String> for $self
        where
            Self: IterFtzr<&'a [u8]>,
        {
            type TokenGroup = <Self as IterFtzr<&'a [u8]>>::TokenGroup;
            type Iter = <Self as IterFtzr<&'a [u8]>>::Iter;
            fn chunk_size(&self) -> usize {
                unimplemented!() //<self as IterFtzr<&'a [T]>>.chunk_size()
            }
            fn extract_tokens(&self, origin: &'a String) -> Self::Iter {
                self.extract_tokens(origin.as_str().as_bytes())
            }
        }
    };
}

pub(crate) use impl_ftrzs_2;
