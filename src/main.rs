mod accumulators;
use accumulators::*;

mod featurizers;
use featurizers::*;

mod hastokens;
use hastokens::*;

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
        assert_len!($len, $ftzr, String, $origin);
        assert_len!($len, $ftzr, Feature64, $origin);
    };
}

fn main() {
    let abcde = "abcde";
    assert_str_lengths!(4, n_gram(2), abcde);
    assert_str_lengths!(4, n_gram(2), &abcde.to_string()); //needs & ???
    assert_str_lengths!(4, n_gram(2), &chars_of(abcde));
    let _1234: [u8; 4] = [1, 2, 3, 4];
    let x: Vec<Feature64> = n_gram(2).featurize(&_1234[..]);
    let x2: Vec<Vec<&[u8]>> = n_gram(2).featurize(&_1234[..]);

    ////////////////////  THIS NEEDS TO WORK
    //let x: Vec<Vec<u8>> = n_gram(2).featurize(&_1234[..]);

    let ftzr = featurizers::book_ends((3, 3), featurizers::n_gram(2));
    let sentence: Vec<&str> =
        Iterator::collect("one fish two fish red fish blue fish".split_ascii_whitespace());

    let feats: Vec<Feature64> = ftzr.featurize(&sentence);
    let feats2: Vec<Vec<&[&str]>> = ftzr.featurize(&sentence);

    /////////////////// THIS NEEDS TO WORK
    //let feats: Vec<Vec<&str>> = ftzr.featurize(&sentence);

    // what about this?
    //let feats2: Vec<&[&str]> = ftzr.featurize(&sentence); ???

    let mut s = "".to_string();

    println!("{:?}", feats);
}
