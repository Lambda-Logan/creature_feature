use fxhash::FxHasher64;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::{BuildHasher, Hash, Hasher};
use std::ops::AddAssign;

#[cfg(feature = "serde1")]
use serde::{Deserialize, Serialize};

pub trait Accumulates<Token> {
    type State: Default;
    fn accum_token(state: &mut Self::State, token: Token);

    fn finish(state: Self::State) -> Self;

    ////////////////////////////////////
    ////////////////////////////////////

    fn accum_token_at_idx(state: &mut Self::State, token: Token, idx: isize) {
        Self::accum_token(state, token);
    }

    fn default_at_idx(idx: isize) -> Self::State {
        Default::default()
    }

    fn finish_at_idx(state: Self::State, idx: isize) -> Self::State {
        state
    }
}

#[derive(Hash, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Debug)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct Feature64(u64);

impl<Token: Eq + Hash> Accumulates<Token> for Feature64 {
    type State = FxHasher64;

    fn accum_token(state: &mut Self::State, token: Token) {
        &token.hash(state);
    }

    fn finish(state: Self::State) -> Self {
        Feature64(state.finish())
    }
}

impl<Token, A, B> Accumulates<Token> for (A, B)
where
    Token: Clone,
    A: Accumulates<Token>,
    B: Accumulates<Token>,
{
    type State = (A::State, B::State);
    fn accum_token(state: &mut Self::State, token: Token) {
        A::accum_token(&mut state.0, token.clone());
        B::accum_token(&mut state.1, token);
    }

    fn finish(state: Self::State) -> Self {
        (A::finish(state.0), B::finish(state.1))
    }

    ////////////////////////////////////
    ////////////////////////////////////

    fn accum_token_at_idx(state: &mut Self::State, token: Token, idx: isize) {
        A::accum_token_at_idx(&mut state.0, token.clone(), idx);
        B::accum_token_at_idx(&mut state.1, token, idx);
    }

    fn default_at_idx(idx: isize) -> Self::State {
        (A::default_at_idx(idx), B::default_at_idx(idx))
    }
}

impl<Token> Accumulates<Token> for Vec<Token> {
    type State = Self;
    fn accum_token(state: &mut Self, token: Token) {
        state.push(token);
    }
    fn finish(state: Self) -> Self {
        state
    }
}

impl<Token: Eq + Hash, S: Default + BuildHasher> Accumulates<Token> for HashSet<Token, S> {
    type State = Self;
    fn accum_token(state: &mut Self, token: Token) {
        state.insert(token);
    }
    fn finish(state: Self) -> Self {
        state
    }
}

impl<Token, N, S> Accumulates<Token> for HashMap<Token, N, S>
where
    Token: Eq + Hash,
    N: Default + AddAssign + From<u8>,
    S: Default + BuildHasher,
{
    type State = Self;
    fn accum_token(state: &mut Self, token: Token) {
        *state.entry(token).or_default() += From::from(1);
    }
    fn finish(state: Self) -> Self {
        state
    }
}

impl Accumulates<&[&str]> for String {
    type State = Self;

    #[inline]
    fn accum_token(state: &mut Self::State, tokens: &[&str]) {
        let mut x = 0;
        let len = tokens.len();
        for t in tokens.iter() {
            state.push_str(*t);
            if x != len - 1 {
                state.push_str("  ");
            }
            x += 1
        }
    }
    #[inline]
    fn finish(state: Self::State) -> Self {
        state
    }
    #[inline]
    fn accum_token_at_idx(state: &mut Self::State, tokens: &[&str], idx: isize) {
        Self::accum_token(state, tokens);
        state.push_str(format!(" @ {:?}", idx).as_str());
    }
}

impl<'a> Accumulates<&'a [char]> for String {
    type State = Self;

    #[inline]
    fn accum_token(state: &mut Self::State, tokens: &'a [char]) {
        state.extend(tokens.iter())
    }
    #[inline]
    fn finish(state: Self::State) -> Self {
        state
    }
    #[inline]
    fn accum_token_at_idx(state: &mut Self::State, tokens: &'a [char], idx: isize) {
        Self::accum_token(state, tokens);
        state.push_str(format!(" @ {:?}", idx).as_str());
    }
}

impl<'a> Accumulates<&'a [u8]> for String {
    type State = Self;

    #[inline]
    fn accum_token(state: &mut Self::State, tokens: &'a [u8]) {
        state.extend(tokens.iter().map(|token| {
            let chr: char = From::from(*token);
            chr
        }))
    }
    #[inline]
    fn finish(state: Self::State) -> Self {
        state
    }
    #[inline]
    fn accum_token_at_idx(state: &mut Self::State, tokens: &'a [u8], idx: isize) {
        Self::accum_token(state, tokens);
        state.push_str(format!(" @ {:?}", idx).as_str());
    }
}
///////////////////////////////////////////////
/*
impl<TokenPtr, Token, Tokens: IntoIterator<Item = TokenPtr>> Accumulates<Tokens> for String
where
    TokenPtr: ToOwned<Owned = Token>,
    char: From<Token>, //String: Extend<Token>,
{
    type State = Self;

    #[inline]
    fn accum_token(state: &mut Self::State, token: Tokens) {
        state.extend(token.into_iter().map(|x| {
            let t: Token = x.to_owned();
            let chr: char = From::from(t);
            chr
        }))
    }
    #[inline]
    fn finish(state: Self::State) -> Self {
        state
    }
    #[inline]
    fn accum_token_at_idx(state: &mut Self::State, token: Tokens, idx: isize) {
        if idx > 0 {
            state.push('_');
        }
        //state.extend(token.into_iter().map(|x| From::from(x.to_owned(): Token): char));
    }
}*/
