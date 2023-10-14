#![allow(clippy::wrong_self_convention)]
#![allow(clippy::new_without_default)]

use super::{filtertype::DcBlockFilterType, coeffstruct::OnePoleCoeffs};
use pyo3::prelude::*;

struct DesignDcFilter {
    mode: DcBlockFilterType,
    filt_coeffs: OnePoleCoeffs,
    r: f64
}

impl DesignDcFilter {
    fn new(mode: DcBlockFilterType, fc: f64, fs: f64) -> Self {
        let filt_coeffs = OnePoleCoeffs::new();
        let twopi = 2.0 * std::f64::consts::PI;
        let w = twopi * fc / fs;
        let r = 1.0 - w;

        Self {
            mode,
            filt_coeffs,
            r
        }
    }

    fn coeffs(&mut self) {
        match self.mode {
            DcBlockFilterType::DcBlockJulius => {
                self.filt_coeffs.set_coeffs((1.0, 0.0, self.r));
            }
        }
    } 

}

fn _filt_sample(x: &f64, coeffs: &(f64, f64), x1: f64, y1: f64) -> f64 {
    coeffs.0 * x - x1 + coeffs.1 * y1
}

#[pyclass]
pub struct DcFilter {
    fs: f64,
    _x: f64,
    _y: f64
}

#[pymethods]
impl DcFilter {
    #[new]

    ///
    /// INIT DCFILTER CLASS
    ///
    /// Args
    /// ----
    ///     fs: f64
    ///         sampling rate 
    ///
    
    pub fn new(fs: f64) -> Self {
        Self { fs, _x: 0.0, _y: 0.0 }
    }

    ///
    /// GENERATE DC FILTER COEFFICIENTS
    ///
    /// Args
    /// ----
    ///     mode: &str
    ///         filter type:
    ///             dcj = dc block from J. O. Smith 
    ///     fc: f64
    ///         corner/cutoff frequency in Hz
    /// 
    /// Return
    /// ------
    ///     tuple -> (f64, f64):
    ///         filter coefficients (b0, a1)
    ///

    #[pyo3(text_signature = "(mode: str, fc: float) -> tuple[float, float]")]
    pub fn design_filter(&mut self, mode: &str, fc: f64) -> (f64, f64) {

        let filt_type: DcBlockFilterType = match mode {
            "dcj" => DcBlockFilterType::DcBlockJulius,
            _ => {
                println!("[ERROR] Filt mode not allowed!");
                std::process::exit(1)
            }          
        };

        let mut design_filter = DesignDcFilter::new(filt_type, fc, self.fs);
        design_filter.coeffs();

        (design_filter.filt_coeffs.b0, design_filter.filt_coeffs.a1)

    }

    ///
    /// APPLY FILTER SAMPLE BY SAMPLE
    ///
    /// Args
    /// ----
    ///     sample: f64
    ///         input sample
    ///     coeffs: tuple(f64, f64)
    ///         filter coefficients (b0, a1)
    ///
    /// Return
    /// ------
    ///     f64
    ///         filtered sample
    ///
    ///

    #[pyo3(text_signature = "(sample: float, coeffs: tuple[float, float]) -> float")]
    pub fn filt_sample(&mut self, sample: f64, coeffs: (f64, f64)) -> f64 {
        let y = _filt_sample(&sample, &coeffs, self._x, self._y);
        self._x = sample;
        self._y = y;
        y
    }

    ///
    /// APPLY FILTER ON FRAME OR SIGNAL
    ///
    /// Args
    /// ----
    ///     frame: Vec<f64>
    ///         input frame
    ///     coeffs: tuple(f64, f64)
    ///         filter coefficients (b0, a1)
    ///
    /// Return
    /// ------
    ///     Vec<f64>
    ///         filtered frame
    ///
    ///

    #[pyo3(text_signature = "(frame: list[float], coeffs: tuple[float, float, float]) -> list[float]")]
    pub fn filt_frame(&mut self, frame: Vec<f64>, coeffs: (f64, f64)) -> Vec<f64> {

        let y = frame
            .iter()
            .map(|&x| self.filt_sample(x, coeffs))
            .collect();

        y

    }

    ///
    /// CLEAR DELAYED SAMPLES CACHE
    /// set:
    ///     y[n - 1] = 0.0
    ///

    pub fn clear_delayed_samples_cache(&mut self) {
        self._x = 0.0;
        self._y = 0.0;
        println!("[DONE] cache cleared!")
    }



}