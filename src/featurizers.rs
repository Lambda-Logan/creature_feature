use crate::accumulators::Accumulates;
use crate::hastokens::HasTokens;
use std::cmp;

#[cfg(feature = "serde1")]
use serde::{Deserialize, Serialize};

pub trait Featurizer<TokenGroup: Clone>
where
    Self: Sized,
{
    fn use_tokens_from<Feature, Push, State>(&self, tokengroup: TokenGroup, push_feat: &mut Push)
    where
        Feature: Accumulates<TokenGroup, State = State>,
        Push: FnMut(Feature) -> (),
        State: Default,
    {
        self.use_tokens_at_idx(tokengroup, push_feat, 0)
    }

    fn use_tokens_at_idx<Feature, Push, State>(
        &self,
        tokengroup: TokenGroup,
        push_feat: &mut Push,
        idx: isize,
    ) where
        Feature: Accumulates<TokenGroup, State = State>,
        Push: FnMut(Feature) -> (),
        State: Default;

    fn featurize<Origin, FeatureStep, FeatureGroup>(&self, origin: Origin) -> FeatureGroup
    where
        Origin: HasTokens<TokenGroup = TokenGroup>,
        FeatureStep: Accumulates<TokenGroup>,
        FeatureGroup: Accumulates<FeatureStep>,
    {
        let mut group: FeatureGroup::State = Default::default();
        {
            let mut push_step = |step: FeatureStep| FeatureGroup::accum_token(&mut group, step);
            //self.run(origin.expose_tokens(), &mut push_step);
            origin.give_tokens_to(self, &mut push_step);
        }
        FeatureGroup::finish(group)
    }

    fn featurize_x2<Origin, FeatureStepA, FeatureGroupA, FeatureStepB, FeatureGroupB>(
        &self,
        origin: Origin,
    ) -> (FeatureGroupA, FeatureGroupB)
    where
        Origin: HasTokens<TokenGroup = TokenGroup>,
        FeatureStepA: Accumulates<TokenGroup>,
        FeatureGroupA: Accumulates<FeatureStepA>,

        FeatureStepB: Accumulates<TokenGroup>,
        FeatureGroupB: Accumulates<FeatureStepB>,
    {
        let mut group_a: FeatureGroupA::State = Default::default();
        let mut group_b: FeatureGroupB::State = Default::default();
        {
            let mut push_step = |steps: (FeatureStepA, FeatureStepB)| {
                FeatureGroupA::accum_token(&mut group_a, steps.0);
                FeatureGroupB::accum_token(&mut group_b, steps.1);
            };
            //self.run(origin.expose_tokens(), &mut push_step);
            origin.give_tokens_to(self, &mut push_step);
        }
        (
            FeatureGroupA::finish(group_a),
            FeatureGroupB::finish(group_b),
        )
    }

    fn featurize_x3<
        Origin,
        FeatureStepA,
        FeatureGroupA,
        FeatureStepB,
        FeatureGroupB,
        FeatureStepC,
        FeatureGroupC,
    >(
        &self,
        origin: Origin,
    ) -> (FeatureGroupA, FeatureGroupB, FeatureGroupC)
    where
        Origin: HasTokens<TokenGroup = TokenGroup>,
        FeatureStepA: Accumulates<TokenGroup>,
        FeatureGroupA: Accumulates<FeatureStepA>,

        FeatureStepB: Accumulates<TokenGroup>,
        FeatureGroupB: Accumulates<FeatureStepB>,

        FeatureStepC: Accumulates<TokenGroup>,
        FeatureGroupC: Accumulates<FeatureStepC>,
    {
        let mut group_a: FeatureGroupA::State = Default::default();
        let mut group_b: FeatureGroupB::State = Default::default();
        let mut group_c: FeatureGroupC::State = Default::default();
        {
            let mut push_step = |steps: (FeatureStepA, (FeatureStepB, FeatureStepC))| {
                FeatureGroupA::accum_token(&mut group_a, steps.0);
                FeatureGroupB::accum_token(&mut group_b, steps.1 .0);
                FeatureGroupC::accum_token(&mut group_c, steps.1 .1);
            };
            //self.run(origin.expose_tokens(), &mut push_step);
            origin.give_tokens_to(self, &mut push_step);
        }
        (
            FeatureGroupA::finish(group_a),
            FeatureGroupB::finish(group_b),
            FeatureGroupC::finish(group_c),
        )
    }
}

#[derive(Hash, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Debug)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct GramSchema {
    pub(crate) group_a: (usize, usize),
    pub(crate) gap: (usize, usize),
    pub(crate) group_b: (usize, usize),
}

