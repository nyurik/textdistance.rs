use super::algorithm::{Algorithm, Result};
use super::levenshtein::Levenshtein;
use std::hash::Hash;

#[derive(Default)]
pub struct YujianBo {
    levenshtein: Levenshtein,
}

impl Algorithm<f64> for YujianBo {
    fn for_iter<C, E>(&self, s1: C, s2: C) -> Result<f64>
    where
        C: Iterator<Item = E>,
        E: Eq + Copy + Hash,
    {
        let lev = self.levenshtein.for_iter(s1, s2);
        let dc: usize = self.levenshtein.del_cost;
        let ic: usize = self.levenshtein.ins_cost;
        let res = if lev.abs == 0 {
            0.0
        } else {
            (2 * lev.abs) as f64 / (lev.len1 * dc + lev.len2 * ic + lev.abs) as f64
        };
        Result {
            abs: res,
            is_distance: true,
            max: 1.0,
            len1: lev.len1,
            len2: lev.len2,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::textdistance::str::yujian_bo;
    use assert2::assert;
    use rstest::rstest;

    fn is_close(a: f64, b: f64) -> bool {
        (a - b).abs() < 1E-5
    }

    #[rstest]
    #[case("", "", 0.0)]
    // parity with abydos
    #[case("a", "", 1.0)]
    #[case("", "a", 1.0)]
    #[case("bc", "", 1.0)]
    #[case("", "bc", 1.0)]
    #[case("bc", "bc", 0.0)]
    #[case("bcd", "fgh", 0.6666666666666666)]
    #[case("ATCG", "TAGC", 0.5454545454545454)]
    #[case("cat", "hat", 0.285714285714)]
    #[case("aluminum", "Catalan", 0.6363636363636364)]
    fn function_str(#[case] s1: &str, #[case] s2: &str, #[case] exp: f64) {
        assert!(is_close(yujian_bo(s1, s2), exp));
    }
}