#![allow(clippy::wrong_self_convention)]
#![allow(clippy::new_without_default)]

use super::{filtertype::{FilterType, TwoZeroTwoPoleFilterType}, coeffstruct::BiquadCoeffs, delayline::DelayLine};
use pyo3::prelude::*;

struct DesignTwoPoleTwoZeroFilter {
    mode: FilterType,
    filt_coeffs: BiquadCoeffs,
    theta_cosine: f64,
    r: f64
}

impl DesignTwoPoleTwoZeroFilter {
    fn new(mode: FilterType, fc: f64, fs: f64, bw: f64) -> Self {

        let pi = std::f64::consts::PI;
        let filt_coeffs = BiquadCoeffs::new();
        let theta_cosine: f64 = (2.0 * pi * fc / fs).cos();
        let r = (-pi * bw / fs).exp();

        Self {
            mode,
            filt_coeffs,
            theta_cosine,
            r
        }
    }

    fn coeffs(&mut self) {
        match self.mode {
            FilterType::TwoZeroTwoPoleType(TwoZeroTwoPoleFilterType::Notch) => {
                let b0 = 1.0;
                let b1 = -2.0 * self.r * self.theta_cosine;
                let b2 = self.r * self.r;

                self.filt_coeffs.set_coeffs((b0, b1, b2, 0.0, 0.0, 0.0))

            },
            FilterType::TwoZeroTwoPoleType(TwoZeroTwoPoleFilterType::Bp) => {
                let b0 = 1.0;
                let a1 = -2.0 * self.r * self.theta_cosine;
                let a2 = self.r * self.r;

                self.filt_coeffs.set_coeffs((b0, 0.0, 0.0, 0.0, a1, a2))

            },
            _ => {}
            
        }
    }
}


#[pyclass]
pub struct TwoZeroTwoPole {
    fs: f64,
    x1: DelayLine,
    x2: DelayLine,
    y1: DelayLine,
    y2: DelayLine
}

#[pymethods]
impl TwoZeroTwoPole {
    #[new]

    ///
    /// INIT TWOZEROTWOPOLE CLASS
    ///
    /// Args
    /// ----
    ///     fs: f64
    ///         sampling rate
    ///
    
    #[pyo3(text_signature = "(fs: float) -> None")]
    pub fn new(fs: f64) -> Self { 
        Self { 
            fs, 
            x1: DelayLine::new(1), 
            x2: DelayLine::new(2), 
            y1: DelayLine::new(1), 
            y2: DelayLine::new(2) 
        }
    }

    ///
    /// GENERATE TWO ZERO/POLE FILTER COEFFICIENTS
    ///
    /// Args
    /// ----
    ///     mode: &str
    ///         filter type:
    ///             notch = two zero (notch)
    ///             bp = two pole (band pass)
    ///     fc: f64
    ///         corner/cutoff frequency in Hz
    ///     fs: f64
    ///         sampling rate in Hz
    ///     bw: f64
    ///         band width in Hz
    /// 
    /// Return
    /// ------
    ///     tuple -> (f64, f64, f64, f64, f64, f64):
    ///         filter coefficients (b0, b1, b2) or (b0, a1, a2)
    ///         

    #[pyo3(text_signature = "(mode: str, fc: float, bw: float) -> tuple[float, float, float]")]
    pub fn design_filter(&mut self, mode: &str, fc: f64, bw: f64) -> (f64, f64, f64, f64, f64, f64) {

        let filt_type = match mode {
            "notch" => FilterType::TwoZeroTwoPoleType(TwoZeroTwoPoleFilterType::Notch),
            "bp" => FilterType::TwoZeroTwoPoleType(TwoZeroTwoPoleFilterType::Bp),
            _ => {
                println!("[ERROR] Filt mode not allowed!");
                std::process::exit(1)
            }
        };

        let mut design_filter = DesignTwoPoleTwoZeroFilter::new(filt_type, fc, self.fs, bw);
        design_filter.coeffs();
        let coeffs = design_filter.filt_coeffs;
        (coeffs.b0, coeffs.b1, coeffs.b2, coeffs.a0, coeffs.a1, coeffs.a2)
    
    }

    ///
    /// APPLY FILTER SAMPLE BY SAMPLE
    ///
    /// Args
    /// ----
    ///     sample: f64
    ///         input sample
    ///     coeffs: tuple(f64, f64, f64)
    ///         filter coefficients (b0, b1, b2) or (bo, a1, a2)
    ///
    /// Return
    /// ------
    ///     f64
    ///         filtered sample
    ///
    ///

    #[pyo3(text_signature = "(sample: float, coeffs: tuple[float, float, float, float, float, float]) -> float")]
    pub fn filt_sample(&mut self, sample: f64, coeffs: (f64, f64, f64, f64, f64, f64)) -> f64 {

        let y = coeffs.0 * sample + coeffs.1 * self.x1.read() + coeffs.2 * self.x2.read() - coeffs.4 * self.y1.read() - coeffs.5 * self.y2.read();

        self.x1.write_and_advance(&sample);
        self.x2.write_and_advance(&sample);
        self.y1.write_and_advance(&y);
        self.y2.write_and_advance(&y);

        y
    }

    ///
    /// APPLY FILTER ON FRAME OR SIGNAL
    ///
    /// Args
    /// ----
    ///     frame: Vec<f64>
    ///         input frame
    ///     coeffs: tuple(f64, f64, f64)
    ///         filter coefficients (b0, b1, b2) or (bo, a1, a2)
    ///
    /// Return
    /// ------
    ///     Vec<f64>
    ///         filtered frame
    ///
    ///

    #[pyo3(text_signature = "(frame: list[float], coeffs: tuple[float, float, float]) -> list[float]")]
    pub fn filt_frame(&mut self, frame: Vec<f64>, coeffs: (f64, f64, f64, f64, f64, f64)) -> Vec<f64> {

        let y = frame
            .iter()
            .map(|&x| self.filt_sample(x, coeffs))
            .collect();

        y

    }

    ///
    /// CLEAR DELAYED SAMPLES CACHE
    /// set:
    ///     x[n - 1] = 0.0
    ///     x[n - 2] = 0.0
    /// or
    ///     y[n - 1] = 0.0
    ///     y[n - 2] = 0.0
    ///

    pub fn clear_delayed_samples_cache(&mut self) {
        self.x1.clear();
        self.x2.clear();
        self.y1.clear();
        self.y2.clear();
        println!("[DONE] cache cleared!")
    }
}