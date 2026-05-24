use crate::common::constants::LAMBDA;

pub type CutTuple = (usize, Vec<usize>, f64);

pub fn jaccard_similarity(s1: &[usize], s2: &[usize]) -> f64 {
    let mut i = 0;
    let mut j = 0;
    let mut intersection = 0.0;
    let mut union = 0.0;

    while i < s1.len() && j < s2.len() {
        if s1[i] == s2[j] {
            intersection += 1.0;
            union += 1.0;
            i += 1;
            j += 1;
        } else if s1[i] < s2[j] {
            union += 1.0;
            i += 1;
        } else {
            union += 1.0;
            j += 1;
        }
    }
    union += (s1.len() - i) as f64;
    union += (s2.len() - j) as f64;

    if union == 0.0 {
        0.0
    } else {
        intersection / union
    }
}

pub fn filter_orthogonal_cuts(
    mut all_cuts: Vec<CutTuple>
) -> Vec<CutTuple> {
    all_cuts.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());

    let mut w_cuts: Vec<CutTuple> = Vec::new();
    w_cuts.push(all_cuts.remove(0));

    loop {
        let mut best_candidate_idx = usize::MAX;
        let mut best_candidate_violation = -1.0;

        for (idx, cut) in all_cuts.iter().enumerate() {
            let mut is_orthogonal = true;
            for w_cut in &w_cuts {
                if jaccard_similarity(&cut.1, &w_cut.1) > LAMBDA {
                    is_orthogonal = false;
                    break;
                }
            }

            if is_orthogonal && cut.2 > best_candidate_violation {
                best_candidate_violation = cut.2;
                best_candidate_idx = idx;
            }
        }

        if best_candidate_idx == usize::MAX {
            break;
        }

        w_cuts.push(all_cuts.remove(best_candidate_idx));
    }

    w_cuts
}