use crate::accum_ftzr::{Ftzr, IterFtzr};
use std::iter::Fuse;
use std::marker::PhantomData;

#[derive(Hash, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Debug)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct ForEach<F, Sentence, Word>(F, PhantomData<(Sentence, Word)>);

#[derive(Hash, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Debug)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct ForEachIter<F, Sentence, Word, SentIter, FWordIter> {
    ftzr: F,
    sentence: SentIter,
    current: Option<FWordIter>,
    tags: PhantomData<(Sentence, Word)>,
}

impl<F, Sentence, Word, SentIter> Iterator
    for ForEachIter<F, Sentence, Word, SentIter, <F as IterFtzr<Word>>::Iter>
where
    SentIter: Iterator<Item = Word>,
    F: IterFtzr<Word>,
{
    type Item = F::TokenGroup;
    fn next(&mut self) -> Option<Self::Item> {
        /*
        TODO
        let mut ret: Option<Self::Item> = None;
        match self.current {
            Some(letters) => match letters.next() {
                None =>
            }
        } */
        unimplemented!("ForEach does not yet impl IterFtzr... Feel free to file a PR")
    }
}

impl<F, Sentence, Word> IterFtzr<Sentence> for ForEach<F, Sentence, Word>
where
    Sentence: IntoIterator<Item = Word>,
    F: IterFtzr<Word>,
{
    type TokenGroup = F::TokenGroup;
    type Iter = ForEachIter<F, Sentence, Word, Sentence::IntoIter, F::Iter>;
    fn extract_tokens(&self, origin: Sentence) -> Self::Iter {
        unimplemented!("ForEach does not yet impl IterFtzr... Feel free to file a PR")
    }
    fn chunk_size(&self) -> usize {
        self.0.chunk_size()
    }
}

impl<F, Sentence, Word> Ftzr<Sentence> for ForEach<F, Sentence, Word>
where
    Sentence: IntoIterator<Item = Word>,
    F: Ftzr<Word>,
{
    type TokenGroup = F::TokenGroup;
    fn push_tokens<Push>(&self, origin: Sentence, push: &mut Push)
    where
        Push: FnMut(Self::TokenGroup) -> (),
    {
        for word in origin.into_iter() {
            self.0.push_tokens(word, push);
        }
    }
}

pub fn for_each<F, Sentence, Word>(f: F) -> ForEach<F, Sentence, Word> {
    ForEach(f, PhantomData::default())
}
