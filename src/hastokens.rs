use crate::accumulators::Accumulates;
use crate::featurizers::Featurizer;
use std::fmt::Debug;
use std::hash::Hash;

pub trait HasTokens {
    type TokenGroup: Clone;
    fn give_tokens_to<Ftzr, Push, Feature>(&self, ftzr: &Ftzr, push_feat: &mut Push)
    where
        Ftzr: Featurizer<Self::TokenGroup>,
        Feature: Accumulates<Self::TokenGroup>,
        Push: FnMut(Feature) -> ();
}

impl<T> HasTokens for &[T] {
    type TokenGroup = Self;
    fn give_tokens_to<Ftzr, Push, Feature>(&self, ftzr: &Ftzr, push_feat: &mut Push)
    where
        Ftzr: Featurizer<Self::TokenGroup>,
        Feature: Accumulates<Self::TokenGroup>,
        Push: FnMut(Feature) -> (),
    {
        ftzr.use_tokens_from(&self, push_feat);
    }
}

impl<'a, T> HasTokens for &'a Vec<T> {
    type TokenGroup = &'a [T];
    fn give_tokens_to<Ftzr, Push, Feature>(&self, ftzr: &Ftzr, push_feat: &mut Push)
    where
        Ftzr: Featurizer<Self::TokenGroup>,
        Feature: Accumulates<Self::TokenGroup>,
        Push: FnMut(Feature) -> (),
    {
        ftzr.use_tokens_from(&self, push_feat);
    }
}

impl<'a> HasTokens for &'a str {
    type TokenGroup = &'a [u8];
    fn give_tokens_to<Ftzr, Push, Feature>(&self, ftzr: &Ftzr, push_feat: &mut Push)
    where
        Ftzr: Featurizer<Self::TokenGroup>,
        Feature: Accumulates<Self::TokenGroup>,
        Push: FnMut(Feature) -> (),
    {
        let bytes = self.as_bytes();
        ftzr.use_tokens_from(bytes, push_feat);
    }
}

impl<'a> HasTokens for &'a String {
    type TokenGroup = <&'a str as HasTokens>::TokenGroup;
    fn give_tokens_to<Ftzr, Push, Feature>(&self, ftzr: &Ftzr, push_feat: &mut Push)
    where
        Ftzr: Featurizer<Self::TokenGroup>,
        Feature: Accumulates<Self::TokenGroup>,
        Push: FnMut(Feature) -> (),
    {
        self.as_str().give_tokens_to(ftzr, push_feat);
    }
}

pub fn chars_of(s: &str) -> Vec<char> {
    Iterator::collect(s.chars())
}
