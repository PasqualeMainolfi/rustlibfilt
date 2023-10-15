pub mod biquadeq;
pub mod twozerotwopole;
pub mod filtertype;
pub mod coeffstruct;
pub mod onepole;
pub mod dc;
pub mod harmonic;
pub mod narrow;
pub mod zavalishin;
pub mod butter;

use pyo3::prelude::*;
use biquadeq::Biquad;
use twozerotwopole::TwoZeroTwoPole;
use onepole::OnePole;
use dc::DcFilter;
use harmonic::Harmonic;
use narrow::Narrow;
use zavalishin::Zavalishin;
use butter::Butter;


/// A Python module implemented in Rust.
#[pymodule]
fn rustlibfilt(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Biquad>()?;
    m.add_class::<TwoZeroTwoPole>()?;
    m.add_class::<OnePole>()?;
    m.add_class::<DcFilter>()?;
    m.add_class::<Harmonic>()?;
    m.add_class::<Narrow>()?;
    m.add_class::<Zavalishin>()?;
    m.add_class::<Butter>()?;
    Ok(())
}