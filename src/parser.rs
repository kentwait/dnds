use crate::dtype::{
    SequenceItem, PwAlnItem,
    Base, AminoAcid, Codon, 
};

// String alignment functions

pub fn aln_str_to_bases(s: &str) -> Result<Vec<SequenceItem<Base, ()>>, ()> {
    let vec = s.chars()
        .map(|c| Base::from_char_(&c) )
        .collect();
    Ok(vec)
}

pub fn aln_str_to_amino_acids(s: &str) -> Result<Vec<SequenceItem<AminoAcid, ()>>, ()> {
    let vec = s.chars()
        .map(|c| AminoAcid::from_one_char_(&c) )
        .collect();
    Ok(vec)
}

pub fn aln_str_to_codons(s: &str) -> Result<Vec<SequenceItem<Codon, ()>>, ()> {
    let c_vec: Vec<char> = s.chars().collect();
    if c_vec.len() % 3 != 0 { return Err(()) }
    let vec = c_vec.chunks_exact(3)
        .map(|c_array| c_array.iter().collect::<String>() )
        .map(|s| Codon::from_str_(&s) )
        .collect();
    Ok(vec)
}


// Pairwise alignment functions

pub fn pairwise_aln_vec_to_paired<T, E>(vec1: Vec<SequenceItem<T, E>>, vec2: Vec<SequenceItem<T, E>>) -> Result<Vec<PwAlnItem<T, E>>, ()> {
    let pw_vec = vec1.into_iter().zip(vec2.into_iter())
        .enumerate()
        .map(|(i, (item1, item2))| {
            PwAlnItem(item1, item2, i+1)
        }).collect();
    Ok(pw_vec)
}

pub fn pairwise_aln_str_to_paired_bases(s1: &str, s2: &str) -> Result<Vec<PwAlnItem<Base, ()>>, ()> {
    pairwise_aln_vec_to_paired(aln_str_to_bases(s1)?, aln_str_to_bases(s2)?)
}

pub fn pairwise_aln_str_to_paired_amino_acids(s1: &str, s2: &str) -> Result<Vec<PwAlnItem<AminoAcid, ()>>, ()> {
    pairwise_aln_vec_to_paired(aln_str_to_amino_acids(s1)?, aln_str_to_amino_acids(s2)?)
}

pub fn pairwise_aln_str_to_paired_codons(s1: &str, s2: &str) -> Result<Vec<PwAlnItem<Codon, ()>>, ()> {
    pairwise_aln_vec_to_paired(aln_str_to_codons(s1)?, aln_str_to_codons(s2)?)
}


// Filtering pairwise functions
pub fn keep_valid_sites<T, E>(pw_vec: Vec<PwAlnItem<T, E>>) -> Vec<PwAlnItem<T, E>> {
    pw_vec.into_iter()
        .filter(|p| p.both_valid())
        .collect()
}

#[cfg(test)]
mod aln_str_to_codon_vec_tests {
    use super::*;

    #[test]
    fn aln_str_to_codons_len_9() {
        let s: &str = "ATGCGCTTT";
        let expected: Vec<SequenceItem<Codon, ()>> = vec![
            SequenceItem::Some(Codon::ATG),
            SequenceItem::Some(Codon::CGC),
            SequenceItem::Some(Codon::TTT),
        ];
        if let Ok(codons) = aln_str_to_codons(s) {
            assert_eq!(codons, expected);
        }
    }

    #[test]
    fn aln_str_to_codons_len_8() {
        let s: &str = "ATGCGCTT";
        assert!(aln_str_to_codons(s).is_err())
    }

    #[test]
    fn aln_str_to_codons_len_10() {
        let s: &str = "ATGCGCTTTG";
        assert!(aln_str_to_codons(s).is_err())
    }

    #[test]
    fn aln_str_to_codons_len_0() {
        let s: &str = "";
        if let Ok(codons) = aln_str_to_codons(s) {
            assert_eq!(codons, vec![]);
        }
    }

}


#[cfg(test)]
mod pairwise_aln_str_to_paired_tests {
    use super::*;

