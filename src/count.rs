use crate::dtype::{
    Codon, 
    SequenceItem,
    WrappedSequenceItem,
    PwAlnItem,
};
use crate::parser::keep_valid_sites;


fn count_sites_single_codon(codon: &Codon) -> (f64, f64) {
    let mut syn_site_count: f64 = 0.;
    let mut nonsyn_site_count: f64 = 0.;
    codon.one_hit_mutants_().iter()
        .filter(|c| !c.is_stop_codon_() )
        .for_each(|other| {
            if codon.is_synonymous_change_(other) {
                syn_site_count += 1.;
            } else {
                nonsyn_site_count += 1.;
            }

        });
    // Sum counts and compute synonymous and non-synonymous site counts
    let sum_count: f64 = syn_site_count + nonsyn_site_count;
    let s: f64 = 3. * (syn_site_count / sum_count);
    let n: f64 = 3. * (nonsyn_site_count / sum_count);
    (s, n)
}

pub fn count_sites(codon1: &Codon, codon2: &Codon) -> Result<(f64, f64), ()> {
    if codon1.is_stop_codon_() || codon2.is_stop_codon_() {
        return Err(())
    }
    let (s_1, n_1) = count_sites_single_codon(codon1);
    let (s_2, n_2) = count_sites_single_codon(codon2);
    // Average ove the pair of codons
    let average_s = (s_1 + s_2) / 2.;
    let average_n = (n_1 + n_2) / 2.;
    Ok((average_s, average_n))
}

pub fn count_differences(codon1: &Codon, codon2: &Codon) -> Result<(f64, f64), ()> {
    if codon1 == codon2 { return Ok((0., 0.)) }
    let mut syn_diff = 0;
    let mut nonsyn_diff = 0;
    let mut path_count = 0;
    codon1.list_mutation_pathways_(codon2).into_iter()
        .for_each(|path| {
            path.iter().zip(path.iter().skip(1))
                .for_each(|(codon1, codon2)| {
                    if codon1.is_synonymous_change_(codon2) {
                        syn_diff += 1;
                    } else {
                        nonsyn_diff += 1;
                    }
                });
            path_count += 1;
        });
    let average_syn_diff = (syn_diff as f64) / (path_count as f64);
    let average_nonsyn_diff = (nonsyn_diff as f64) / (path_count as f64);
    Ok((average_syn_diff, average_nonsyn_diff))
}

pub fn count_total_sites<E>(vec: Vec<PwAlnItem<Codon, E>>) -> Result<(f64, f64), ()> {
    // Remove gaps, unknowns, errors
    let mut total_s = 0.;
    let mut total_n = 0.;
    for pair in keep_valid_sites(vec).into_iter() {
        let codon1 = pair.0.unwrap()?;
        let codon2 = pair.1.unwrap()?;
        let (s, n) = count_sites(&codon1, &codon2)?;
        total_s += s;
        total_n += n;
    }
    Ok((total_s, total_n))
}

pub fn count_total_differences<E>(vec: Vec<PwAlnItem<Codon, E>>) -> Result<(f64, f64), ()> {
    // Remove gaps, unknowns, errors
    let mut total_sd = 0.;
    let mut total_nd = 0.;
    for pair in keep_valid_sites(vec).into_iter() {
        let codon1 = pair.0.unwrap()?;
        let codon2 = pair.1.unwrap()?;
        let (sd, nd) = count_differences(&codon1, &codon2)?;
        total_sd += sd;
        total_nd += nd;
    }
    Ok((total_sd, total_nd))
}


