use crate::accum_ftzr::*;
use crate::hashedfeature::*;
use crate::n_gram::*;
use crate::n_slice::*;
use crate::tokengroup::chars_of;
use senor_borroso;
use senor_borroso::hasfeatures::HasFeatures;
use std::time::Instant;
pub(crate) fn bench() {
    let az = "abcdefghijklmnopqrstuvuxyz1$n34567890!@#$%^&*()";
    let mut bigstring = "".to_owned();
    for _ in (0..100) {
        bigstring.push_str(&az);
    }
    let bigstring = bigstring.as_str();

    fn bench_single<Data: Clone, F: Fn(Data) -> usize>(n: usize, msg: &str, data: Data, f: F) {
        let epochs = 10000;
        let mut k = 0;
        let pre = Instant::now();
        for _ in (0..epochs) {
            k += f(data.clone());
        }
        let post = pre.elapsed().as_micros();
        println!("\n\n");
        println!("n = {} @ {}", n, msg);
        println!("    Result: {}", k / epochs);
        println!(
            "    Microseconds to featurize 10k grams: {}μs",
            post / ((k / 10000) as u128)
        );
    }
    let sb_ftzr = senor_borroso::ftzrs::n_gram;
    macro_rules! bench_n {
        ($n:expr) => {{
            bench_single($n, &"n_slice + Vec<&str> (from &str)", bigstring, |s| {
                let v: Vec<&str> = n_slice($n).featurize(s);
                v.len()
            });

            /*
            bench_single($n, &"n_gram + Vec<String> (from &str)", bigstring, |s| {
                let v: Vec<String> = n_gram::<$n>().featurize(s);
                v.len()
            });*/

            { /*
                     use ngrammatic::NgramBuilder;
                     bench_single(
                         $n,
                         &"OTHER CRATE C + HashMap<String, usize> (from Iterator of <u8>)",
                         &bigstring,
                         |s| {
                             //let v: Vec<Vec<u8>> = s.bytes().ngrams($n).collect();
                             let bag = NgramBuilder::new(s).arity($n).finish().grams;
                             let mut n = 0;
                             for (k, v) in bag.iter() {
                                 n += v;
                             }
                             n
                         },
                     );
                 */
            }
            bench_single(
                $n,
                &"n_slice + Vec<HashedAs<u64>> (from &str)",
                bigstring,
                |s| {
                    let v: Vec<HashedAs<u64>> = n_slice($n).featurize(s);
                    v.len()
                },
            );

            bench_single(
                $n,
                &"senor_borroso + Vec<HashedAs<u64>> (from &str)",
                bigstring,
                |s| {
                    let v: Vec<senor_borroso::ftzrs::Feature> =
                        s.collect_features_with(&sb_ftzr($n));
                    v.len()
                },
            );

            bench_single(
                $n,
                &"n_gram + Vec<HashedAs<u64>> (from &str)",
                bigstring,
                |s| {
                    let v: Vec<HashedAs<u64>> = n_gram::<$n>().featurize(s);
                    v.len()
                },
            );

            bench_single(
                $n,
                &"n_slice + Vec<HashedAs<u16>> (from &str)",
                bigstring,
                |s| {
                    let v: Vec<HashedAs<u16>> = n_slice($n).featurize(s);
                    v.len()
                },
            );

            bench_single(
                $n,
                &"n_gram + Vec<HashedAs<u16>> (from &str)",
                bigstring,
                |s| {
                    let v: Vec<HashedAs<u16>> = n_gram::<$n>().featurize(s);
                    v.len()
                },
            );

            bench_single($n, &"n_slice + Vec<&[u8]> (from &str)", bigstring, |s| {
                let v: Vec<&[u8]> = n_slice($n).featurize(s);
                v.len()
            });
            bench_single($n, &"n_gram + Vec<[u8;N]> (from &str)", bigstring, |s| {
                let v: Vec<[u8; $n]> = n_gram::<$n>().featurize(s);
                v.len()
            });

            /*
            {
                use ngram::NGram;
                bench_single(
                    $n,
                    &"OTHER CRATE B + Vec<Vec<u8>> (from Iterator of <u8>)",
                    &bigstring,
                    |s| {
                        let v: Vec<Vec<u8>> = s.bytes().ngrams($n).collect();
                        v.len()
                    },
                );
            }*/

            let chars = chars_of(bigstring);
            /*
            {
                use ngrams::Ngram;
                bench_single(
                    $n,
                    &"OTHER CRATE A + Vec<Vec<char>> (from Iterator of <char>)",
                    &bigstring,
                    |s| {
                        let v: Vec<Vec<char>> = s.chars().ngrams($n).collect();
                        v.len()
                    },
                );
            }
            {
                use ngram::NGram;
                bench_single(
                    $n,
                    &"OTHER CRATE B + Vec<Vec<char>> (from Iterator of <char>)",
                    &bigstring,
                    |s| {
                        let v: Vec<Vec<char>> = s.chars().ngrams($n).collect();
                        v.len()
                    },
                );
            }*/

            bench_single(
                $n,
                &"n_gram + Vec<[char;N]> (from Vec<char>)",
                &chars,
                |s| {
                    let v: Vec<[char; $n]> = n_gram::<$n>().featurize(s);
                    v.len()
                },
            );

            bench_single(
                $n,
                &"n_slice + Vec<&[char]> (from Vec<char>)",
                &chars,
                |s| {
                    let v: Vec<&[char]> = n_slice($n).featurize(s);
                    v.len()
                },
            );

            bench_single(
                $n,
                &"n_gram + Vec<HashedAs<u64>> (from Vec<char>)",
                &chars,
                |s| {
                    let v: Vec<HashedAs<u64>> = n_gram::<$n>().featurize(s);
                    v.len()
                },
            );

            bench_single(
                $n,
                &"n_slice + Vec<HashedAs<u64>> (from Vec<char>)",
                &chars,
                |s| {
                    let v: Vec<HashedAs<u64>> = n_slice($n).featurize(s);
                    v.len()
                },
            );

            bench_single(
                $n,
                &"senor_borroso + Vec<HashedAs<u64>> (from &[char])",
                &chars,
                |s| {
                    let v: Vec<senor_borroso::ftzrs::Feature> =
                        s.collect_features_with(&sb_ftzr($n));
                    v.len()
                },
            );
            bench_single(
                $n,
                &"senor_borroso (bookends) + Vec<HashedAs<u64>> (from &[char])",
                &chars,
                |s| {
                    let v: Vec<senor_borroso::ftzrs::Feature> =
                        s.collect_features_with(&sb_ftzr($n));
                    v.len()
                },
            );
        }};
    }
    bench_n!(2);
    bench_n!(4);
    bench_n!(8);
    //bench_n!(16);
    bench_n!(32);
    //bench_n!(64);
    bench_n!(128);
}
