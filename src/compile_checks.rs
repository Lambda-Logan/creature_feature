#![allow(warnings, unused)]

use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};

use crate::feature_from::FeatureFrom;
use crate::tokengroup::*;
use crate::whole_empty::*;

use crate::convert::*;

use crate::for_each::*;

use crate::multiftzr::*;

use crate::accum_ftzr::*;

use crate::n_slice::*;

use crate::gap_gram::*;

use crate::n_gram::*;

use crate::hashedfeature::HashedAs;

use crate::bookends::*;

fn featurize<Out, Origin, F: Ftzr<Origin>>(f: F, o: Origin) -> Vec<Out>
where
    Out: FeatureFrom<F::TokenGroup>,
{
    f.featurize(o)
}

macro_rules! test_vec_feats {
    ($f:expr,  $out:ty, $v:expr) => {
        let _feats: Vec<$out> = $f;
        assert_eq!(_feats, $v);
    };
}

macro_rules! featurizers {
    ($a:expr) => {
        $a
    };
    ($a:expr $(, $tail:expr)*) => {{
        MultiFtzr($a, featurizers!($($tail), *),
    )
    }};
}

pub(crate) fn run_checks() {
    let bigram = n_gram::<2>();
    let bislice = n_slice(2);
    let g_bigram = gap_gram(n_gram::<1>(), 0, n_gram::<1>());
    let g_g_bigram = gap_gram(g_bigram, 2, g_bigram);
    let g_s_bigram = gap_gram(bislice, 2, bislice);
    let sentence = "one fish two fish red rish blue fish";
    let ak_bigrams = &["ab", "bc", "cd", "de", "ef", "fg", "gh", "hi", "ij", "jk"];
    let ak_bigrams_feat64: &[HashedAs<u64>] = &[
        HashedAs(7222436297203265833),
        HashedAs(488219265294888090),
        HashedAs(12200746307096061963),
        HashedAs(4415645335814054341),
        HashedAs(17790974485650316728),
        HashedAs(11004783825300746331),
        HashedAs(16845529860537917751),
        HashedAs(8876065120756845939),
        HashedAs(14716811155994017359),
        HashedAs(14633802556225993672),
    ];
    let bigrams_12_usize = &[
        [0, 1],
        [1, 2],
        [2, 3],
        [3, 4],
        [4, 5],
        [5, 6],
        [6, 7],
        [7, 8],
        [8, 9],
        [9, 10],
        [10, 11],
    ];
    let bigrams_12_usize_feats64 = &[
        HashedAs(3351245001697020842),
        HashedAs(3476130915693767771),
        HashedAs(11967000998307149924),
        HashedAs(13702715391897792713),
        HashedAs(17297361034655441419),
        HashedAs(10615117631188256330),
        HashedAs(12402805653220091773),
        HashedAs(1615530608106378053),
        HashedAs(13391796650981597793),
        HashedAs(13464708936537152068),
        HashedAs(16031824384145339114),
    ];
    let ak: &str = "abcdefghijk";
    let n_usize_12: &[usize; 12] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    ////////////////////////////
    //         &str
    ////////////////////////////
    {
        //        NGram<2>
        test_vec_feats!(bigram.featurize(ak), String, ak_bigrams);
        test_vec_feats!(bigram.featurize(ak), HashedAs<u64>, ak_bigrams_feat64);

        //        SliceGram
        test_vec_feats!(bislice.featurize(ak), &str, ak_bigrams); // NOTICE &str
        test_vec_feats!(bislice.featurize(ak), HashedAs<u64>, ak_bigrams_feat64);

        //        GapGram
        let _feats: Vec<(String, String)> = gap_gram(bigram, 2, bigram).featurize(ak);
        let _feats: Vec<(&str, &str)> = gap_gram(bislice, 2, bislice).featurize(ak);
        let _feats: Vec<(HashedAs<u64>, HashedAs<u64>)> = g_g_bigram.featurize(ak);
    }

    ////////////////////////////
    //         &String
    ////////////////////////////

    {
        let t: &String = &ak.to_owned();
        //        NGram<2>
        test_vec_feats!(bigram.featurize(ak), String, ak_bigrams);
        test_vec_feats!(bigram.featurize(ak), HashedAs<u64>, ak_bigrams_feat64);

        //        SliceGram
        test_vec_feats!(bislice.featurize(ak), &str, ak_bigrams);
        test_vec_feats!(bislice.featurize(ak), HashedAs<u64>, ak_bigrams_feat64);

        //        GapGram
        let _feats: Vec<(String, String)> = gap_gram(bigram, 2, bigram).featurize(t);
        let _feats: Vec<(&str, &str)> = gap_gram(bislice, 2, bislice).featurize(t);
        let _feats: Vec<(HashedAs<u64>, HashedAs<u64>)> = g_g_bigram.featurize(t);
    }

    ////////////////////////////
    //         &[usize; 12]      (MUST USE &x[..])
    ////////////////////////////
    {
        ////////////////NGram<2>
        test_vec_feats!(
            bigram.featurize(&n_usize_12[..]),
            [usize; 2],
            bigrams_12_usize
        );
        test_vec_feats!(
            bigram.featurize(&n_usize_12[..]),
            Vec<usize>,
            bigrams_12_usize
        );
        test_vec_feats!(
            bigram.featurize(&n_usize_12[..]),
            HashedAs<u64>,
            bigrams_12_usize_feats64
        );

        //////////////SliceGram
        test_vec_feats!(
            bislice.featurize(&n_usize_12[..]),
            &[usize],
            bigrams_12_usize
        ); //NOTICE &[usize]
        let _feats: Vec<&[usize]> = bislice.featurize(&n_usize_12[..]); //NOTICE &[usize]
                                                                        // NOTICE ... no Vec<usize>
        test_vec_feats!(
            bislice.featurize(&n_usize_12[..]),
            HashedAs<u64>,
            bigrams_12_usize_feats64
        );

        ////////// GapGram
        let _feats: Vec<([usize; 2], [usize; 2])> =
            gap_gram(bigram, 2, bigram).featurize(&n_usize_12[..]);
        let _feats: Vec<(&[usize], &[usize])> =
            gap_gram(bislice, 2, bislice).featurize(&n_usize_12[..]);
        let _feats: Vec<(HashedAs<u64>, HashedAs<u64>)> = g_g_bigram.featurize(&n_usize_12[..]);
    }
    let _feats: Vec<Result<Vec<usize>, HashedAs<u64>>> =
        bookends((bigram, 4), (bigram, 4)).featurize(n_usize_12);
    let _feats: Vec<Merged<Vec<usize>>> = //TODO is this ok????
        bookends((bigram, 4), (bigram, 4)).featurize(n_usize_12);
    //let e: EmptyFtzr<usize> = EmptyFtzr::new();

    let _feats: Vec<Merged<String>> = featurizers!(bislice, n_gram::<3>()).featurize(ak);

    let _feats: Vec<Merged<HashedAs<u64>>> = featurizers!(bislice, n_gram::<3>()).featurize(ak);
    //let _feats: Vec<EitherGroup<_, _>> = MultiFtzr(bigram, n_gram::<3>()).featurize(ak);

    //TODO
    //let _feats: Vec<HashedAs> = gap_gram(bislice, 4, bigram).featurize(n_usize_12);
    let _feats: Vec<HashedAs<u64>> = gap_gram(bislice, 4, bigram).featurize(&n_usize_12[..]);
    use std::collections::LinkedList;
    //TODO Linked list & friends
    //let _feats: LinkedList<(&str, String)> = gap_gram(bislice, 4, n_gram::<3>()).featurize(ak);
    let _feats: Bag<HashMap<&str, u8>> = bislice.featurize(ak);

    let _feats: Vec<&str> = for_each(bislice).featurize(sentence.split_ascii_whitespace());

    let doc = r#"
    i went to the market
    to buy a fat pig
    home again home again
    jig itty jig
"#;
    let doc_iter = doc.lines().map(|line| line.split_ascii_whitespace());

    //TODO
    //let _feats: Vec<Merged<HashedAs>> =
    //    for_each(for_each(bislice.and_then(n_slice(3)))).featurize(doc_iter);
    //let _feats: Vec<Merged<String>> = n_gram::<2>().and_then(n_gram::<3>()).featurize(doc);
    let _feats: Vec<Merged<String>> = featurizers!(bigram, n_gram::<3>()).featurize(doc);
    let _feats: Vec<Merged<&str>> =
        for_each(featurizers!(bislice, n_slice(3))).featurize(doc.split_ascii_whitespace());
    let vecdoc: Vec<&str> = Iterator::collect(doc.split_ascii_whitespace());
    let _feats: Vec<((&[&str], &[&str]), (&[&str], &[&str]))> =
        gap_gram(g_s_bigram, 2, g_s_bigram).featurize(&vecdoc);

    struct SizeBasedFtzr;

    impl<'a> Ftzr<&'a str> for SizeBasedFtzr {
        type TokenGroup = &'a str;
        fn push_tokens<Push>(&self, origin: &'a str, push: &mut Push)
        where
            Push: FnMut(Self::TokenGroup) -> (),
        {
            if origin.len() < 6 {
                n_slice(2).push_tokens_from(origin, push);
            } else {
                n_slice(4).push_tokens_from(origin, push);
            }
        }
    }

    let _feats: Vec<&str> = SizeBasedFtzr.featurize(&"this ");
    let _feats: Vec<&str> = whole().featurize(ak);
    let _feats: Vec<Merged<&str>> = featurizers!(whole(), SizeBasedFtzr).featurize(ak);
    let _feats: Vec<&str> = for_each(SizeBasedFtzr).featurize(doc.split_ascii_whitespace());

    let nums: Vec<_> = Iterator::collect((0..32));
    let every_other = gap_gram(n_gram::<1>(), 1, n_gram::<1>());
    let _feats: Vec<([i32; 1], [i32; 1], [i32; 1], [i32; 1])> =
        gap_gram(every_other, 1, every_other).featurize(&nums);
    let _feats: Vec<([[i32; 1]; 4])> = gap_gram(every_other, 1, every_other).featurize(&nums);

    let _feats: (HashSet<HashedAs<u64>>, Vec<&str>) = bislice.featurize_x2(ak);
    //type Collide = Collisions<&[u8], HashMap<HashedAs<u16>, &str>>;
    //let _feats: Collisions<&[u8], HashMap<HashedAs<u16>, String>> =
    //    bislice.featurize::<&[u8], _>(ak);
    println!("{:?}", _feats);
}