#[cfg(test)]
mod count_sites_tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn count_sites_single_codon_phe() {
        let (s, n) = count_sites_single_codon(&Codon::TTT);  // Phe
        assert_approx_eq!(s, 0.3333334);
        assert_approx_eq!(n, 2.6666667);
        assert_approx_eq!(s+n, 3.0);
    }

    #[test]
    fn count_s_n_sites_single_codon_no_syn_change() {
        let (s, n) = count_sites_single_codon(&Codon::ATG);  // Met
        assert_approx_eq!(s, 0.);
        assert_approx_eq!(n, 3.);
        assert_approx_eq!(s+n, 3.0);
    }

    #[test]
    fn count_s_n_sites_single_codon_fourfold() {
        let (s, n) = count_sites_single_codon(&Codon::GGG);  // Leu
        assert_approx_eq!(s, 1.0);
        assert_approx_eq!(n, 2.0);
        assert_approx_eq!(s+n, 3.0);
    }

    #[test]
    fn count_s_n_sites_single_codon_sixfold() {
        let (s, n) = count_sites_single_codon(&Codon::CTA);  // Gly
        assert_approx_eq!(s, 1.3333334);
        assert_approx_eq!(n, 1.6666667);
        assert_approx_eq!(s+n, 3.0);
    }

    #[test]
    fn count_sites_single_codon_leu() {
        let (s, n) = count_sites_single_codon(&Codon::TTG);  // Leu
        assert_approx_eq!(s, 0.75);
        assert_approx_eq!(n, 2.25);
        assert_approx_eq!(s+n, 3.0);
    }

    #[test]
    fn count_sites_same() {
        if let Ok((s, n)) = count_sites(&Codon::ATG, &Codon::ATG) { // Met
            assert_approx_eq!(s, 0.);
            assert_approx_eq!(n, 3.);
            assert_approx_eq!(s+n, 3.0);
        }
    }

    #[test]
    fn count_sites_phe_leu() {
        let codon1 = Codon::TTT;
        let codon2 = Codon::TTG;
        let (s, n) = count_sites(&codon1, &codon2).unwrap();  // Phe, Leu
        assert_approx_eq!(s, 0.541666);
        assert_approx_eq!(n, 2.458333);
        assert_approx_eq!(s+n, 3.0);
    }

}


#[cfg(test)]
mod count_differences_tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn count_differences_same() {
        let (codon1, codon2) = (Codon::AAA, Codon::AAA);
        let (s, n) = count_differences(&codon1, &codon2).unwrap();
        assert_approx_eq!(s, 0.);
        assert_approx_eq!(n, 0.);
        assert_approx_eq!(s+n, 0.);
    }

    #[test]
    fn count_differences_one_path() {
        let (codon1, codon2) = (Codon::CCT, Codon::CAT);
        let (s, n) = count_differences(&codon1, &codon2).unwrap();
        assert_approx_eq!(s, 0.);
        assert_approx_eq!(n, 1.);
        assert_approx_eq!(s+n, 1.);
    }

    #[test]
    fn count_differences_two_paths() {
        let (codon1, codon2) = (Codon::CCT, Codon::CAG);
        let (s, n) = count_differences(&codon1, &codon2).unwrap();
        assert_approx_eq!(s, 0.5);
        assert_approx_eq!(n, 1.5);
        assert_approx_eq!(s+n, 2.);
    }

    #[test]
    fn count_differences_six_paths() {
        let (codon1, codon2) = (Codon::TTT, Codon::GGG);
        let (s, n) = count_differences(&codon1, &codon2).unwrap();
        assert_approx_eq!(s, 0.5);
        assert_approx_eq!(n, 2.5);
        assert_approx_eq!(s+n, 3.);
    }

}


