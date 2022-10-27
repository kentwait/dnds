extern crate enum_iterator;
use pyo3::prelude::*;

mod dtype;
mod convert;
mod count;
// mod pnps;
mod python;


#[pymodule]
fn dnds(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    // Classes
    m.add_class::<dtype::AminoAcid>()?;
    m.add_class::<dtype::Base>()?;
    m.add_class::<dtype::Codon>()?;

    // Conversion functions
    m.add_function(wrap_pyfunction!(python::convert::str_to_amino_acids, m)?)?;
    m.add_function(wrap_pyfunction!(python::convert::str_to_bases, m)?)?;
    m.add_function(wrap_pyfunction!(python::convert::str_to_codons, m)?)?;

    // Counting functions
    m.add_function(wrap_pyfunction!(python::count::count_sites, m)?)?;
    m.add_function(wrap_pyfunction!(python::count::count_differences, m)?)?;

    Ok(())
}


// // pub fn add(left: usize, right: usize) -> usize {
// //     left + right
// // }

// // #[cfg(test)]
// // mod tests {
// //     use super::*;

// //     #[test]
// //     fn it_works() {
// //         let result = add(2, 2);
// //         assert_eq!(result, 4);
// //     }
// // }
