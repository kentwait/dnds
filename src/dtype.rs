use pyo3::prelude::*;
use enum_iterator::{all, Sequence};


#[derive(Debug, Copy, Clone, PartialEq, )]
pub enum SequenceItem<I, E> {
    Some(I),
    Gap,
    Unknown,
    Err(E),
}

pub trait WrappedSequenceItem {
    type Output;

    fn unwrap(self) -> Result<Self::Output, ()>;
    fn to_str(&self) -> String;
}

impl<E> WrappedSequenceItem for SequenceItem<Base, E> {
    type Output = Base;

    fn unwrap(self) -> Result<Self::Output, ()> {
        match self {
            Self::Some(i) => Ok(i),
            Self::Gap => Err(()), 
            Self::Unknown => Err(()), 
            Self::Err(e) => Err(()), 
        }
    }

    fn to_str(&self) -> String {
        match self {
            Self::Some(i) => i.to_char_().to_string(),
            Self::Gap => "-".to_string(),
            Self::Unknown => "N".to_string(),
            Self::Err(_) => "!".to_string(),
        }
    }
}

impl<E> WrappedSequenceItem for SequenceItem<AminoAcid, E> {
    type Output = AminoAcid;

    fn unwrap(self) -> Result<Self::Output, ()> {
        match self {
            Self::Some(i) => Ok(i),
            Self::Gap => Err(()), 
            Self::Unknown => Err(()), 
            Self::Err(e) => Err(()), 
        }
    }

    fn to_str(&self) -> String {
        match self {
            Self::Some(i) => i.to_one_char_().to_string(),
            Self::Gap => "-".to_string(),
            Self::Unknown => "X".to_string(),
            Self::Err(_) => "!".to_string(),
        }
    }
}

impl<E> WrappedSequenceItem for SequenceItem<Codon, E> {
    type Output = Codon;

    fn unwrap(self) -> Result<Self::Output, ()> {
        match self {
            Self::Some(i) => Ok(i),
            Self::Gap => Err(()), 
            Self::Unknown => Err(()), 
            Self::Err(e) => Err(()), 
        }
    }

    fn to_str(&self) -> String {
        match self {
            Self::Some(i) => i.to_str_().to_string(),
            Self::Gap => "---".to_string(),
            Self::Unknown => "NNN".to_string(),
            Self::Err(_) => "!!!".to_string(),
        }
    }
}


#[derive(Debug, Copy, Clone, PartialEq, Sequence)]
#[pyclass]
pub enum AminoAcid {
    Ala,
    Arg,
    Asn,
    Asp,
    // Asx,
    Cys,
    Glu,
    Gln,
    // Glx,
    Gly,
    His,
    Ile,
    Leu,
    Lys,
    Met,
    Phe,
    Pro,
    Ser,
    Thr,
    Trp,
    Tyr,
    Val,
    Stop,  // Stop
}

// Converters and translators
impl AminoAcid {
    // Enum to string representation=
    pub fn to_three_str_(&self) -> &str {
        match self {
            Self::Ala => "Ala",
            Self::Arg => "Arg",
            Self::Asn => "Asn",
            Self::Asp => "Asp",
            // Self::Asx => "Asx",
            Self::Cys => "Cys",
            Self::Glu => "Glu",
            Self::Gln => "Gln",
            // Self::Glx => "Glx",
            Self::Gly => "Gly",
            Self::His => "His",
            Self::Ile => "Ile",
            Self::Leu => "Leu",
            Self::Lys => "Lys",
            Self::Met => "Met",
            Self::Phe => "Phe",
            Self::Pro => "Pro",
            Self::Ser => "Ser",
            Self::Thr => "Thr",
            Self::Trp => "Trp",
            Self::Tyr => "Tyr",
            Self::Val => "Val",
            Self::Stop => "Ter",
        }
    }    

    pub fn to_one_char_(&self) -> &char {
        match self {
            Self::Ala => &'A',
            Self::Arg => &'R',
            Self::Asn => &'N',
            Self::Asp => &'D',
            // Self::Asx => &'B',
            Self::Cys => &'C',
            Self::Glu => &'E',
            Self::Gln => &'Q',
            // Self::Glx => &'Z',
            Self::Gly => &'G',
            Self::His => &'H',
            Self::Ile => &'I',
            Self::Leu => &'L',
            Self::Lys => &'K',
            Self::Met => &'M',
            Self::Phe => &'F',
            Self::Pro => &'P',
            Self::Ser => &'S',
            Self::Thr => &'T',
            Self::Trp => &'W',
            Self::Tyr => &'Y',
            Self::Val => &'V',
            Self::Stop => &'*',
        }
    }
    
    // From string/char to enum
    pub fn from_three_str_(aa: &str) -> SequenceItem<Self, ()> {
        match aa.to_ascii_lowercase().as_str() {
            "ala" => SequenceItem::Some(Self::Ala),
            "arg" => SequenceItem::Some(Self::Arg),
            "asn" => SequenceItem::Some(Self::Asn),
            "asp" => SequenceItem::Some(Self::Asp),
            // "asx" => SequenceItem::Some(Self::Asx),
            "cys" => SequenceItem::Some(Self::Cys),
            "glu" => SequenceItem::Some(Self::Glu),
            "gln" => SequenceItem::Some(Self::Gln),
            // "glx" => SequenceItem::Some(Self::Glx),
            "gly" => SequenceItem::Some(Self::Gly),
            "his" => SequenceItem::Some(Self::His),
            "ile" => SequenceItem::Some(Self::Ile),
            "leu" => SequenceItem::Some(Self::Leu),
            "lys" => SequenceItem::Some(Self::Lys),
            "met" => SequenceItem::Some(Self::Met),
            "phe" => SequenceItem::Some(Self::Phe),
            "pro" => SequenceItem::Some(Self::Pro),
            "ser" => SequenceItem::Some(Self::Ser),
            "thr" => SequenceItem::Some(Self::Thr),
            "trp" => SequenceItem::Some(Self::Trp),
            "tyr" => SequenceItem::Some(Self::Tyr),
            "val" => SequenceItem::Some(Self::Val),
            "ter" => SequenceItem::Some(Self::Stop),
            "unk" => SequenceItem::Unknown,
            "gap" => SequenceItem::Gap,
            _ => SequenceItem::Err(()),  // TODO: Make an error type
        }
    }