#[cfg(test)]
mod count_total_sites_tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn count_total_sites_same() {
        let vec: Vec<PwAlnItem<Codon, ()>> = vec![
            PwAlnItem(
                SequenceItem::Some(Codon::ATG), 
                SequenceItem::Some(Codon::ATG), 
                1
            ),
            // PwAlnItem(
            //     SequenceItem::Some(Codon::ATA), 
            //     SequenceItem::Gap, 
            //     2
            // ),
            PwAlnItem(
                SequenceItem::Some(Codon::TTT), 
                SequenceItem::Some(Codon::TTT), 
                3
            ),
            // PwAlnItem(
            //     SequenceItem::Unknown, 
            //     SequenceItem::Some(Codon::TGA), 
            //     4
            // ),
            // PwAlnItem(
            //     SequenceItem::Some(Codon::TGA), 
            //     SequenceItem::Some(Codon::TGA), 
            //     5
            // ),
        ];
        let (s, n) = count_total_sites(vec).unwrap();
        assert_approx_eq!(s, 0.333333);
        assert_approx_eq!(n, 5.666666);
    }

    #[test]
    fn count_total_sites_different() {
        let vec: Vec<PwAlnItem<Codon, ()>> = vec![
            PwAlnItem(
                SequenceItem::Some(Codon::ATG), 
                SequenceItem::Some(Codon::GAC), 
                1
            ),
            // PwAlnItem(
            //     SequenceItem::Some(Codon::ATA), 
            //     SequenceItem::Gap, 
            //     2
            // ),
            PwAlnItem(
                SequenceItem::Some(Codon::TTT), 
                SequenceItem::Some(Codon::AAA),  // has stop codon mutant TAA
                3
            ),
            // PwAlnItem(
            //     SequenceItem::Unknown, 
            //     SequenceItem::Some(Codon::TGA), 
            //     4
            // ),
            // PwAlnItem(
            //     SequenceItem::Some(Codon::TGA), 
            //     SequenceItem::Some(Codon::TGA), 
            //     5
            // ),
        ];
        let (s, n) = count_total_sites(vec).unwrap();
        assert_approx_eq!(s, 0.520833);
        assert_approx_eq!(n, 5.479166);
    }

    #[test]
    fn count_total_sites_invalids() {
        let vec: Vec<PwAlnItem<Codon, ()>> = vec![
            PwAlnItem(
                SequenceItem::Some(Codon::ATG), 
                SequenceItem::Some(Codon::GAC), 
                1
            ),
            PwAlnItem(
                SequenceItem::Some(Codon::ATA), 
                SequenceItem::Gap, 
                2
            ),
            PwAlnItem(
                SequenceItem::Some(Codon::TTT), 
                SequenceItem::Some(Codon::AAA),  // has stop codon mutant TAA
                3
            ),
            // PwAlnItem(
            //     SequenceItem::Unknown, 
            //     SequenceItem::Some(Codon::TGA), 
            //     4
            // ),
            // PwAlnItem(
            //     SequenceItem::Some(Codon::TGA), 
            //     SequenceItem::Some(Codon::TGA), 
            //     5
            // ),
        ];
        let (s, n) = count_total_sites(vec).unwrap();
        assert_approx_eq!(s, 0.520833);
        assert_approx_eq!(n, 5.479166);
    }
}


#[cfg(test)]
mod count_total_differences_tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn count_total_differences_same() {
        let vec: Vec<PwAlnItem<Codon, ()>> = vec![
            PwAlnItem(
                SequenceItem::Some(Codon::ATG), 
                SequenceItem::Some(Codon::ATG), 
                1
            ),
            // PwAlnItem(
            //     SequenceItem::Some(Codon::ATA), 
            //     SequenceItem::Gap, 
            //     2
            // ),
            PwAlnItem(
                SequenceItem::Some(Codon::TTT), 
                SequenceItem::Some(Codon::TTT), 
                3
            ),
            // PwAlnItem(
            //     SequenceItem::Unknown, 
            //     SequenceItem::Some(Codon::TGA), 
            //     4
            // ),
            // PwAlnItem(
            //     SequenceItem::Some(Codon::TGA), 
            //     SequenceItem::Some(Codon::TGA), 
            //     5
            // ),
        ];
        let (s, n) = count_total_differences(vec).unwrap();
        assert_approx_eq!(s, 0.);
        assert_approx_eq!(n, 0.);
        assert_approx_eq!(s+n, 0.);
    }
    
}
