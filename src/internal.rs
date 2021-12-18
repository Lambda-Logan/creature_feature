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

            fn extract_tokens(&self, origin: &'a String) -> Self::Iter {
                self.extract_tokens(origin.as_str().as_bytes())
            }
        }
        /////////////////////////////////////////////////////
        /////////////////////////////////////////////////////
        /////////////////////////////////////////////////////
        /////////////////////////////////////////////////////
        /*
        impl<'a, X, Y> Ftzr<&'a str> for $self
        where
            Self: Ftzr<&'a [u8]>,
        {
            fn push_tokens<Push>(&self, origin: &'a str, push: &mut Push)
            where
                Push: FnMut(Self::TokenGroup) -> (),
            {
                self.push_tokens(origin.as_bytes(), push)
            }
        }

        impl<'a, X, Y> Ftzr<&'a String> for $self
        where
            Self: Ftzr<&'a [u8]>,
        {
            fn push_tokens<Push>(&self, origin: &'a str, push: &mut Push)
            where
                Push: FnMut(Self::TokenGroup) -> (),
            {
                self.push_tokens(origin.as_str().as_bytes(), push)
            }
        }

        impl<'a, X, Y> Ftzr<&'a String> for $self
        where
            Self: Ftzr<&'a [u8]>,
        {
            fn push_tokens<Push>(&self, origin: &'a str, push: &mut Push)
            where
                Push: FnMut(Self::TokenGroup) -> (),
            {
                self.push_tokens(origin.as_str().as_bytes(), push)
            }
        }

        impl<'a, X, Y, T> Ftzr<&'a Vec<T>> for $self
        where
            Self: Ftzr<&'a [T]>,
        {
            fn push_tokens<Push>(&self, origin: &'a Vec<T>, push: &mut Push)
            where
                Push: FnMut(Self::TokenGroup) -> (),
            {
                self.push_tokens(origin.as_slice(), push)
            }
        }

        impl<'a, X, Y, T, const N: usize> Ftzr<&'a [T; N]> for $self
        where
            Self: Ftzr<&'a [T]>,
        {
            fn push_tokens<Push>(&self, origin: &'a [T; N], push: &mut Push)
            where
                Push: FnMut(Self::TokenGroup) -> (),
            {
                self.push_tokens(&origin[..], push)
            }
        } */
    };
}
pub(crate) use impl_ftrzs_2;