    pub fn from_one_char_(aa: &char) -> SequenceItem<Self, ()> {
        match aa.to_ascii_uppercase() {
            'A' => SequenceItem::Some(Self::Ala),
            'R' => SequenceItem::Some(Self::Arg),
            'N' => SequenceItem::Some(Self::Asn),
            'D' => SequenceItem::Some(Self::Asp),
            // 'B' => SequenceItem::Some(Self::Asx),
            'C' => SequenceItem::Some(Self::Cys),
            'E' => SequenceItem::Some(Self::Glu),
            'Q' => SequenceItem::Some(Self::Gln),
            // 'Z' => SequenceItem::Some(Self::Glx),
            'G' => SequenceItem::Some(Self::Gly),
            'H' => SequenceItem::Some(Self::His),
            'I' => SequenceItem::Some(Self::Ile),
            'L' => SequenceItem::Some(Self::Leu),
            'K' => SequenceItem::Some(Self::Lys),
            'M' => SequenceItem::Some(Self::Met),
            'F' => SequenceItem::Some(Self::Phe),
            'P' => SequenceItem::Some(Self::Pro),
            'S' => SequenceItem::Some(Self::Ser),
            'T' => SequenceItem::Some(Self::Thr),
            'W' => SequenceItem::Some(Self::Trp),
            'Y' => SequenceItem::Some(Self::Tyr),
            'V' => SequenceItem::Some(Self::Val),
            '*' => SequenceItem::Some(Self::Stop),
            'X' => SequenceItem::Unknown,
            '-' => SequenceItem::Gap,
            _ => SequenceItem::Err(()),  // TODO: Make an error type
        }
    }

    // Backtranslate
    pub fn backtranslate_(&self) -> Vec<Codon> {
        match self {
            Self::Ala => vec![Codon::GCA, Codon::GCC, Codon::GCG, Codon::GCT],
            Self::Arg => vec![Codon::AGA, Codon::AGG,Codon::CGA,Codon::CGC,Codon::CGG,Codon::CGT],
            Self::Asn => vec![Codon::AAC, Codon::AAT],
            Self::Asp => vec![Codon::GAC, Codon::GAT],
            Self::Cys => vec![Codon::TGC, Codon::TGT],
            Self::Glu => vec![Codon::GAA, Codon::GAG],
            Self::Gln => vec![Codon::CAA, Codon::CAG],
            Self::Gly => vec![Codon::GGA, Codon::GGC, Codon::GGG, Codon::GGT],
            Self::His => vec![Codon::CAC, Codon::CAT],
            Self::Ile => vec![Codon::ATA, Codon::ATC, Codon::ATT],
            Self::Leu => vec![Codon::CTA, Codon::CTC, Codon::CTG, Codon::CTT, Codon::TTA, Codon::TTG],
            Self::Lys => vec![Codon::AAA, Codon::AAG],
            Self::Met => vec![Codon::ATG],
            Self::Phe => vec![Codon::TTC, Codon::TTT],
            Self::Pro => vec![Codon::CCA, Codon::CCC, Codon::CCG, Codon::CCT],
            Self::Ser => vec![Codon::AGC, Codon::AGT, Codon::TCA, Codon::TCC, Codon::TCG, Codon::TCT],
            Self::Thr => vec![Codon::ACA, Codon::ACC, Codon::ACG, Codon::ACT],
            Self::Trp => vec![Codon::TGG],
            Self::Tyr => vec![Codon::TAC, Codon::TAT],
            Self::Val => vec![Codon::GTA, Codon::GTC, Codon::GTG, Codon::GTT],
            Self::Stop => vec![Codon::TAG, Codon::TGA, Codon::TAA]
        }
    }
}


#[derive(Debug, Copy, Clone, PartialEq, Sequence)]
#[pyclass]
pub enum Base {
    A,
    C,
    G,
    T,
}

impl Base {
    pub fn to_char_(&self) -> &char {
        match self {
            Self::A => &'A',
            Self::C => &'C',
            Self::G => &'G',
            Self::T => &'T',
        }
    }

    pub fn from_char_(c: &char) -> SequenceItem<Self, ()> {
        match c.to_ascii_uppercase() {
            'A' => SequenceItem::Some(Self::A),
            'C' => SequenceItem::Some(Self::C),
            'G' => SequenceItem::Some(Self::G),
            'T' => SequenceItem::Some(Self::T),
            'N' => SequenceItem::Unknown,
            '-' => SequenceItem::Gap,
            _ => SequenceItem::Err(()),  // TODO: Make an error type
        }
    }
}


#[derive(Debug, Copy, Clone, PartialEq, Sequence)]
#[pyclass]
pub enum Codon {
    AAA,
    AAC,
    AAG,
    AAT,
    ACA,
    ACC,
    ACG,
    ACT,
    AGA,
    AGC,
    AGG,
    AGT,
    ATA,
    ATC,
    ATG,
    ATT,
    CAA,
    CAC,
    CAG,
    CAT,
    CCA,
    CCC,
    CCG,
    CCT,
    CGA,
    CGC,
    CGG,
    CGT,
    CTA,
    CTC,
    CTG,
    CTT,
    GAA,
    GAC,
    GAG,
    GAT,
    GCA,
    GCC,
    GCG,
    GCT,
    GGA,
    GGC,
    GGG,
    GGT,
    GTA,
    GTC,
    GTG,
    GTT,
    TAA,
    TAC,
    TAG,
    TAT,
    TCA,
    TCC,
    TCG,
    TCT,
    TGA,
    TGC,
    TGG,
    TGT,
    TTA,
    TTC,
    TTG,
    TTT,
}

impl Codon {
    pub fn to_str_(&self) -> &str {
        match self {
            Self::AAA => "AAA",
            Self::AAC => "AAC",
            Self::AAG => "AAG",
            Self::AAT => "AAT",
            Self::ACA => "ACA",
            Self::ACC => "ACC",
            Self::ACG => "ACG",
            Self::ACT => "ACT",
            Self::AGA => "AGA",
            Self::AGC => "AGC",
            Self::AGG => "AGG",
            Self::AGT => "AGT",
            Self::ATA => "ATA",
            Self::ATC => "ATC",
            Self::ATG => "ATG",
            Self::ATT => "ATT",
            Self::CAA => "CAA",
            Self::CAC => "CAC",
            Self::CAG => "CAG",
            Self::CAT => "CAT",
            Self::CCA => "CCA",
            Self::CCC => "CCC",
            Self::CCG => "CCG",
            Self::CCT => "CCT",
            Self::CGA => "CGA",
            Self::CGC => "CGC",
            Self::CGG => "CGG",
            Self::CGT => "CGT",
            Self::CTA => "CTA",
            Self::CTC => "CTC",
            Self::CTG => "CTG",
            Self::CTT => "CTT",
            Self::GAA => "GAA",
            Self::GAC => "GAC",
            Self::GAG => "GAG",
            Self::GAT => "GAT",
            Self::GCA => "GCA",
            Self::GCC => "GCC",
            Self::GCG => "GCG",
            Self::GCT => "GCT",
            Self::GGA => "GGA",
            Self::GGC => "GGC",
            Self::GGG => "GGG",
            Self::GGT => "GGT",
            Self::GTA => "GTA",
            Self::GTC => "GTC",
            Self::GTG => "GTG",
            Self::GTT => "GTT",
            Self::TAA => "TAA",
            Self::TAC => "TAC",
            Self::TAG => "TAG",
            Self::TAT => "TAT",
            Self::TCA => "TCA",
            Self::TCC => "TCC",
            Self::TCG => "TCG",
            Self::TCT => "TCT",
            Self::TGA => "TGA",
            Self::TGC => "TGC",
            Self::TGG => "TGG",
            Self::TGT => "TGT",
            Self::TTA => "TTA",
            Self::TTC => "TTC",
            Self::TTG => "TTG",
            Self::TTT => "TTT",
        }
    }