impl<'a, T> Featurizer<&'a [T]> for GramSchema {
    #[inline]
    fn use_tokens_at_idx<Feat, Push, State>(
        &self,
        tokengroup: &'a [T],
        push_feat: &mut Push,
        idx: isize,
    ) where
        Feat: Accumulates<&'a [T], State = State>,
        Push: FnMut(Feat) -> (),
        State: Default,
    {
        let s = tokengroup;
        let min = self.group_a.0 + self.gap.0 + self.group_b.0;
        if s.len() < min {
            return ();
        };
        if self.gap.1 > 0 {
            let min_gap = cmp::max(1, self.gap.0);
            for x in 0..(s.len() - min + 1) {
                for grp_a_idx in (x + self.group_a.0)..(x + self.group_a.1 + 1) {
                    if grp_a_idx > s.len() {
                        break;
                    }
                    let group_a = &s[x..grp_a_idx];
                    for space_idx in (grp_a_idx + min_gap)..(grp_a_idx + self.gap.1 + 1) {
                        for grp_b_idx in
                            (space_idx + self.group_b.0)..(space_idx + self.group_b.1 + 1)
                        {
                            if grp_b_idx > s.len() {
                                break;
                            }

                            let group_b = &s[space_idx..grp_b_idx];
                            let mut eater: State = Default::default();

                            if group_a.len() != 0 {
                                Feat::accum_token_at_idx(&mut eater, group_a, -2 * idx);
                            };
                            if group_b.len() != 0 {
                                Feat::accum_token_at_idx(&mut eater, group_b, 2 * idx);
                            };
                            push_feat(Feat::finish(eater));
                        }
                    }
                }
            }
        }

        if self.gap.0 == 0 {
            let a = self.group_a.0 + self.group_b.0;
            let b = self.group_a.1 + self.group_b.1;
            for x in 0..(s.len() - a + 1) {
                for _y in a..(b + 1) {
                    let y = x + _y;

                    if y > s.len() {
                        break;
                    }
                    if x != y {
                        let mut eater: State = Default::default();
                        Feat::accum_token(&mut eater, &s[x..y]);
                        push_feat(Feat::finish(eater));
                    };
                }
            }
        }
    }
}

pub fn n_gram(n: usize) -> GramSchema {
    GramSchema {
        group_a: (0, 0),
        gap: (0, 0),
        group_b: (n, n),
    }
}

pub fn skipgram(a: usize, gap: (usize, usize), b: usize) -> GramSchema {
    GramSchema {
        group_a: (a, a),
        gap: gap,
        group_b: (b, b),
    }
}

#[derive(Hash, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Debug)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct BookEndsFtzr<T> {
    head: usize,
    toe: usize,
    ftzr: T,
}

pub fn book_ends<Ftzr>(head_toe: (usize, usize), ftzr: Ftzr) -> BookEndsFtzr<Ftzr> {
    BookEndsFtzr {
        head: head_toe.0,
        toe: head_toe.1,
        ftzr: ftzr,
    }
}

//TODO test this
impl<'a, T, Ftzr: Featurizer<&'a [T]>> Featurizer<&'a [T]> for BookEndsFtzr<Ftzr> {
    #[inline]
    fn use_tokens_at_idx<Feat, Push, State>(
        &self,
        tokengroup: &'a [T],
        push_feat: &mut Push,
        idx: isize,
    ) where
        Feat: Accumulates<&'a [T], State = State>,
        Push: FnMut(Feat) -> (),
        State: Default,
    {
        //let mut pf = |n: Feature| push_feat(Feature(BookEnds::Head(n.0).uniq()));
        {
            if tokengroup.len() >= self.head {
                //println!("head {:?}", &s[..self.head]);
                //Feat::accum_token_with_flag(&mut eater, &tokengroup[..self.head], 0);

                {
                    self.ftzr
                        .use_tokens_at_idx(&tokengroup[..self.head], push_feat, idx);
                }
                //self.ftzr.run(&s[..self.head], &mut pf);
            }
            //push_feat(Feat::produce_feature(eater));
        }

        //let mut pf = |n: Feature| push_feat(Feature(BookEnds::Toe(n.0).uniq()));
        {
            //let mut eater: State = Default::default();
            if tokengroup.len() >= self.toe {
                //println!("toe {:?}", &s[(s.len() - self.toe)..s.len()]);
                //self.ftzr.run(&s[(s.len() - self.toe)..s.len()], &mut pf);
                //Feat::accum_token_with_flag(&mut eater,&tokengroup[(tokengroup.len() - self.toe)..tokengroup.len()],1,);
                self.ftzr.use_tokens_at_idx(
                    &tokengroup[(tokengroup.len() - self.toe)..tokengroup.len()],
                    push_feat,
                    -1 * idx,
                );
            }
            //push_feat(Feat::produce_feature(eater));
        }
    }
}

#[derive(Hash, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Debug)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct EmptyFtzr;

impl<T: Clone> Featurizer<T> for EmptyFtzr {
    #[inline]
    fn use_tokens_at_idx<Feat, Push, State>(&self, tokengroup: T, push_feat: &mut Push, idx: isize)
    where
        Feat: Accumulates<T, State = State>,
        Push: FnMut(Feat) -> (),
        State: Default,
    {
    }
}

#[derive(Hash, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Debug)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct MultiFtzr<A, B> {
    pub a: A,
    pub b: B,
}

impl<T, A, B> Featurizer<T> for MultiFtzr<A, B>
where
    T: Clone,
    A: Featurizer<T>,
    B: Featurizer<T>,
{
    #[inline]
    fn use_tokens_at_idx<Feat, Push, State>(&self, tokengroup: T, push_feat: &mut Push, idx: isize)
    where
        Feat: Accumulates<T, State = State>,
        Push: FnMut(Feat) -> (),
        State: Default,
    {
        self.a.use_tokens_at_idx(tokengroup.clone(), push_feat, idx);
        self.b.use_tokens_at_idx(tokengroup, push_feat, idx);
    }
}
