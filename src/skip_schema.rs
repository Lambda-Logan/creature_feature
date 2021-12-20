use crate::accum_ftzr::Ftzr;
use crate::gap_gram::GapPair;
use crate::multiftzr::EitherGroup;
use std::cmp;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Hash, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub(crate) struct SkipSchema {
    pub(crate) group_a: (usize, usize),
    pub(crate) gap: (usize, usize),
    pub(crate) group_b: (usize, usize),
}

impl<'a, T> Ftzr<&'a [T]> for SkipSchema {
    type TokenGroup = EitherGroup<&'a [T], GapPair<&'a [T], &'a [T]>>;
    #[inline]
    fn push_tokens<Push>(&self, origin: &'a [T], push: &mut Push)
    where
        Push: FnMut(Self::TokenGroup) -> (),
    {
        let s = origin;
        let min = self.group_a.0 + self.gap.0 + self.group_b.0;
        //println!("{:?}", &s[grp_a_1..grp_a_2]);
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
                    //if x != grp_a_idx {
                    //    println!("ga: {:?}", ((x, grp_a_idx), &s[x..grp_a_idx]));
                    //};
                    let group_a = &s[x..grp_a_idx];
                    for space_idx in (grp_a_idx + min_gap)..(grp_a_idx + self.gap.1 + 1) {
                        for grp_b_idx in
                            (space_idx + self.group_b.0)..(space_idx + self.group_b.1 + 1)
                        {
                            if grp_b_idx > s.len() {
                                break;
                            }

                            let group_b = &s[space_idx..grp_b_idx];
                            //let mut hasher: FxHasher64 = Default::default();

                            if group_a.len() != 0 && group_b.len() != 0 {
                                push(EitherGroup::Right(GapPair(
                                    group_a,
                                    group_b,
                                    (space_idx - grp_a_idx) as u16,
                                )))
                            }
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
                        push(EitherGroup::Left(&s[x..y]));
                    };
                }
            }
        }
    }
}
