#![allow(clippy::wrong_self_convention)]
#![allow(clippy::new_without_default)]

use pyo3::prelude::*;
use super::{filtertype::{FilterType, NarrowFilterType}, coeffstruct::BiquadCoeffs, delayline::DelayLine};

struct DesignNarrowFilter {
    mode: FilterType,
    filt_coeffs: BiquadCoeffs,
    theta_cosine: f64,
    k: f64,
    r: f64
}

impl DesignNarrowFilter {
    fn new(mode: FilterType, fc: f64, fs: f64, bw: f64) -> Self {
        const TWOPI: f64 = 2.0 * std::f64::consts::PI;
        let filt_coeffs = BiquadCoeffs::new();
        let w = TWOPI * fc / fs;
        let theta_cosine = w.cos();
        let r = 1.0 - 3.0 * bw / fs;
        let k = (1.0 - 2.0 * r * theta_cosine + r.powf(2.0)) / (2.0 - 2.0 * theta_cosine);

        Self {
            mode,
            filt_coeffs,
            theta_cosine,
            k,
            r
        }
    }

    fn coeffs(&mut self) {
        match self.mode {
            FilterType::NarrowType(NarrowFilterType::Bp) => {
                let b0: f64 = 1.0 - self.k;
                let b1: f64 = 2.0 * (self.k - self.r) * self.theta_cosine;
                let b2: f64 = self.r.powf(2.0) - self.k;
                
                let a1: f64 = 2.0 * self.r * self.theta_cosine;
                let a2: f64 = -self.r.powf(2.0);

                self.filt_coeffs.set_coeffs((b0, b1, b2, 1.0, a1, a2))
            },
            FilterType::NarrowType(NarrowFilterType::Notch) => {
                let b0: f64 = self.k;
                let b1: f64 = -2.0 * self.k * self.theta_cosine;
                let b2: f64 = self.k;
                
                let a1: f64 = 2.0 * self.r * self.theta_cosine;
                let a2: f64 = -self.r.powf(2.0);

                self.filt_coeffs.set_coeffs((b0, b1, b2, 1.0, a1, a2))
            },
            _ => {}
        }
    }

}

#[pyclass]
pub struct Narrow {
    fs: f64,
    x1: DelayLine,
    x2: DelayLine,
    y1: DelayLine,
    y2: DelayLine,
    order: usize,
}

#[pymethods]
impl Narrow {
    
    #[new]

    ///
    /// INIT NARROW CLASS
    /// 
    /// Args
    /// ----
    ///     fs: f64
    ///         sampling rate
    ///     order: usize
    ///         filter order
    /// 
    
    #[pyo3(text_signature = "(fs: float, order: int = 1) -> None")]
    pub fn new(fs: f64, order: Option<usize>) -> Self {
        
        let filt_order = match order {
            Some(order_value) => { order_value },
            None => { 1 }
        };

        Self { 
            fs, 
            x1: DelayLine::new(filt_order), 
            x2: DelayLine::new(filt_order), 
            y1: DelayLine::new(filt_order), 
            y2: DelayLine::new(filt_order),
            order: filt_order,
        }
    }
    
    ///
    /// GENERATE BIQUAD FILTER COEFFICIENTS
    ///
    /// Args
    /// ----
    ///     mode: &str
    ///         filter type:
    ///             bp = Band Pass
    ///             notch = Notch filter
    ///     fc: f64
    ///         corner/cutoff frequency in Hz
    ///     bw: f64
    ///         band width in Hz
    ///     dbgain: Optional<f64>
    ///         dB value for peaking and shelf filters
    /// 
    /// Return
    /// ------
    ///     tuple -> (f64, f64, f64, f64, f64, f64):
    ///         filter coefficients (b0, b1, b2, a0, a1, a2)
    ///         

    #[pyo3(text_signature = "(mode: str, fc: float, bw: float) -> tuple[float, float, float, float, float, float]")]
    pub fn design_filter(&self, mode: &str, fc: f64, bw: f64) -> (f64, f64, f64, f64, f64, f64) {

        let filt_type: FilterType = match mode {
            "bp" => FilterType::NarrowType(NarrowFilterType::Bp),
            "notch" => FilterType::NarrowType(NarrowFilterType::Notch),
            _ => {
                println!("[ERROR] Filt mode not allowed!");
                std::process::exit(1)
            }
        };
    
        let mut design_filter: DesignNarrowFilter = DesignNarrowFilter::new(filt_type, fc, self.fs, bw);
        design_filter.coeffs();
        let coeffs = design_filter.filt_coeffs;
        
        let b0 = coeffs.b0;
        let b1 = coeffs.b1;
        let b2 = coeffs.b2;
        let a0 = coeffs.a0;
        let a1 = coeffs.a1;
        let a2 = coeffs.a2;
    
        (b0, b1, b2, a0, a1, a2)
    
    }

    ///
    /// APPLY FILTER SAMPLE BY SAMPLE
    ///
    /// Args
    /// ----
    ///     sample: f64
    ///         input sample
    ///     coeffs: tuple(f64, f64, f64, f64, f64, f64)
    ///         filter coefficients (b0, b1, b2, a0, a1, a2)
    ///
    /// Return
    /// ------
    ///     f64
    ///         filtered sample
    ///
    ///
    
    #[pyo3(text_signature = "(sample: float, coeffs: tuple[float, float, float, float, float, float]) -> float")]
    pub fn filt_sample(&mut self, sample: f64, coeffs: (f64, f64, f64, f64, f64, f64)) -> f64 {

        let mut x = sample;
        let mut y: f64 = 0.0;
        for _ in 0..self.order {
            y = coeffs.0 * x + coeffs.1 * self.x1.read() + coeffs.2 * self.x2.read() + coeffs.4 * self.y1.read() + coeffs.5 * self.y2.read();

            self.x1.write_and_advance(&x);
            self.x2.write_and_advance(&x);
            self.y1.write_and_advance(&y);
            self.y2.write_and_advance(&y);

            x = y;

        }

        y
    }

    ///
    /// APPLY FILTER ON FRAME OR SIGNAL
    ///
    /// Args
    /// ----
    ///     sample: f64
    ///         input sample
    ///     coeffs: tuple(f64, f64, f64, f64, f64, f64)
    ///         filter coefficients (b0, b1, b2, a0, a1, a2)
    ///
    /// Return
    /// ------
    ///     Vec<f64>
    ///         filtered frame
    ///
    ///

    #[pyo3(text_signature = "(frame: list[float], coeffs: tuple[float, float, float, float, float, float]) -> list[float]")]
    pub fn filt_frame(&mut self, frame: Vec<f64>, coeffs: (f64, f64, f64, f64, f64, f64)) -> Vec<f64> {
        
        let y: Vec<f64> = frame
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