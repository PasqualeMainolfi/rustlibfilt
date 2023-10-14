#![allow(clippy::wrong_self_convention)]
#![allow(clippy::new_without_default)]

use super::{filtertype::OnePoleFilterType, coeffstruct::OnePoleCoeffs};
use pyo3::prelude::*;

pub struct DesignOnePoleFilter {
    mode: OnePoleFilterType,
    pub filt_coeffs: OnePoleCoeffs,
    alpha: f64
}

impl DesignOnePoleFilter {
    pub fn new(mode: OnePoleFilterType, fc: f64, fs: f64) -> Self {
        let filt_coeffs = OnePoleCoeffs::new();
        let twopi = 2.0 * std::f64::consts::PI;
        let w = twopi * fc / fs;
        let alpha = (-2.0 * w).exp();

        Self {
            mode,
            filt_coeffs,
            alpha
        }
    }

    pub fn coeffs(&mut self) {
        match self.mode {
            OnePoleFilterType::LowPass => {
                let b0 = 1.0 - self.alpha;
                let a1 = self.alpha;
                self.filt_coeffs.set_coeffs((b0, 0.0, a1));
            },
            OnePoleFilterType::HighPass => {
                let b0 = (1.0 + self.alpha) / 2.0;
                let b1 = -b0;
                let a1 = self.alpha;
                self.filt_coeffs.set_coeffs((b0, b1, a1));
            }
        }
    } 

}

fn _filt_sample(x: &f64, coeffs: &(f64, f64, f64), x1: f64, y1: f64) -> f64 {
    coeffs.0 * x + coeffs.1 * x1 + coeffs.2 * y1
}

#[pyclass]
pub struct OnePole {
    fs: f64,
    x: f64,
    y: f64
}

#[pymethods]
impl OnePole {
    #[new]

    ///
    /// INIT ONEPOLE CLASS
    ///
    /// Args
    /// ----
    ///     fs: f64
    ///         sampling rate
    ///
    
    #[pyo3(text_signature = "(fs: float) -> None")]
    pub fn new(fs: f64) -> Self {
        Self { fs, x: 0.0, y: 0.0 }
    }

    ///
    /// GENERATE ONE POLE FILTER COEFFICIENTS
    ///
    /// Args
    /// ----
    ///     mode: &str
    ///         filter type:
    ///             lp = low pass
    ///             hp = high pass
    ///     fc: f64
    ///         corner/cutoff frequency in Hz
    ///     fs: f64
    ///         sampling rate in Hz
    /// 
    /// Return
    /// ------
    ///     tuple -> (f64, f64):
    ///         filter coefficients (b0, a1)
    /// 

    #[pyo3(text_signature = "(mode: str, fc: float) -> tuple[float, float, float]")]
    pub fn design_filter(&mut self, mode: &str, fc: f64) -> (f64, f64, f64) {

        let filt_type: OnePoleFilterType = match mode {
            "lp" => OnePoleFilterType::LowPass,
            "hp" => OnePoleFilterType::HighPass,
            _ => {
                println!("[ERROR] Filt mode not allowed!");
                std::process::exit(1)
            }          
        };

        let mut design_filter = DesignOnePoleFilter::new(filt_type, fc, self.fs);
        design_filter.coeffs();

        (design_filter.filt_coeffs.b0, design_filter.filt_coeffs.b1, design_filter.filt_coeffs.a1)

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

    #[pyo3(text_signature = "(sample: float, coeffs: tuple[float, float, float]) -> float")]
    pub fn filt_sample(&mut self, sample: f64, coeffs: (f64, f64, f64)) -> f64 {

        let y = _filt_sample(&sample, &coeffs, self.x, self.y);
        self.x = sample;
        self.y = y;
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

    #[pyo3(text_signature = "(frame: liat[float], coeffs: tuple[float, float, float]) -> list[float]")]
    pub fn filt_frame(&mut self, frame: Vec<f64>, coeffs: (f64, f64, f64)) -> Vec<f64> {

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
        self.y = 0.0;
        println!("[DONE] cache cleared!")
    }



}