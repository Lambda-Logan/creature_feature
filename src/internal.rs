use std::marker::PhantomData;
use std::ops::Deref;

pub(crate) fn compose<A, B, C, F, G>(mut f: F, mut g: G) -> impl FnMut(A) -> C
where
    F: FnMut(B) -> C,
    G: FnMut(A) -> B,
{
    move |a| f(g(a))
}

pub(crate) trait OneDimCPS<T> {
    fn use_slice<U, F: FnOnce(&[T]) -> U>(&self, f: F) -> U;
}

impl<'a, T> OneDimCPS<T> for &'a [T] {
    fn use_slice<U, F: FnOnce(&[T]) -> U>(&self, f: F) -> U {
        f(self)
    }
}

impl<'a, T> OneDimCPS<T> for &'a Vec<T> {
    fn use_slice<U, F: FnOnce(&[T]) -> U>(&self, f: F) -> U {
        unimplemented!() // f(self.as_slice())
    }
}

impl<T> OneDimCPS<T> for Vec<T> {
    fn use_slice<U, F: FnOnce(&[T]) -> U>(&self, f: F) -> U {
        unimplemented!() // f(self.as_slice())
    }
}

impl<'a> OneDimCPS<u8> for &'a str {
    fn use_slice<U, F: FnOnce(&[u8]) -> U>(&self, f: F) -> U {
        f(self.as_bytes())
    }
}

impl<'a> OneDimCPS<u8> for &'a String {
    fn use_slice<U, F: FnOnce(&[u8]) -> U>(&self, f: F) -> U {
        f(self.as_bytes())
    }
}
