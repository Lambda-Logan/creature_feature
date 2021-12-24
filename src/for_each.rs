use crate::accum_ftzr::{Ftzr, IterFtzr, LinearFixed};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

/// A featurizer combinator that will featurize each item of an input iterator. Created with `for_each(ftzr)`.
#[derive(Hash, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ForEach<F, Meta>(F, PhantomData<Meta>);

#[derive(Hash, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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

impl<F: LinearFixed, T> LinearFixed for ForEach<F, T> {
    fn chunk_size(&self) -> usize {
        self.0.chunk_size()
    }
}

impl<F, Sentence, Word> IterFtzr<Sentence> for ForEach<F, (Sentence, Word)>
where
    Sentence: IntoIterator<Item = Word>,
    F: IterFtzr<Word>,
{
    type TokenGroup = F::TokenGroup;
    type Iter = ForEachIter<F, Sentence, Word, Sentence::IntoIter, F::Iter>;
    fn iterate_features(&self, origin: Sentence) -> Self::Iter {
        unimplemented!("ForEach does not yet impl IterFtzr... Feel free to file a PR")
    }
}

impl<F, Sentence, Word> Ftzr<Sentence> for ForEach<F, (Sentence, Word)>
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
/// A featurizer combinator that will featurize each item of an input iterator. Note that it, logically, is nestable: `for_each(for_each(ftzr))` will featurize an iterable of iterables.
/// ```        
/// use creature_feature::convert::Bag;
/// use creature_feature::ftzrs::{whole, for_each};
/// use std::collections::BTreeMap;
///
/// let sentence = "one fish two fish red fish blue fish"
///                 .split_ascii_whitespace();
///
/// let bag_of_words: Bag<BTreeMap<String, i32>> = for_each(whole()).featurize(sentence);
/// ```
pub fn for_each<F, Sentence, Word>(f: F) -> ForEach<F, (Sentence, Word)> {
    ForEach(f, PhantomData::default())
}
