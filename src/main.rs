//mod accumulators;
//use accumulators::*;
#![allow(unused)]
//#[allow(warnings, unused)]
mod for_each;

#[macro_use]
mod internal;
mod token_from;

mod whole_empty;

mod bench;

mod tokengroup;
use tokengroup::*;

mod bookends;
use bookends::*;

mod accum_ftzr;
use accum_ftzr::*;

mod slice_gram;
use slice_gram::*;

mod multiftzr;
use multiftzr::*;

mod gap_gram;
use gap_gram::*;

mod n_gram;
use n_gram::*;

mod hashedfeature;
use hashedfeature::*;

mod compile_checks;

//mod featurizers;
//use featurizers::*;

//mod hastokens;
//use hastokens::*;

use std::collections::HashSet;
use std::hash::Hash;

use std::iter::FromIterator;

macro_rules! assert_len {
    ($len:expr, $ftzr:expr, $t:ty, $origin:expr) => {
        let x: HashSet<$t> = $ftzr.featurize($origin);
        assert_eq!($len, x.len());
    };
}

macro_rules! assert_str_lengths {
    ($len: expr, $ftzr: expr, $origin: expr) => {
        //    assert_len!($len, $ftzr, String, $origin);
        //    assert_len!($len, $ftzr, Feature64, $origin);
    };
}

fn main() {
    let abcde = "abcde";
    assert_str_lengths!(4, slice_gram(2), abcde);
    assert_str_lengths!(4, slice_gram(2), &abcde.to_string()); //needs & ???
    assert_str_lengths!(4, slice_gram(2), &chars_of(abcde));
    let _1234: [u8; 4] = [1, 2, 3, 4];
    let x: Vec<Feature64> = slice_gram(2).featurize(&_1234[..]);
    //let x2: Vec<Vec<_>> = slice_gram(2).featurize(&_1234[..]);
    let x2: Vec<&[u8]> = slice_gram(2).featurize(&_1234[..]);

    ////////////////////  THIS NEEDS TO WORK
    //let x: Vec<Vec<u8>> = slice_gram(2).featurize(&_1234[..]);

    //let ftzr = featurizers::book_ends((3, 3), featurizers::slice_gram(2));
    let sentence: Vec<&str> =
        Iterator::collect("one fish two fish red fish blue fish".split_ascii_whitespace());

    //let feats: Vec<Feature64> = ftzr.featurize(&sentence);
    //let feats2: Vec<Vec<&[&str]>> = ftzr.featurize(&sentence);
    let s: Vec<&[&str]> = slice_gram(3).featurize(&sentence);
    let gram: NGram<2> = Default::default();
    let kip = gap_gram(gram, 2, gram);
    /// GapGram<_, _, 2> = GapGram(gram, gram);
    let skip = gap_gram(kip, 2, kip);
    /// GapGram<_, _, 2> = GapGram(kip, kip.clone());
    let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17];

    let az = "abcdefghijklmnopqrstuvqxyz";
    //cjk(kip, vec.as_slice());
    //let h: &[i32] = &vec.one_dim();
    let v: Vec<[i32; 2]> = gram.featurize(&vec);

    //this needs to work as well
    //let v: Vec<([i32; 2], [i32; 2])> = kip.featurize(&vec);
    let v: Vec<(Vec<i32>, Vec<i32>)> = kip.featurize(vec.as_slice());
    let v: Vec<([_; 2], [_; 2])> = kip.featurize(chars_of(az).as_slice());
    let v: Vec<(String, String)> = kip.featurize(chars_of(az).as_slice());
    let v: Vec<(String, String)> = kip.featurize(az);

    //TODO
    //let v: Vec<GapPair<_, _>> = skip.featurize(vec.as_slice());
    let v: Vec<(([_; 2], [_; 2]), ([_; 2], [_; 2]))> = skip.featurize(vec.as_slice());
    let v: Vec<&str> = slice_gram(4).featurize(az);
    let v: Vec<(&[i32], &[_])> =
        gap_gram(slice_gram(5), 4, slice_gram(2)).featurize(vec.as_slice());
    //let fnn = slice_gram(3).as_fn::<Feature64>();
    //let v: Vec<([_; 2], [_; 2], [_; 2], [_; 2])> = skip.featurize(vec.as_slice());
    //let v: TokenGroup<i32> = From::from(CopyGroup(&2));
    //let v: Vec<(&str, &str)> = kip.featurize("abcdefghijklmnopqrs");
    //let v: Vec<String> = gram.featurize(&chars_of("abcde"));
    //println!("\n{:?}\n", v);

    /////////////////// THIS NEEDS TO WORK
    //let feats: Vec<Vec<&str>> = ftzr.featurize(&sentence);

    // what about this?
    //let feats2: Vec<&[&str]> = ftzr.featurize(&sentence); ???
    {
        //use ngrams::{Ngram, Ngrams};
        //let feats: Vec<_> = Ngrams::new("one two three".bytes(), 2).collect();
        //println!("{:?}", feats);
    }
    compile_checks::run_checks();

    //bench::bench();
}
