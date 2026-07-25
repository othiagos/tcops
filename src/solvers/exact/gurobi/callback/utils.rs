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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jaccard_similarity_empty() {
        assert_eq!(jaccard_similarity(&[], &[]), 0.0);
    }

    #[test]
    fn test_jaccard_similarity_identical() {
        let s1 = vec![1, 2, 3];
        let s2 = vec![1, 2, 3];
        assert_eq!(jaccard_similarity(&s1, &s2), 1.0);
    }

    #[test]
    fn test_jaccard_similarity_disjoint() {
        let s1 = vec![1, 2];
        let s2 = vec![3, 4];
        assert_eq!(jaccard_similarity(&s1, &s2), 0.0);
    }

    #[test]
    fn test_jaccard_similarity_partial() {
        let s1 = vec![1, 2, 3];
        let s2 = vec![2, 3, 4];
        // intersection = 2 ({2,3}), union = 4 ({1,2,3,4}), similarity = 2/4 = 0.5
        assert_eq!(jaccard_similarity(&s1, &s2), 0.5);
    }

    #[test]
    fn test_filter_orthogonal_cuts_empty() {
        let cuts = filter_orthogonal_cuts(vec![]);
        assert!(cuts.is_empty());
    }

    #[test]
    fn test_filter_orthogonal_cuts_sorting_and_filtering() {
        // CutTuple: (vehicle_id, tour, violation, src)
        let cut1: CutTuple = (0, vec![1, 2], 0.8, 1);
        let cut2: CutTuple = (0, vec![1, 2], 0.9, 1); // Identical tour, higher violation -> should be kept, cut1 dropped
        let cut3: CutTuple = (0, vec![5, 6], 0.7, 5); // Disjoint tour -> should be kept

        let cuts = filter_orthogonal_cuts(vec![cut1, cut2, cut3]);

        assert_eq!(cuts.len(), 2);
        assert_eq!(cuts[0].2, 0.9); // Highest violation first
        assert_eq!(cuts[1].2, 0.7);
    }
}
