use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};

use crate::tokengroup::*;

use crate::token_from::TokenFrom;

use crate::for_each::*;

use crate::multiftzr::*;

use crate::accum_ftzr::*;

use crate::slice_gram::*;

use crate::gap_gram::*;

use crate::n_gram::*;

use crate::hashedfeature::*;

use crate::bookends::*;

fn featurize<Out, Origin, F: Ftzr<Origin>>(f: F, o: Origin) -> Vec<Out>
where
    Out: TokenFrom<F::TokenGroup>,
{
    f.featurize(o)
}

macro_rules! test_vec_feats {
    ($f:expr,  $out:ty, $v:expr) => {
        let feats: Vec<$out> = $f;
        assert_eq!(feats, $v);
    };
}
/*
#[macro_export]
macro_rules! featurizers {
    () => {
        (EmptyFtzr::new())
    };
    ($a:expr $(, $tail:expr)*) => {{
        MultiFtzr($a, featurizers!($($tail), *),
    )
    }};
} */

#[macro_export]
macro_rules! featurizers {
    //() => {
    //    (EmptyFtzr::new())
    //};
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
    let bislice = slice_gram(2);
    let g_bigram = gap_gram(n_gram::<1>(), 0, n_gram::<1>());
    let g_g_bigram = gap_gram(g_bigram, 2, g_bigram);
    let g_s_bigram = gap_gram(bislice, 2, bislice);
    let sentence = "one fish two fish red rish blue fish";
    let ak_bigrams = &["ab", "bc", "cd", "de", "ef", "fg", "gh", "hi", "ij", "jk"];
    let ak_bigrams_feat64 = &[
        Feature64(7222436297203265833),
        Feature64(488219265294888090),
        Feature64(12200746307096061963),
        Feature64(4415645335814054341),
        Feature64(17790974485650316728),
        Feature64(11004783825300746331),
        Feature64(16845529860537917751),
        Feature64(8876065120756845939),
        Feature64(14716811155994017359),
        Feature64(14633802556225993672),
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
        Feature64(3351245001697020842),
        Feature64(3476130915693767771),
        Feature64(11967000998307149924),
        Feature64(13702715391897792713),
        Feature64(17297361034655441419),
        Feature64(10615117631188256330),
        Feature64(12402805653220091773),
        Feature64(1615530608106378053),
        Feature64(13391796650981597793),
        Feature64(13464708936537152068),
        Feature64(16031824384145339114),
    ];
    let ak: &str = "abcdefghijk";
    let n_usize_12: &[usize; 12] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    ////////////////////////////
    //         &str
    ////////////////////////////
    {
        //        NGram<2>
        test_vec_feats!(bigram.featurize(ak), String, ak_bigrams);
        test_vec_feats!(bigram.featurize(ak), Feature64, ak_bigrams_feat64);

        //        SliceGram
        test_vec_feats!(bislice.featurize(ak), &str, ak_bigrams); // NOTICE &str
        test_vec_feats!(bislice.featurize(ak), Feature64, ak_bigrams_feat64);

        //        GapGram
        let feats: Vec<(String, String)> = gap_gram(bigram, 2, bigram).featurize(ak);
        let feats: Vec<(&str, &str)> = gap_gram(bislice, 2, bislice).featurize(ak);
        let feats: Vec<(Feature64, Feature64)> = g_g_bigram.featurize(ak);
    }

    ////////////////////////////
    //         &String
    ////////////////////////////

    {
        let t: &String = &ak.to_owned();
        //        NGram<2>
        test_vec_feats!(bigram.featurize(ak), String, ak_bigrams);
        test_vec_feats!(bigram.featurize(ak), Feature64, ak_bigrams_feat64);

        //        SliceGram
        test_vec_feats!(bislice.featurize(ak), &str, ak_bigrams);
        test_vec_feats!(bislice.featurize(ak), Feature64, ak_bigrams_feat64);

        //        GapGram
        let feats: Vec<(String, String)> = gap_gram(bigram, 2, bigram).featurize(t);
        let feats: Vec<(&str, &str)> = gap_gram(bislice, 2, bislice).featurize(t);
        let feats: Vec<(Feature64, Feature64)> = g_g_bigram.featurize(t);
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
            Feature64,
            bigrams_12_usize_feats64
        );

        //////////////SliceGram
        test_vec_feats!(
            bislice.featurize(&n_usize_12[..]),
            &[usize],
            bigrams_12_usize
        ); //NOTICE &[usize]
        let feats: Vec<&[usize]> = bislice.featurize(&n_usize_12[..]); //NOTICE &[usize]
                                                                       // NOTICE ... no Vec<usize>
        test_vec_feats!(
            bislice.featurize(&n_usize_12[..]),
            Feature64,
            bigrams_12_usize_feats64
        );

        ////////// GapGram
        let feats: Vec<([usize; 2], [usize; 2])> =
            gap_gram(bigram, 2, bigram).featurize(&n_usize_12[..]);
        let feats: Vec<(&[usize], &[usize])> =
            gap_gram(bislice, 2, bislice).featurize(&n_usize_12[..]);
        let feats: Vec<(Feature64, Feature64)> = g_g_bigram.featurize(&n_usize_12[..]);
    }
    let feats: Vec<Result<Vec<usize>, Feature64>> =
        bookends((4, bigram), (bigram, 4)).featurize(n_usize_12);
    let feats: Vec<Token<Vec<usize>>> = //TODO is this ok????
        bookends((4, bigram), (bigram, 4)).featurize(n_usize_12);
    //let e: EmptyFtzr<usize> = EmptyFtzr::new();

    let feats: Vec<Token<String>> = featurizers!(bislice, n_gram::<3>()).featurize(ak);

    let feats: Vec<Token<Feature64>> = featurizers!(bislice, n_gram::<3>()).featurize(ak);
    //let feats: Vec<EitherGroup<_, _>> = MultiFtzr(bigram, n_gram::<3>()).featurize(ak);

    //TODO
    //let feats: Vec<Feature64> = gap_gram(bislice, 4, bigram).featurize(n_usize_12);
    let feats: Vec<Feature64> = gap_gram(bislice, 4, bigram).featurize(&n_usize_12[..]);
    use std::collections::LinkedList;
    //TODO Linked list & friends
    //let feats: LinkedList<(&str, String)> = gap_gram(bislice, 4, n_gram::<3>()).featurize(ak);
    let feats: HashMap<&str, u8> = bislice.featurize(ak);

    let feats: Vec<&str> = for_each(bislice).featurize(sentence.split_ascii_whitespace());

    let doc = r#"
    i went to the market
    to buy a fat pig
    home again home again
    jig itty jig
"#;
    let doc_iter = doc.lines().map(|line| line.split_ascii_whitespace());
    let feats: Vec<&str> = for_each(for_each(bislice)).featurize(doc_iter);

    println!("{:?}", feats);
}