    #[test]
    fn pairwise_aln_str_to_paired_bases_ok() {
        let s1 = "ATG-A";
        let s2 = "AT-NA";
        let result = pairwise_aln_str_to_paired_bases(s1, s2).unwrap();
        let expected = vec![
            PwAlnItem(
                SequenceItem::Some(Base::A), 
                SequenceItem::Some(Base::A), 
                1
            ),
            PwAlnItem(
                SequenceItem::Some(Base::T), 
                SequenceItem::Some(Base::T), 
                2
            ),
            PwAlnItem(
                SequenceItem::Some(Base::G), 
                SequenceItem::Gap, 
                3
            ),
            PwAlnItem(
                SequenceItem::Gap, 
                SequenceItem::Unknown, 
                4
            ),
            PwAlnItem(
                SequenceItem::Some(Base::A), 
                SequenceItem::Some(Base::A), 
                5
            ),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn pairwise_aln_str_to_paired_amino_acids_ok() {
        let s1 = "M-GY*";
        let s2 = "MGGY*";
        let result = pairwise_aln_str_to_paired_amino_acids(s1, s2).unwrap();
        let expected = vec![
            PwAlnItem(
                SequenceItem::Some(AminoAcid::Met), 
                SequenceItem::Some(AminoAcid::Met), 
                1
            ),
            PwAlnItem(
                SequenceItem::Gap, 
                SequenceItem::Some(AminoAcid::Gly), 
                2
            ),
            PwAlnItem(
                SequenceItem::Some(AminoAcid::Gly), 
                SequenceItem::Some(AminoAcid::Gly), 
                3
            ),
            PwAlnItem(
                SequenceItem::Some(AminoAcid::Tyr), 
                SequenceItem::Some(AminoAcid::Tyr), 
                4
            ),
            PwAlnItem(
                SequenceItem::Some(AminoAcid::Stop), 
                SequenceItem::Some(AminoAcid::Stop), 
                5
            ),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn pairwise_aln_str_to_paired_codons_ok() {
        let s1 = "ATGATATTTTGA";
        let s2 = "ATG---TCTTGA";
        let result = pairwise_aln_str_to_paired_codons(s1, s2).unwrap();
        let expected = vec![
            PwAlnItem(
                SequenceItem::Some(Codon::ATG), 
                SequenceItem::Some(Codon::ATG), 
                1
            ),
            PwAlnItem(
                SequenceItem::Some(Codon::ATA), 
                SequenceItem::Gap, 
                2
            ),
            PwAlnItem(
                SequenceItem::Some(Codon::TTT), 
                SequenceItem::Some(Codon::TCT), 
                3
            ),
            PwAlnItem(
                SequenceItem::Some(Codon::TGA), 
                SequenceItem::Some(Codon::TGA), 
                4
            ),
        ];
        assert_eq!(result, expected);
    }

}


#[cfg(test)]
mod keep_valid_sites_tests {
    use super::*;

    #[test]
    fn keep_valid_sites_amino_acid_ok() {
        let vec: Vec<PwAlnItem<AminoAcid, ()>> = vec![
            PwAlnItem(
                SequenceItem::Some(AminoAcid::Met), 
                SequenceItem::Some(AminoAcid::Met), 
                1
            ),
            PwAlnItem(
                SequenceItem::Gap, 
                SequenceItem::Some(AminoAcid::Gly), 
                2
            ),
            PwAlnItem(
                SequenceItem::Some(AminoAcid::Gly), 
                SequenceItem::Some(AminoAcid::Gly), 
                3
            ),
            PwAlnItem(
                SequenceItem::Some(AminoAcid::Tyr), 
                SequenceItem::Some(AminoAcid::Tyr), 
                4
            ),
            PwAlnItem(
                SequenceItem::Some(AminoAcid::Stop), 
                SequenceItem::Some(AminoAcid::Stop), 
                5
            ),
        ];
        let result = keep_valid_sites(vec);
        let expected: Vec<PwAlnItem<AminoAcid, ()>> = vec![
            PwAlnItem(
                SequenceItem::Some(AminoAcid::Met), 
                SequenceItem::Some(AminoAcid::Met), 
                1
            ),
            PwAlnItem(
                SequenceItem::Some(AminoAcid::Gly), 
                SequenceItem::Some(AminoAcid::Gly), 
                3
            ),
            PwAlnItem(
                SequenceItem::Some(AminoAcid::Tyr), 
                SequenceItem::Some(AminoAcid::Tyr), 
                4
            ),
            PwAlnItem(
                SequenceItem::Some(AminoAcid::Stop), 
                SequenceItem::Some(AminoAcid::Stop), 
                5
            ),
        ];
        // is stop amino acid "valid"??
        assert_eq!(result, expected);
    }

    #[test]
    fn keep_valid_sites_codon_ok() {
        let vec: Vec<PwAlnItem<Codon, ()>> = vec![
            PwAlnItem(
                SequenceItem::Some(Codon::ATG), 
                SequenceItem::Some(Codon::ATG), 
                1
            ),
            PwAlnItem(
                SequenceItem::Some(Codon::ATA), 
                SequenceItem::Gap, 
                2
            ),
            PwAlnItem(
                SequenceItem::Some(Codon::TTT), 
                SequenceItem::Some(Codon::TCT), 
                3
            ),
            PwAlnItem(
                SequenceItem::Unknown, 
                SequenceItem::Some(Codon::TGA), 
                4
            ),
            PwAlnItem(
                SequenceItem::Some(Codon::TGA), 
                SequenceItem::Some(Codon::TGA), 
                5
            ),
        ];
        let result = keep_valid_sites(vec);
        let expected = vec![
            PwAlnItem(
                SequenceItem::Some(Codon::ATG), 
                SequenceItem::Some(Codon::ATG), 
                1
            ),
            PwAlnItem(
                SequenceItem::Some(Codon::TTT), 
                SequenceItem::Some(Codon::TCT), 
                3
            ),
            PwAlnItem(
                SequenceItem::Some(Codon::TGA), 
                SequenceItem::Some(Codon::TGA), 
                5
            ),
        ];
        // is stop amino acid "valid"??
        assert_eq!(result, expected);
    }

}