    pub fn to_bases_(&self) -> [Base; 3] {
        match self {
            Self::AAA => [Base::A, Base::A, Base::A],
            Self::AAC => [Base::A, Base::A, Base::C],
            Self::AAG => [Base::A, Base::A, Base::G],
            Self::AAT => [Base::A, Base::A, Base::T],
            Self::ACA => [Base::A, Base::C, Base::A],
            Self::ACC => [Base::A, Base::C, Base::C],
            Self::ACG => [Base::A, Base::C, Base::G],
            Self::ACT => [Base::A, Base::C, Base::T],
            Self::AGA => [Base::A, Base::G, Base::A],
            Self::AGC => [Base::A, Base::G, Base::C],
            Self::AGG => [Base::A, Base::G, Base::G],
            Self::AGT => [Base::A, Base::G, Base::T],
            Self::ATA => [Base::A, Base::T, Base::A],
            Self::ATC => [Base::A, Base::T, Base::C],
            Self::ATG => [Base::A, Base::T, Base::G],
            Self::ATT => [Base::A, Base::T, Base::T],
            Self::CAA => [Base::C, Base::A, Base::A],
            Self::CAC => [Base::C, Base::A, Base::C],
            Self::CAG => [Base::C, Base::A, Base::G],
            Self::CAT => [Base::C, Base::A, Base::T],
            Self::CCA => [Base::C, Base::C, Base::A],
            Self::CCC => [Base::C, Base::C, Base::C],
            Self::CCG => [Base::C, Base::C, Base::G],
            Self::CCT => [Base::C, Base::C, Base::T],
            Self::CGA => [Base::C, Base::G, Base::A],
            Self::CGC => [Base::C, Base::G, Base::C],
            Self::CGG => [Base::C, Base::G, Base::G],
            Self::CGT => [Base::C, Base::G, Base::T],
            Self::CTA => [Base::C, Base::T, Base::A],
            Self::CTC => [Base::C, Base::T, Base::C],
            Self::CTG => [Base::C, Base::T, Base::G],
            Self::CTT => [Base::C, Base::T, Base::T],
            Self::GAA => [Base::G, Base::A, Base::A],
            Self::GAC => [Base::G, Base::A, Base::C],
            Self::GAG => [Base::G, Base::A, Base::G],
            Self::GAT => [Base::G, Base::A, Base::T],
            Self::GCA => [Base::G, Base::C, Base::A],
            Self::GCC => [Base::G, Base::C, Base::C],
            Self::GCG => [Base::G, Base::C, Base::G],
            Self::GCT => [Base::G, Base::C, Base::T],
            Self::GGA => [Base::G, Base::G, Base::A],
            Self::GGC => [Base::G, Base::G, Base::C],
            Self::GGG => [Base::G, Base::G, Base::G],
            Self::GGT => [Base::G, Base::G, Base::T],
            Self::GTA => [Base::G, Base::T, Base::A],
            Self::GTC => [Base::G, Base::T, Base::C],
            Self::GTG => [Base::G, Base::T, Base::G],
            Self::GTT => [Base::G, Base::T, Base::T],
            Self::TAA => [Base::T, Base::A, Base::A],
            Self::TAC => [Base::T, Base::A, Base::C],
            Self::TAG => [Base::T, Base::A, Base::G],
            Self::TAT => [Base::T, Base::A, Base::T],
            Self::TCA => [Base::T, Base::C, Base::A],
            Self::TCC => [Base::T, Base::C, Base::C],
            Self::TCG => [Base::T, Base::C, Base::G],
            Self::TCT => [Base::T, Base::C, Base::T],
            Self::TGA => [Base::T, Base::G, Base::A],
            Self::TGC => [Base::T, Base::G, Base::C],
            Self::TGG => [Base::T, Base::G, Base::G],
            Self::TGT => [Base::T, Base::G, Base::T],
            Self::TTA => [Base::T, Base::T, Base::A],
            Self::TTC => [Base::T, Base::T, Base::C],
            Self::TTG => [Base::T, Base::T, Base::G],
            Self::TTT => [Base::T, Base::T, Base::T],
        }
    }

