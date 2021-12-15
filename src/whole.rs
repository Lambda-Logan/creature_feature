use crate::accum_ftzr::{Ftzr, IterFtzr};

#[derive(Hash, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Debug)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct Whole;

#[derive(Hash, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Debug)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct WholeAtom<T>(pub T);

impl<'a, T> IterFtzr<&'a [T]> for Whole {
    type TokenGroup = WholeAtom<&'a [T]>;
    type Iter = std::option::IntoIter<Self::TokenGroup>;
    fn chunk_size(&self) -> usize {
        unimplemented!()
    }
    fn extract_tokens(&self, origin: &'a [T]) -> Self::Iter {
        Some(WholeAtom(origin)).into_iter()
    }
}

impl<'a> IterFtzr<&'a str> for Whole {
    type TokenGroup = WholeAtom<&'a [u8]>;
    type Iter = std::option::IntoIter<Self::TokenGroup>;
    fn chunk_size(&self) -> usize {
        unimplemented!()
    }
    fn extract_tokens(&self, origin: &'a str) -> Self::Iter {
        self.extract_tokens(origin.as_bytes())
    }
}

impl<'a> IterFtzr<&'a String> for Whole {
    type TokenGroup = WholeAtom<&'a [u8]>;
    type Iter = std::option::IntoIter<Self::TokenGroup>;
    fn chunk_size(&self) -> usize {
        unimplemented!()
    }
    fn extract_tokens(&self, origin: &'a String) -> Self::Iter {
        self.extract_tokens(origin.as_str())
    }
}

impl<Origin> Ftzr<Origin> for Whole
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
