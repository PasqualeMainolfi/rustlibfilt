#![allow(clippy::wrong_self_convention)]
#![allow(clippy::new_without_default)]

use pyo3::prelude::*;
use super::{coeffstruct::OnePoleCoeffs, filtertype::ButterFilterType};

struct DesignButterFilter {
    filt_coeffs: OnePoleCoeffs,
}

impl DesignButterFilter {
    fn new() -> Self {
        let filt_coeffs = OnePoleCoeffs::new();
        Self {
            filt_coeffs
        }
    }

    fn coeffs(&mut self, mode: ButterFilterType, fc: f64, fs: f64) {
        let twopi = std::f64::consts::PI;
        let wd = twopi * fc;
        let ts = 1.0 / fs;
        let wdts = wd * ts / 2.0; 
        match mode {
            ButterFilterType::Lp => {
                let b0 = 2.0 * wdts.tan() / (2.0 + 2.0 * wdts.tan());
                let b1 = b0;
                let a1 = (2.0 * wdts.tan() - 2.0) / (2.0 + 2.0 * wdts.tan());

                self.filt_coeffs.set_coeffs((b0, b1, a1))
            },
            ButterFilterType::Hp => {
                let b0 = 2.0 / (2.0 + 2.0 * wdts.tan());
                let b1 = -b0;
                let a1 = (2.0 * wdts.tan() - 2.0) / (2.0 + 2.0 * wdts.tan());

                self.filt_coeffs.set_coeffs((b0, b1, a1))
            },
            ButterFilterType::Bp => {},
            ButterFilterType::Notch => {}
        }
    }
}

fn _filt_sample(sample: &f64, coeffs: &[f64], x1: f64, y1: f64) -> f64 {
    coeffs[0] * sample + coeffs[1] * x1 - coeffs[2] * y1
}

#[pyclass]
pub struct Butter {
    mode: String,
    fs: f64,
    xlp: f64,
    ylp: f64,
    x1: f64,
    y1: f64,

}

#[pymethods]
impl Butter {
    #[new]

    ///
    /// INIT BUTTER CLASS
    /// 
    /// Args
    /// ----
    ///     fs: f64
    ///         sampling rate
    ///

    pub fn new(fs: f64) -> Self {
        Self { mode: String::from(""), fs, xlp: 0.0, ylp: 0.0, x1: 0.0, y1: 0.0 }
    }

    ///
    /// GENERATE BUTTERWORTH FILTER COEFFICIENTS
    ///
    /// Args
    /// ----
    ///     mode: &str
    ///         lp = first order butterworth low pass filter IIR
    ///         hp = first order butterworth high pass filter IIR
    ///         bp = band pass from lp + hp
    ///         br = band reject
    ///     fc: f64
    ///         cut off frequency in Hz
    ///
    /// Return
    /// ------
    ///     Vec<f64>
    ///         [b0, b1, a1] or [b0lp, b1lp, a1lp, b0hp, b1hp, a1hp]
    ///
    ///

    pub fn design_filter(&mut self, mode: &str, fc: f64, bw: Option<f64>) -> Vec<f64> {
        
        let filt_type = match mode {
            "lp" => ButterFilterType::Lp,
            "hp" => ButterFilterType::Hp,
            "bp" => ButterFilterType::Bp,
            "br" => ButterFilterType::Notch,
            _ => {
                println!("[ERROR] Filter mode not allowed!");
                std::process::exit(1)
            }
            
        };

        self.mode = String::from(mode);
        let mut design_filter = DesignButterFilter::new();

        let coeffs: Vec<f64> = match bw {
                Some(value) => {
    
                    design_filter.coeffs(ButterFilterType::Lp, fc + value / 2.0, self.fs);
                    let b0lp = design_filter.filt_coeffs.b0; 
                    let b1lp = design_filter.filt_coeffs.b1;
                    let a1lp = design_filter.filt_coeffs.a1;
    
                    design_filter.coeffs(ButterFilterType::Hp, fc - value / 2.0, self.fs);
                    let b0hp = design_filter.filt_coeffs.b0; 
                    let b1hp = design_filter.filt_coeffs.b1;
                    let a1hp = design_filter.filt_coeffs.a1;
                    
                    vec![b0lp, b1lp, a1lp, b0hp, b1hp, a1hp]
    
                },
                None => {
                    design_filter.coeffs(filt_type, fc, self.fs);
                    let b0 = design_filter.filt_coeffs.b0; 
                    let b1 = design_filter.filt_coeffs.b1;
                    let a1 = design_filter.filt_coeffs.a1;
    
                    vec![b0, b1, a1]
                }
    
            };
        coeffs
    }

    ///
    /// APPLY FILTER SAMPLE BY SAMPLE
    ///
    /// Args
    /// ----
    ///     sample: f64
    ///         input sample
    ///     coeffs: Vec<f64>
    ///         filter coefficients from fn design_filter
    ///
    /// Return
    /// ------
    ///     f64
    ///         filtered sample
    ///
    ///

    #[pyo3(text_signature = "(sample: float, coeffs: list[float]) -> float")]
    pub fn filt_sample(&mut self, sample: f64, coeffs: Vec<f64>) -> f64 {

        let (y, xlp, ylp, x1, y1) = if self.mode.eq("bp") || self.mode.eq("br") {
            let _ylp = _filt_sample(&sample, &coeffs[0..2], self.xlp, self.ylp);
            let _y = _filt_sample(&_ylp, &coeffs[3..5], self.x1, self.y1);
            (_y, sample, _ylp, _ylp, _y)
        } else {
            let _y = _filt_sample(&sample, &coeffs, self.x1, self.y1);
            (_y, 0.0, 0.0, sample, _y)
        };

        self.xlp = xlp;
        self.ylp = ylp;
        self.x1 = x1;
        self.y1 = y1;
        
        y
    }

    ///
    /// APPLY FILTER ON FRAME OR SIGNAL
    ///
    /// Args
    /// ----
    ///     frame: Vec<f64>
    ///         input frame
    ///     coeffs: Vec<f64>
    ///         filter coefficients from design_filter
    ///
    /// Return
    /// ------
    ///     Vec<f64>
    ///         filtered frame
    ///
    ///

    #[pyo3(text_signature = "(frame: list[float], coeffs: list[float]) -> list[float]")]
    pub fn filt_frame(&mut self, frame: Vec<f64>, coeffs: Vec<f64>) -> Vec<f64> {

        let y = frame
            .iter()
            .map(|&x| self.filt_sample(x, coeffs.to_vec()))
            .collect();
        y

    }

    ///
    /// CLEAR DELAYED SAMPLES CACHE
    /// set:
    ///     xlp[n - 1] = 0.0
    ///     ylp[n - 1] = 0.0
    ///     x1[n - 1] = 0.0
    ///     y1[n - 1] = 0.0
    ///

    pub fn clear_delayed_samples_cache(&mut self) {
        self.xlp = 0.0;
        self.ylp = 0.0;
        self.x1 = 0.0;
        self.y1 = 0.0;
        println!("[DONE] cache cleared!")
    }
}