    pub fn from_str_(codon: &str) -> SequenceItem<Self, ()> {
        // TODO: Change U to T
        match codon {
            "AAA" => SequenceItem::Some(Self::AAA),
            "AAC" => SequenceItem::Some(Self::AAC),
            "AAG" => SequenceItem::Some(Self::AAG),
            "AAT" => SequenceItem::Some(Self::AAT),
            "ACA" => SequenceItem::Some(Self::ACA),
            "ACC" => SequenceItem::Some(Self::ACC),
            "ACG" => SequenceItem::Some(Self::ACG),
            "ACT" => SequenceItem::Some(Self::ACT),
            "AGA" => SequenceItem::Some(Self::AGA),
            "AGC" => SequenceItem::Some(Self::AGC),
            "AGG" => SequenceItem::Some(Self::AGG),
            "AGT" => SequenceItem::Some(Self::AGT),
            "ATA" => SequenceItem::Some(Self::ATA),
            "ATC" => SequenceItem::Some(Self::ATC),
            "ATG" => SequenceItem::Some(Self::ATG),
            "ATT" => SequenceItem::Some(Self::ATT),
            "CAA" => SequenceItem::Some(Self::CAA),
            "CAC" => SequenceItem::Some(Self::CAC),
            "CAG" => SequenceItem::Some(Self::CAG),
            "CAT" => SequenceItem::Some(Self::CAT),
            "CCA" => SequenceItem::Some(Self::CCA),
            "CCC" => SequenceItem::Some(Self::CCC),
            "CCG" => SequenceItem::Some(Self::CCG),
            "CCT" => SequenceItem::Some(Self::CCT),
            "CGA" => SequenceItem::Some(Self::CGA),
            "CGC" => SequenceItem::Some(Self::CGC),
            "CGG" => SequenceItem::Some(Self::CGG),
            "CGT" => SequenceItem::Some(Self::CGT),
            "CTA" => SequenceItem::Some(Self::CTA),
            "CTC" => SequenceItem::Some(Self::CTC),
            "CTG" => SequenceItem::Some(Self::CTG),
            "CTT" => SequenceItem::Some(Self::CTT),
            "GAA" => SequenceItem::Some(Self::GAA),
            "GAC" => SequenceItem::Some(Self::GAC),
            "GAG" => SequenceItem::Some(Self::GAG),
            "GAT" => SequenceItem::Some(Self::GAT),
            "GCA" => SequenceItem::Some(Self::GCA),
            "GCC" => SequenceItem::Some(Self::GCC),
            "GCG" => SequenceItem::Some(Self::GCG),
            "GCT" => SequenceItem::Some(Self::GCT),
            "GGA" => SequenceItem::Some(Self::GGA),
            "GGC" => SequenceItem::Some(Self::GGC),
            "GGG" => SequenceItem::Some(Self::GGG),
            "GGT" => SequenceItem::Some(Self::GGT),
            "GTA" => SequenceItem::Some(Self::GTA),
            "GTC" => SequenceItem::Some(Self::GTC),
            "GTG" => SequenceItem::Some(Self::GTG),
            "GTT" => SequenceItem::Some(Self::GTT),
            "TAA" => SequenceItem::Some(Self::TAA),
            "TAC" => SequenceItem::Some(Self::TAC),
            "TAG" => SequenceItem::Some(Self::TAG),
            "TAT" => SequenceItem::Some(Self::TAT),
            "TCA" => SequenceItem::Some(Self::TCA),
            "TCC" => SequenceItem::Some(Self::TCC),
            "TCG" => SequenceItem::Some(Self::TCG),
            "TCT" => SequenceItem::Some(Self::TCT),
            "TGA" => SequenceItem::Some(Self::TGA),
            "TGC" => SequenceItem::Some(Self::TGC),
            "TGG" => SequenceItem::Some(Self::TGG),
            "TGT" => SequenceItem::Some(Self::TGT),
            "TTA" => SequenceItem::Some(Self::TTA),
            "TTC" => SequenceItem::Some(Self::TTC),
            "TTG" => SequenceItem::Some(Self::TTG),
            "TTT" => SequenceItem::Some(Self::TTT),
            "NNN" => SequenceItem::Unknown,
            "---" => SequenceItem::Gap,
            _ => SequenceItem::Err(()),  // TODO: Make an error type
        }
    }
    
