use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;

use crate::dtype::{Base, AminoAcid, Codon};


pub mod dtype {
    use super::*;
    use pyo3::types::PyType;
    use crate::dtype::*;


    #[pymethods]
    impl AminoAcid {
        #[classmethod]
        pub fn from_str(_cls: &PyType, s: &str) -> PyResult<Self> {
            let chars = s.chars().collect::<Vec<char>>();
            match chars.len() {
                1 => match Self::from_one_char_(&chars[0]) {
                    Ok(v) => Ok(v),
                    Err(e) => Err(PyValueError::new_err(e))
                },
                3 => match Self::from_three_str_(s) {
                    Ok(v) => Ok(v),
                    Err(e) => Err(PyValueError::new_err(e))
                },
                _ => Err(PyValueError::new_err(())),
            }
        }

        pub fn to_str(&self) -> PyResult<String> {
            Ok(self.__str__())
        }

        pub fn to_one_str(&self) -> PyResult<String> {
            Ok(self.to_one_char_().to_string())
        }

        pub fn to_three_str(&self) -> PyResult<String> {
            Ok(self.to_three_str_().to_string())
        }

        pub fn __str__(&self) -> String {
            self.to_one_char_().to_string()
        }

        // pub fn __repr__(&self) -> String {
        //     self.to_one_char().to_string()
        // }
    }

    #[pymethods]
    impl Base {
        #[classmethod]
        pub fn from_str(_cls: &PyType, s: &str) -> PyResult<Self> {
            let chars = s.chars().collect::<Vec<char>>();
            match chars.len() {
                1 => match Self::from_char_(&chars[0]) {
                    Ok(v) => Ok(v),
                    Err(e) => Err(PyValueError::new_err(e))
                },
                _ => Err(PyValueError::new_err(())),
            }
        }

        pub fn to_str(&self) -> PyResult<String> {
            Ok(self.__str__())
        }

        pub fn __str__(&self) -> String {
            self.to_char_().to_string()
        }

        // pub fn __repr__(&self) -> String {
        //     self.to_char().to_string()
        // }
    }

    #[pymethods]
    impl Codon {
        #[classmethod]
        pub fn from_str(_cls: &PyType, s: &str) -> PyResult<Self> {
            let chars = s.chars().collect::<Vec<char>>();
            match chars.len() {
                3 => match Self::from_str_(s) {
                    Ok(v) => Ok(v),
                    Err(e) => Err(PyValueError::new_err(e))
                },
                _ => Err(PyValueError::new_err(())),
            }
        }

        pub fn to_str(&self) -> PyResult<String> {
            Ok(self.__str__())
        }

        pub fn is_start_codon(&self) -> PyResult<bool> {
            Ok(self.is_start_codon_())
        }

        pub fn is_stop_codon(&self) -> PyResult<bool> {
            Ok(self.is_stop_codon_())
        }

        pub fn is_synonymous_change(&self, other: &Codon) -> PyResult<bool> {
            Ok(self.is_synonymous_change_(other))
        }
        
        pub fn translate(&self) -> PyResult<Option<AminoAcid>> {
            Ok(self.translate_())
        }

        pub fn __str__(&self) -> String {
            self.to_str_().to_string()
        }

        // pub fn __repr__(&self) -> String {
        //     self.to_str().to_string()
        // }
    }
}

pub mod convert {
    use super::*;
    use crate::convert::{
        str_to_bases as str_to_bases_,
        str_to_amino_acids as str_to_amino_acids_,
        str_to_codons as str_to_codons_,
    };

    #[pyfunction]
    pub fn str_to_bases(s: &str) -> PyResult<Vec<Base>> {
        match str_to_bases_(s) {
            Ok(v) => Ok(v),
            Err(e) => Err(PyValueError::new_err(e)),
        }
    }
    
    #[pyfunction]
    pub fn str_to_amino_acids(s: &str) -> PyResult<Vec<AminoAcid>> {
        match str_to_amino_acids_(s) {
            Ok(v) => Ok(v),
            Err(e) => Err(PyValueError::new_err(e)),
        }
    }
    
    #[pyfunction]
    pub fn str_to_codons(s: &str) -> PyResult<Vec<Codon>> {
        match str_to_codons_(s) {
            Ok(v) => Ok(v),
            Err(e) => Err(PyValueError::new_err(e)),
        }
    }
}

pub mod count {
    use super::*;
    use crate::count::{
        count_sites as count_sites_,
        count_differences as count_differences_,
    };

    #[pyfunction]
    pub fn count_sites(codon1: Codon, codon2: Codon) -> PyResult<(f64, f64)> {
        match count_sites_(&codon1, &codon2) {
            Ok(v) => Ok(v),
            Err(e) => Err(PyValueError::new_err(e))
        }
    }

    #[pyfunction]
    pub fn count_differences(codon1: Codon, codon2: Codon) -> PyResult<(f64, f64)> {
        match count_differences_(&codon1, &codon2) {
            Ok(v) => Ok(v),
            Err(e) => Err(PyValueError::new_err(e))
        }
    }
}
