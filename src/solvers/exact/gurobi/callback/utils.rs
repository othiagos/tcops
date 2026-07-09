use crate::common::constants::LAMBDA;

pub type CutTuple = (usize, Vec<usize>, f64, usize);

pub fn jaccard_similarity(s1: &[usize], s2: &[usize]) -> f64 {
    let mut i = 0;
    let mut j = 0;
    let mut intersection = 0.0;

    while i < s1.len() && j < s2.len() {
        if s1[i] == s2[j] {
            intersection += 1.0;
            i += 1;
            j += 1;
        } else if s1[i] < s2[j] {
            i += 1;
        } else {
            j += 1;
        }
    }

    let total_elements = (s1.len() + s2.len()) as f64;
    let union = total_elements - intersection;

    if union == 0.0 {
        0.0
    } else {
        intersection / union
    }
}

pub fn filter_orthogonal_cuts(mut all_cuts: Vec<CutTuple>) -> Vec<CutTuple> {
    if all_cuts.is_empty() {
        return vec![];
    }

    all_cuts.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));

    let mut w_cuts: Vec<CutTuple> = Vec::new();

    for cut in all_cuts {
        let is_orthogonal = w_cuts
            .iter()
            .all(|w_cut| jaccard_similarity(&cut.1, &w_cut.1) <= LAMBDA);

        if is_orthogonal {
            w_cuts.push(cut);
        }
    }

    w_cuts
}