    pub fn from_bases_(b1: Base, b2: Base, b3: Base) -> Result<Self, &'static str> {
        match [b1, b2, b3] {
            [Base::A, Base::A, Base::A] => Ok(Self::AAA),
            [Base::A, Base::A, Base::C] => Ok(Self::AAC),
            [Base::A, Base::A, Base::G] => Ok(Self::AAG),
            [Base::A, Base::A, Base::T] => Ok(Self::AAT),
            [Base::A, Base::C, Base::A] => Ok(Self::ACA),
            [Base::A, Base::C, Base::C] => Ok(Self::ACC),
            [Base::A, Base::C, Base::G] => Ok(Self::ACG),
            [Base::A, Base::C, Base::T] => Ok(Self::ACT),
            [Base::A, Base::G, Base::A] => Ok(Self::AGA),
            [Base::A, Base::G, Base::C] => Ok(Self::AGC),
            [Base::A, Base::G, Base::G] => Ok(Self::AGG),
            [Base::A, Base::G, Base::T] => Ok(Self::AGT),
            [Base::A, Base::T, Base::A] => Ok(Self::ATA),
            [Base::A, Base::T, Base::C] => Ok(Self::ATC),
            [Base::A, Base::T, Base::G] => Ok(Self::ATG),
            [Base::A, Base::T, Base::T] => Ok(Self::ATT),
            [Base::C, Base::A, Base::A] => Ok(Self::CAA),
            [Base::C, Base::A, Base::C] => Ok(Self::CAC),
            [Base::C, Base::A, Base::G] => Ok(Self::CAG),
            [Base::C, Base::A, Base::T] => Ok(Self::CAT),
            [Base::C, Base::C, Base::A] => Ok(Self::CCA),
            [Base::C, Base::C, Base::C] => Ok(Self::CCC),
            [Base::C, Base::C, Base::G] => Ok(Self::CCG),
            [Base::C, Base::C, Base::T] => Ok(Self::CCT),
            [Base::C, Base::G, Base::A] => Ok(Self::CGA),
            [Base::C, Base::G, Base::C] => Ok(Self::CGC),
            [Base::C, Base::G, Base::G] => Ok(Self::CGG),
            [Base::C, Base::G, Base::T] => Ok(Self::CGT),
            [Base::C, Base::T, Base::A] => Ok(Self::CTA),
            [Base::C, Base::T, Base::C] => Ok(Self::CTC),
            [Base::C, Base::T, Base::G] => Ok(Self::CTG),
            [Base::C, Base::T, Base::T] => Ok(Self::CTT),
            [Base::G, Base::A, Base::A] => Ok(Self::GAA),
            [Base::G, Base::A, Base::C] => Ok(Self::GAC),
            [Base::G, Base::A, Base::G] => Ok(Self::GAG),
            [Base::G, Base::A, Base::T] => Ok(Self::GAT),
            [Base::G, Base::C, Base::A] => Ok(Self::GCA),
            [Base::G, Base::C, Base::C] => Ok(Self::GCC),
            [Base::G, Base::C, Base::G] => Ok(Self::GCG),
            [Base::G, Base::C, Base::T] => Ok(Self::GCT),
            [Base::G, Base::G, Base::A] => Ok(Self::GGA),
            [Base::G, Base::G, Base::C] => Ok(Self::GGC),
            [Base::G, Base::G, Base::G] => Ok(Self::GGG),
            [Base::G, Base::G, Base::T] => Ok(Self::GGT),
            [Base::G, Base::T, Base::A] => Ok(Self::GTA),
            [Base::G, Base::T, Base::C] => Ok(Self::GTC),
            [Base::G, Base::T, Base::G] => Ok(Self::GTG),
            [Base::G, Base::T, Base::T] => Ok(Self::GTT),
            [Base::T, Base::A, Base::A] => Ok(Self::TAA),
            [Base::T, Base::A, Base::C] => Ok(Self::TAC),
            [Base::T, Base::A, Base::G] => Ok(Self::TAG),
            [Base::T, Base::A, Base::T] => Ok(Self::TAT),
            [Base::T, Base::C, Base::A] => Ok(Self::TCA),
            [Base::T, Base::C, Base::C] => Ok(Self::TCC),
            [Base::T, Base::C, Base::G] => Ok(Self::TCG),
            [Base::T, Base::C, Base::T] => Ok(Self::TCT),
            [Base::T, Base::G, Base::A] => Ok(Self::TGA),
            [Base::T, Base::G, Base::C] => Ok(Self::TGC),
            [Base::T, Base::G, Base::G] => Ok(Self::TGG),
            [Base::T, Base::G, Base::T] => Ok(Self::TGT),
            [Base::T, Base::T, Base::A] => Ok(Self::TTA),
            [Base::T, Base::T, Base::C] => Ok(Self::TTC),
            [Base::T, Base::T, Base::G] => Ok(Self::TTG),
            [Base::T, Base::T, Base::T] => Ok(Self::TTT),
        }
    }

    pub fn translate_(&self) -> Option<AminoAcid> {
        match self {
            Self::AAA => Some(AminoAcid::Lys),
            Self::AAC => Some(AminoAcid::Asn),
            Self::AAG => Some(AminoAcid::Lys),
            Self::AAT => Some(AminoAcid::Asn),
            Self::ACA => Some(AminoAcid::Thr),
            Self::ACC => Some(AminoAcid::Thr),
            Self::ACG => Some(AminoAcid::Thr),
            Self::ACT => Some(AminoAcid::Thr),
            Self::AGA => Some(AminoAcid::Arg),
            Self::AGC => Some(AminoAcid::Ser),
            Self::AGG => Some(AminoAcid::Arg),
            Self::AGT => Some(AminoAcid::Ser),
            Self::ATA => Some(AminoAcid::Ile),
            Self::ATC => Some(AminoAcid::Ile),
            Self::ATG => Some(AminoAcid::Met),
            Self::ATT => Some(AminoAcid::Ile),
            Self::CAA => Some(AminoAcid::Gln),
            Self::CAC => Some(AminoAcid::His),
            Self::CAG => Some(AminoAcid::Gln),
            Self::CAT => Some(AminoAcid::His),
            Self::CCA => Some(AminoAcid::Pro),
            Self::CCC => Some(AminoAcid::Pro),
            Self::CCG => Some(AminoAcid::Pro),
            Self::CCT => Some(AminoAcid::Pro),
            Self::CGA => Some(AminoAcid::Arg),
            Self::CGC => Some(AminoAcid::Arg),
            Self::CGG => Some(AminoAcid::Arg),
            Self::CGT => Some(AminoAcid::Arg),
            Self::CTA => Some(AminoAcid::Leu),
            Self::CTC => Some(AminoAcid::Leu),
            Self::CTG => Some(AminoAcid::Leu),
            Self::CTT => Some(AminoAcid::Leu),
            Self::GAA => Some(AminoAcid::Glu),
            Self::GAC => Some(AminoAcid::Asp),
            Self::GAG => Some(AminoAcid::Glu),
            Self::GAT => Some(AminoAcid::Asp),
            Self::GCA => Some(AminoAcid::Ala),
            Self::GCC => Some(AminoAcid::Ala),
            Self::GCG => Some(AminoAcid::Ala),
            Self::GCT => Some(AminoAcid::Ala),
            Self::GGA => Some(AminoAcid::Gly),
            Self::GGC => Some(AminoAcid::Gly),
            Self::GGG => Some(AminoAcid::Gly),
            Self::GGT => Some(AminoAcid::Gly),
            Self::GTA => Some(AminoAcid::Val),
            Self::GTC => Some(AminoAcid::Val),
            Self::GTG => Some(AminoAcid::Val),
            Self::GTT => Some(AminoAcid::Val),
            Self::TAA => None,
            Self::TAC => Some(AminoAcid::Tyr),
            Self::TAG => None,
            Self::TAT => Some(AminoAcid::Tyr),
            Self::TCA => Some(AminoAcid::Ser),
            Self::TCC => Some(AminoAcid::Ser),
            Self::TCG => Some(AminoAcid::Ser),
            Self::TCT => Some(AminoAcid::Ser),
            Self::TGA => None,
            Self::TGC => Some(AminoAcid::Cys),
            Self::TGG => Some(AminoAcid::Trp),
            Self::TGT => Some(AminoAcid::Cys),
            Self::TTA => Some(AminoAcid::Leu),
            Self::TTC => Some(AminoAcid::Phe),
            Self::TTG => Some(AminoAcid::Leu),
            Self::TTT => Some(AminoAcid::Phe),
        }
    }

    pub fn is_start_codon_(&self) -> bool {
        matches!(self, Codon::ATG)
    }

    pub fn is_stop_codon_(&self) -> bool {
        match self {
            Codon::TAG => true,
            Codon::TGA => true,
            Codon::TAA => true,
            _ => false,
        }
    }

    pub fn is_synonymous_change_(&self, other: &Codon) -> bool {
        match (self.translate_(), other.translate_()) {
            (Some(a), Some(b)) if a == b => true,
            _ => false,
        }
    }

    pub fn one_hit_mutants_by_position_(&self, i: usize) -> Vec<Codon> {
        let base = self.to_bases_()[i];
        all::<Base>()
            // Iterate only non-self bases
            .filter_map(|b| if b == base { None } else { Some(b) })
            // Generate new codon from new 3-Base combination
            .map(|b| {
                let mut bases: [Base; 3] = self.to_bases_();
                bases[i] = b;
                Codon::from_bases_(bases[0], bases[1], bases[2]).unwrap()
            }).collect()
    }

    pub fn one_hit_mutants_(&self) -> Vec<Codon> {
        (0..3)
            .flat_map(|i| {
                self.one_hit_mutants_by_position_(i)
            })
            .collect()
    }

    pub fn base_change_by_position_(&self, other: &Self) -> Vec<Option<(Base, Base)>> {
        let bases1: [Base; 3] = self.to_bases_();
        let other: [Base; 3] = other.to_bases_();
        bases1.into_iter().zip(other.into_iter())
            .map(|(b1, b2)| {
                if b1 == b2 { None }
                else { Some((b1, b2))}
            })
            .collect()
    }

    pub fn count_base_changes_(&self, other: &Self) -> usize {
        self.base_change_by_position_(other).iter()
            .filter_map(|&result| result)
            .count()
    }

    pub fn list_mutation_pathways_(&self, other: &Self) -> Vec<Vec<Codon>> {
        if self.is_stop_codon_() { return vec![] }
        else if other.is_stop_codon_() { return vec![] }

        let changes_vec = self.base_change_by_position_(other);
        let diff1 = changes_vec[0];
        let diff2 = changes_vec[1];
        let diff3 = changes_vec[2];
        let pos_change_paths = match (diff1, diff2, diff3) {
            (None, None, None) => vec![],
            (Some(_), None, None) => vec![vec![0]],
            (None, Some(_), None) => vec![vec![1]],
            (None, None, Some(_)) => vec![vec![2]],
            (Some(_), Some(_), None) => vec![vec![0, 1], vec![1, 0]],
            (Some(_), None, Some(_)) => vec![vec![0, 2], vec![2, 0]],
            (None, Some(_), Some(_)) => vec![vec![1, 2], vec![2, 1]],
            (Some(_), Some(_), Some(_)) => vec![
                    vec![0, 1, 2],
                    vec![2, 1, 0],
                    vec![1, 2, 0],
                    vec![0, 2, 1],
                    vec![2, 0, 1],
                    vec![1, 0, 2],
                ],
        };
        let other_bases = other.to_bases_();
        pos_change_paths.iter()
            .filter_map(|paths| {
                let mut path = vec![self.clone()];
                paths.iter()
                    .for_each(|&i| {
                        let mut bases: [Base; 3] = path.last().unwrap().to_bases_(); 
                        bases[i] = other_bases[i];
                        let codon = Codon::from_bases_(bases[0], bases[1], bases[2]).unwrap();
                        path.push(codon);
                    });
                // Filter paths the contain stop codon in the middle
                for codon in path.iter().rev() {
                    if codon.is_stop_codon_() { return None }
                }
                Some(path)
            })
            .collect()
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct PwAlnItem<T, E>(pub SequenceItem<T, E>, pub SequenceItem<T, E>, pub usize);

impl<T,E> PwAlnItem<T,E> {
    pub fn both_valid(&self) -> bool {
        if let Self(SequenceItem::Some(_), SequenceItem::Some(_), _) = self {
            true
        } else {
            false
        }
    }

    pub fn any_valid(&self) -> bool {
        match self {
            Self(SequenceItem::Some(_), _, _) => true,
            Self(_, SequenceItem::Some(_), _) => true,
            _ => false
        }
    }

    pub fn both_gap(&self) -> bool {
        if let Self(SequenceItem::Gap, SequenceItem::Gap, _) = self {
            true
        } else {
            false
        }
    }

    pub fn any_gap(&self) -> bool {
        match self {
            Self(SequenceItem::Gap, _, _) => true,
            Self(_, SequenceItem::Gap, _) => true,
            _ => false
        }
    }

    pub fn both_unknown(&self) -> bool {
        if let Self(SequenceItem::Unknown, SequenceItem::Unknown, _) = self {
            true
        } else {
            false
        }
    }

    pub fn any_unknown(&self) -> bool {
        match self {
            Self(SequenceItem::Unknown, _, _) => true,
            Self(_, SequenceItem::Unknown, _) => true,
            _ => false
        }
    }

    pub fn both_error(&self) -> bool {
        if let Self(SequenceItem::Err(_), SequenceItem::Err(_), _) = self {
            true
        } else {
            false
        }
    }

    pub fn any_error(&self) -> bool {
        match self {
            Self(SequenceItem::Err(_), _, _) => true,
            Self(_, SequenceItem::Err(_), _) => true,
            _ => false
        }
    }

}



#[cfg(test)]
mod amino_acid_tests {
    use super::*;

    // Amino acid tests
    #[test]
    fn amino_acid_to_three_str() {
        let test_vec: Vec<(AminoAcid, &str)> = vec![
            (AminoAcid::Ala, "Ala"),
            (AminoAcid::Arg, "Arg"),
            (AminoAcid::Asn, "Asn"),
            (AminoAcid::Asp, "Asp"),
            (AminoAcid::Cys, "Cys"),
            (AminoAcid::Glu, "Glu"),
            (AminoAcid::Gln, "Gln"),
            (AminoAcid::Gly, "Gly"),
            (AminoAcid::His, "His"),
            (AminoAcid::Ile, "Ile"),
            (AminoAcid::Leu, "Leu"),
            (AminoAcid::Lys, "Lys"),
            (AminoAcid::Met, "Met"),
            (AminoAcid::Phe, "Phe"),
            (AminoAcid::Pro, "Pro"),
            (AminoAcid::Ser, "Ser"),
            (AminoAcid::Thr, "Thr"),
            (AminoAcid::Trp, "Trp"),
            (AminoAcid::Tyr, "Tyr"),
            (AminoAcid::Val, "Val"),
        ];
        for (aa, aa_str) in test_vec {
            let result = aa.to_three_str_();
            assert_eq!(result, aa_str)
        }
    }

    #[test]
    fn amino_acid_to_one_char() {
        let test_vec: Vec<(AminoAcid, char)> = vec![
            (AminoAcid::Ala, 'A'),
            (AminoAcid::Arg, 'R'),
            (AminoAcid::Asn, 'N'),
            (AminoAcid::Asp, 'D'),
            (AminoAcid::Cys, 'C'),
            (AminoAcid::Glu, 'E'),
            (AminoAcid::Gln, 'Q'),
            (AminoAcid::Gly, 'G'),
            (AminoAcid::His, 'H'),
            (AminoAcid::Ile, 'I'),
            (AminoAcid::Leu, 'L'),
            (AminoAcid::Lys, 'K'),
            (AminoAcid::Met, 'M'),
            (AminoAcid::Phe, 'F'),
            (AminoAcid::Pro, 'P'),
            (AminoAcid::Ser, 'S'),
            (AminoAcid::Thr, 'T'),
            (AminoAcid::Trp, 'W'),
            (AminoAcid::Tyr, 'Y'),
            (AminoAcid::Val, 'V'),
        ];
        for (aa, aa_char) in test_vec {
            let result = aa.to_one_char_();
            assert_eq!(result, &aa_char)
        }        
    }

    #[test]
    fn amino_acid_from_three_str() {
        let test_vec: Vec<(AminoAcid, &str)> = vec![
            (AminoAcid::Ala, "Ala"),
            (AminoAcid::Arg, "Arg"),
            (AminoAcid::Asn, "Asn"),
            (AminoAcid::Asp, "Asp"),
            (AminoAcid::Cys, "Cys"),
            (AminoAcid::Glu, "Glu"),
            (AminoAcid::Gln, "Gln"),
            (AminoAcid::Gly, "Gly"),
            (AminoAcid::His, "His"),
            (AminoAcid::Ile, "Ile"),
            (AminoAcid::Leu, "Leu"),
            (AminoAcid::Lys, "Lys"),
            (AminoAcid::Met, "Met"),
            (AminoAcid::Phe, "Phe"),
            (AminoAcid::Pro, "Pro"),
            (AminoAcid::Ser, "Ser"),
            (AminoAcid::Thr, "Thr"),
            (AminoAcid::Trp, "Trp"),
            (AminoAcid::Tyr, "Tyr"),
            (AminoAcid::Val, "Val"),
            (AminoAcid::Stop, "Ter"),
        ];
        for (aa, aa_str) in test_vec {
            let result = AminoAcid::from_three_str_(aa_str).unwrap().unwrap();
            assert_eq!(result, aa)
        }
        
        // When error
        let result = AminoAcid::from_three_str_("ASD").unwrap();
        assert_eq!(result.is_err(), true)
    }

    #[test]
    fn amino_acid_from_one_str() {
        let test_vec: Vec<(AminoAcid, char)> = vec![
            (AminoAcid::Ala, 'A'),
            (AminoAcid::Arg, 'R'),
            (AminoAcid::Asn, 'N'),
            (AminoAcid::Asp, 'D'),
            (AminoAcid::Cys, 'C'),
            (AminoAcid::Glu, 'E'),
            (AminoAcid::Gln, 'Q'),
            (AminoAcid::Gly, 'G'),
            (AminoAcid::His, 'H'),
            (AminoAcid::Ile, 'I'),
            (AminoAcid::Leu, 'L'),
            (AminoAcid::Lys, 'K'),
            (AminoAcid::Met, 'M'),
            (AminoAcid::Phe, 'F'),
            (AminoAcid::Pro, 'P'),
            (AminoAcid::Ser, 'S'),
            (AminoAcid::Thr, 'T'),
            (AminoAcid::Trp, 'W'),
            (AminoAcid::Tyr, 'Y'),
            (AminoAcid::Val, 'V'),
            (AminoAcid::Stop, '*'),
        ];
        for (aa, aa_char) in test_vec {
            let result = AminoAcid::from_one_char_(&aa_char).unwrap().unwrap();
            assert_eq!(result, aa)
        }
        
        // When error
        let result = AminoAcid::from_one_char_(&'@').unwrap();
        assert_eq!(result.is_err(), true)
    }

    #[test]
    fn amino_acid_backtranslate_single() {
        let aa = AminoAcid::Met;
        assert_eq!(aa.backtranslate_(), vec![Codon::ATG]);
    }

    #[test]
    fn amino_acid_backtranslate_twofold() {
        let aa = AminoAcid::Cys;
        assert_eq!(aa.backtranslate_(), vec![Codon::TGC, Codon::TGT]);
    }

    
    #[test]
    fn amino_acid_backtranslate_fourfold() {
        let aa = AminoAcid::Ala;
        assert_eq!(aa.backtranslate_(), vec![Codon::GCA, Codon::GCC, Codon::GCG, Codon::GCT]);
    }

    #[test]
    fn amino_acid_backtranslate_sixfold() {
        let aa = AminoAcid::Leu;
        assert_eq!(aa.backtranslate_(), vec![Codon::CTA, Codon::CTC, Codon::CTG, Codon::CTT, Codon::TTA, Codon::TTG]);
    }
}

#[cfg(test)]
mod codon_tests {
    use super::*;
    
    #[test]
    fn codon_to_str() {
        let test_vec: Vec<(Codon, &str)> = vec![
            (Codon::AAA, "AAA"),
            (Codon::AAC, "AAC"),
            (Codon::AAG, "AAG"),
            (Codon::AAT, "AAT"),
            (Codon::ACA, "ACA"),
            (Codon::ACC, "ACC"),
            (Codon::ACG, "ACG"),
            (Codon::ACT, "ACT"),
            (Codon::AGA, "AGA"),
            (Codon::AGC, "AGC"),
            (Codon::AGG, "AGG"),
            (Codon::AGT, "AGT"),
            (Codon::ATA, "ATA"),
            (Codon::ATC, "ATC"),
            (Codon::ATG, "ATG"),
            (Codon::ATT, "ATT"),
            (Codon::CAA, "CAA"),
            (Codon::CAC, "CAC"),
            (Codon::CAG, "CAG"),
            (Codon::CAT, "CAT"),
            (Codon::CCA, "CCA"),
            (Codon::CCC, "CCC"),
            (Codon::CCG, "CCG"),
            (Codon::CCT, "CCT"),
            (Codon::CGA, "CGA"),
            (Codon::CGC, "CGC"),
            (Codon::CGG, "CGG"),
            (Codon::CGT, "CGT"),
            (Codon::CTA, "CTA"),
            (Codon::CTC, "CTC"),
            (Codon::CTG, "CTG"),
            (Codon::CTT, "CTT"),
            (Codon::GAA, "GAA"),
            (Codon::GAC, "GAC"),
            (Codon::GAG, "GAG"),
            (Codon::GAT, "GAT"),
            (Codon::GCA, "GCA"),
            (Codon::GCC, "GCC"),
            (Codon::GCG, "GCG"),
            (Codon::GCT, "GCT"),
            (Codon::GGA, "GGA"),
            (Codon::GGC, "GGC"),
            (Codon::GGG, "GGG"),
            (Codon::GGT, "GGT"),
            (Codon::GTA, "GTA"),
            (Codon::GTC, "GTC"),
            (Codon::GTG, "GTG"),
            (Codon::GTT, "GTT"),
            (Codon::TAA, "TAA"),
            (Codon::TAC, "TAC"),
            (Codon::TAG, "TAG"),
            (Codon::TAT, "TAT"),
            (Codon::TCA, "TCA"),
            (Codon::TCC, "TCC"),
            (Codon::TCG, "TCG"),
            (Codon::TCT, "TCT"),
            (Codon::TGA, "TGA"),
            (Codon::TGC, "TGC"),
            (Codon::TGG, "TGG"),
            (Codon::TGT, "TGT"),
            (Codon::TTA, "TTA"),
            (Codon::TTC, "TTC"),
            (Codon::TTG, "TTG"),
            (Codon::TTT, "TTT"),
        ];
        for (codon, codon_str) in test_vec {
            let result = codon.to_str_();
            assert_eq!(result, codon_str)
        }
    }

    // #[test]
    // fn codon_to_bases() {}

    #[test]
    fn codon_from_str() {
        let test_vec: Vec<(Codon, &str)> = vec![
            (Codon::AAA, "AAA"),
            (Codon::AAC, "AAC"),
            (Codon::AAG, "AAG"),
            (Codon::AAT, "AAT"),
            (Codon::ACA, "ACA"),
            (Codon::ACC, "ACC"),
            (Codon::ACG, "ACG"),
            (Codon::ACT, "ACT"),
            (Codon::AGA, "AGA"),
            (Codon::AGC, "AGC"),
            (Codon::AGG, "AGG"),
            (Codon::AGT, "AGT"),
            (Codon::ATA, "ATA"),
            (Codon::ATC, "ATC"),
            (Codon::ATG, "ATG"),
            (Codon::ATT, "ATT"),
            (Codon::CAA, "CAA"),
            (Codon::CAC, "CAC"),
            (Codon::CAG, "CAG"),
            (Codon::CAT, "CAT"),
            (Codon::CCA, "CCA"),
            (Codon::CCC, "CCC"),
            (Codon::CCG, "CCG"),
            (Codon::CCT, "CCT"),
            (Codon::CGA, "CGA"),
            (Codon::CGC, "CGC"),
            (Codon::CGG, "CGG"),
            (Codon::CGT, "CGT"),
            (Codon::CTA, "CTA"),
            (Codon::CTC, "CTC"),
            (Codon::CTG, "CTG"),
            (Codon::CTT, "CTT"),
            (Codon::GAA, "GAA"),
            (Codon::GAC, "GAC"),
            (Codon::GAG, "GAG"),
            (Codon::GAT, "GAT"),
            (Codon::GCA, "GCA"),
            (Codon::GCC, "GCC"),
            (Codon::GCG, "GCG"),
            (Codon::GCT, "GCT"),
            (Codon::GGA, "GGA"),
            (Codon::GGC, "GGC"),
            (Codon::GGG, "GGG"),
            (Codon::GGT, "GGT"),
            (Codon::GTA, "GTA"),
            (Codon::GTC, "GTC"),
            (Codon::GTG, "GTG"),
            (Codon::GTT, "GTT"),
            (Codon::TAA, "TAA"),
            (Codon::TAC, "TAC"),
            (Codon::TAG, "TAG"),
            (Codon::TAT, "TAT"),
            (Codon::TCA, "TCA"),
            (Codon::TCC, "TCC"),
            (Codon::TCG, "TCG"),
            (Codon::TCT, "TCT"),
            (Codon::TGA, "TGA"),
            (Codon::TGC, "TGC"),
            (Codon::TGG, "TGG"),
            (Codon::TGT, "TGT"),
            (Codon::TTA, "TTA"),
            (Codon::TTC, "TTC"),
            (Codon::TTG, "TTG"),
            (Codon::TTT, "TTT"),
        ];
        for (codon, codon_str) in test_vec {
            let result = Codon::from_str_(codon_str).unwrap().unwrap();
            assert_eq!(result, codon)
        }
        
        // When error
        let result = Codon::from_str_("QWE").unwrap();
        assert_eq!(result.is_err(), true)
    }

    // #[test]
    // fn codon_from_bases() {}

    // #[test]
    // fn codon_translate() {}

    // #[test]
    // fn codon_is_start_codon() {}
    
    // #[test]
    // fn codon_is_stop_codon() {}

    // #[test]
    // fn codon_is_synonymous_change() {}

    // #[test]
    // fn codon_one_hit_mutants_by_position() {}

    // #[test]
    // fn codon_one_hit_mutants() {}
    
    // #[test]
    // fn codon_base_change_by_position() {}

    // #[test]
    // fn codon_count_base_changes() {}

    #[test]
    fn codon_generate_mutation_pathways_same() {
        let codon1 = Codon::ATG;
        let codon2 = Codon::ATG;
        let pathways = codon1.list_mutation_pathways_(&codon2);
        let expected: Vec<Vec<Codon>> = vec![];
        assert_eq!(pathways, expected);
    }

    #[test]
    fn codon_generate_mutation_pathways_one_change() {
        let test_list: [(Codon, Codon); 3] = [
            (Codon::TTT, Codon::CTT),
            (Codon::TTT, Codon::TAT),
            (Codon::TTT, Codon::TTG),
        ];
        for (codon1, codon2) in test_list {
            let pathways = codon1.list_mutation_pathways_(&codon2);
            let expected: Vec<Vec<Codon>> = vec![vec![codon1, codon2]];
            assert_eq!(pathways, expected);
        }
    }

    #[test]
    fn codon_generate_mutation_pathways_two_changes() {
        let test_list: [(Codon, Codon); 3] = [
            (Codon::TTT, Codon::ACT),
            (Codon::TTT, Codon::TCG),
            (Codon::TTT, Codon::ATG),
        ];
        let result_list: [Vec<Vec<Codon>>; 3] = [
            vec![
                vec![Codon::TTT, Codon::ATT, Codon::ACT],
                vec![Codon::TTT, Codon::TCT, Codon::ACT],
            ],
            vec![
                vec![Codon::TTT, Codon::TCT, Codon::TCG],
                vec![Codon::TTT, Codon::TTG, Codon::TCG],
            ],
            vec![
                vec![Codon::TTT, Codon::ATT, Codon::ATG],
                vec![Codon::TTT, Codon::TTG, Codon::ATG],
            ],
        ];
        for (i, (codon1, codon2)) in test_list.into_iter().enumerate() {
            let pathways = codon1.list_mutation_pathways_(&codon2);
            let expected: Vec<Vec<Codon>> = result_list[i].clone();
            assert_eq!(pathways, expected);
        }
    }

    #[test]
    fn codon_generate_mutation_pathways_three_changes() {
        let result: Vec<Vec<Codon>> = vec![
            vec![Codon::TTT, Codon::GTT, Codon::GGT, Codon::GGG],
            vec![Codon::TTT, Codon::TTG, Codon::TGG, Codon::GGG],
            vec![Codon::TTT, Codon::TGT, Codon::TGG, Codon::GGG],
            vec![Codon::TTT, Codon::GTT, Codon::GTG, Codon::GGG],
            vec![Codon::TTT, Codon::TTG, Codon::GTG, Codon::GGG],
            vec![Codon::TTT, Codon::TGT, Codon::GGT, Codon::GGG],
        ];
        let (codon1, codon2) = (Codon::TTT, Codon::GGG);
        let pathways = codon1.list_mutation_pathways_(&codon2);
        let expected: Vec<Vec<Codon>> = result;
        assert_eq!(pathways, expected);
        
    }

    #[test]
    fn codon_generate_mutation_pathways_stop_codon_final_result() {
        let (codon1, codon2) = (Codon::ATG, Codon::TAG);
        let expected: Vec<Vec<Codon>> = vec![];
        let pathways = codon1.list_mutation_pathways_(&codon2);
        assert_eq!(pathways, expected);
    }

    #[test]
    fn codon_generate_mutation_pathways_stop_codon_middle_result() {
        let (codon1, codon2) = (Codon::AAG, Codon::TTG);
        let expected = vec![
            // vec![Codon::AAG, Codon::TAG, Codon::TTG],  // stop codon intermediate
            vec![Codon::AAG, Codon::ATG, Codon::TTG],
        ];
        let pathways = codon1.list_mutation_pathways_(&codon2);
        assert_eq!(pathways, expected);
    }

    #[test]
    fn codon_generate_mutation_pathways_stop_codon_original() {
        let (codon1, codon2) = (Codon::TAA, Codon::TTT);
        let expected: Vec<Vec<Codon>> = vec![];
        let pathways = codon1.list_mutation_pathways_(&codon2);
        assert_eq!(pathways, expected);
    }

}
