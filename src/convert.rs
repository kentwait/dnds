use crate::dtype::{AminoAcid, Codon, Base};


pub fn str_to_bases(s: &str) -> Result<Vec<Base>, ()> {
    let mut base_vec: Vec<Base> = Vec::with_capacity(300);
    for c in s.chars() {
        let b = Base::from_char_(&c).unwrap();
        base_vec.push(b);
    }
    Ok(base_vec)
}

pub fn str_to_amino_acids(s: &str) -> Result<Vec<AminoAcid>, ()> {
    let mut aa_vec: Vec<AminoAcid> = Vec::with_capacity(300);
    for c in s.chars() {
        let aa = AminoAcid::from_one_char_(&c).unwrap();
        aa_vec.push(aa);
    }
    Ok(aa_vec)
}

pub fn str_to_codons(s: &str) -> Result<Vec<Codon>, ()> {
    let codon_chars: Vec<char> = s.chars().collect();
    if codon_chars.len() == 0 { return Ok(vec![]) }
    if codon_chars.len() % 3 != 0 { return Err(()) }
    let mut codon_vec: Vec<Codon> = Vec::with_capacity(100);
    for i in (0..s.len()-2).step_by(3) {
        let codon_string: String = codon_chars[i..i+3].iter().collect();
        let codon = Codon::from_str_(codon_string.as_str()).unwrap();
        codon_vec.push(codon);
    }
    Ok(codon_vec)
}


#[cfg(test)]
mod str_to_codon_vec_tests {
    use super::*;

    #[test]
    fn str_to_codon_vec_len_9() {
        let s: &str = "ATGCGCTTT";
        let expected: Vec<Codon> = vec![
            Codon::ATG,
            Codon::CGC,
            Codon::TTT,
        ];
        if let Ok(codons) = str_to_codons(s) {
            assert_eq!(codons, expected);
        }
    }

    #[test]
    fn str_to_codon_vec_len_8() {
        let s: &str = "ATGCGCTT";
        assert!(str_to_codons(s).is_err())
    }

    #[test]
    fn str_to_codon_vec_len_10() {
        let s: &str = "ATGCGCTTTG";
        assert!(str_to_codons(s).is_err())
    }

    #[test]
    fn str_to_codon_vec_len_0() {
        let s: &str = "";
        if let Ok(codons) = str_to_codons(s) {
            assert_eq!(codons, vec![]);
        }
    }
}