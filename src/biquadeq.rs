#![allow(clippy::wrong_self_convention)]
#![allow(clippy::new_without_default)]

use pyo3::prelude::*;
use super::{filtertype::{FilterType, BiquadFilterType}, coeffstruct::BiquadCoeffs};

struct DesignBiquadFilter {
    mode: FilterType,
    filt_coeffs: BiquadCoeffs,
    theta_sine: f64,
    theta_cosine: f64,
    alpha: f64,
    a: Option<f64>,
    beta: Option<f64>,
}

impl DesignBiquadFilter {
    fn new(mode: FilterType, fc: f64, fs: f64, q: f64, dbgain: Option<f64>) -> Self {
        const TWOPI: f64 = 2.0 * std::f64::consts::PI;
        let filt_coeffs = BiquadCoeffs::new();
        let w = TWOPI * fc / fs;
        let theta_sine = w.sin();
        let theta_cosine = w.cos();
        
        let alpha = theta_sine / (2.0 * q);

        // manual implementation of Option.map
        // self.a = match dbgain {            
        //     Some(dbvalue) => Some((10_f64).powf(dbvalue / 40_f64)),
        //     None => None
        // };

        let a = dbgain.map(|db_value| 10_f64.powf(db_value / 40_f64));
        let beta = a.map(|a_value| a_value.sqrt() / q);

        Self {
            mode,
            filt_coeffs,
            theta_sine,
            theta_cosine,
            alpha,
            a,
            beta
        }
    }

    fn coeffs(&mut self) {
        match self.mode {
            FilterType::BiquadType(BiquadFilterType::Lp) => {
                let c = 1.0 - self.theta_cosine;
                let b0: f64 = c / 2.0;
                let b1: f64 = c;
                let b2: f64 = b0;
                
                let a0: f64 = 1.0 + self.alpha;
                let a1: f64 = -2.0 * self.theta_cosine;
                let a2: f64 = 1.0 - self.alpha;

                self.filt_coeffs.set_coeffs((b0, b1, b2, a0, a1, a2))
            },
            FilterType::BiquadType(BiquadFilterType::Hp) => {
                let c = 1.0 + self.theta_cosine;
                let b0: f64 = c / 2.0;
                let b1: f64 = -c;
                let b2: f64 = b0;
                
                let a0: f64 = 1.0 + self.alpha;
                let a1: f64 = -2.0 * self.theta_cosine;
                let a2: f64 = 1.0 - self.alpha;

                self.filt_coeffs.set_coeffs((b0, b1, b2, a0, a1, a2))
            },
            FilterType::BiquadType(BiquadFilterType::Bp0dB) => {
                let b0: f64 = self.alpha;
                let b1: f64 = 0.0;
                let b2: f64 = -b0;
                
                let a0: f64 = 1.0 + self.alpha;
                let a1: f64 = -2.0 * self.theta_cosine;
                let a2: f64 = 1.0 - self.alpha;

                self.filt_coeffs.set_coeffs((b0, b1, b2, a0, a1, a2))
            },
            FilterType::BiquadType(BiquadFilterType::Bpsg) => {
                let b0: f64 = self.theta_sine / 2.0;
                let b1: f64 = 0.0;
                let b2: f64 = -b0;
                
                let a0: f64 = 1.0 + self.alpha;
                let a1: f64 = -2.0 * self.theta_cosine;
                let a2: f64 = 1.0 - self.alpha;

                self.filt_coeffs.set_coeffs((b0, b1, b2, a0, a1, a2))
            },
            FilterType::BiquadType(BiquadFilterType::Notch) => {
                let b0: f64 = 1.0;
                let b1: f64 = -2.0 * self.theta_cosine;
                let b2: f64 = b0;
                
                let a0: f64 = 1.0 + self.alpha;
                let a1: f64 = -2.0 * self.theta_cosine;
                let a2: f64 = 1.0 - self.alpha;

                self.filt_coeffs.set_coeffs((b0, b1, b2, a0, a1, a2))
            },
            FilterType::BiquadType(BiquadFilterType::Ap) => {
                let b0: f64 = 1.0 - self.alpha;
                let b1: f64 = -2.0 * self.theta_cosine;
                let b2: f64 = 1.0 + self.alpha;
                
                let a0: f64 = 1.0 + self.alpha;
                let a1: f64 = -2.0 * self.theta_cosine;
                let a2: f64 = 1.0 - self.alpha;

                self.filt_coeffs.set_coeffs((b0, b1, b2, a0, a1, a2))
            },
            FilterType::BiquadType(BiquadFilterType::Peq) => {
                match self.a {
                    Some(a_value) => { 
                        let b0: f64 = 1.0 + self.alpha * a_value;
                        let b1: f64 = -2.0 * self.theta_cosine;
                        let b2: f64 = 1.0 - self.alpha * a_value;
                        
                        let a0: f64 = 1.0 + self.alpha / a_value;
                        let a1: f64 = -2.0 * self.theta_cosine;
                        let a2: f64 = 1.0 - self.alpha / a_value;

                        self.filt_coeffs.set_coeffs((b0, b1, b2, a0, a1, a2))

                    },
                    None => {
                        self.filt_coeffs.set_coeffs((0.0, 0.0, 0.0, 0.0, 0.0, 0.0))
                    }
                }
            },
            FilterType::BiquadType(BiquadFilterType::LpShelf) => {
                match (self.a, self.beta) {
                    (Some(a_value), Some(beta_value)) => {
                        let b0: f64 = a_value * ((a_value + 1.0) - (a_value - 1.0) * self.theta_cosine + beta_value * self.theta_sine);
                        let b1: f64 = 2.0 * a_value * ((a_value - 1.0) - (a_value + 1.0) * self.theta_cosine);
                        let b2: f64 = a_value * ((a_value + 1.0) - (a_value - 1.0) * self.theta_cosine - beta_value * self.theta_sine);
                        
                        let a0: f64 = (a_value + 1.0) + (a_value - 1.0) * self.theta_cosine + beta_value * self.theta_sine;
                        let a1: f64 = -2.0 * ((a_value - 1.0) + (a_value + 1.0) * self.theta_cosine);
                        let a2: f64 = (a_value + 1.0) + (a_value - 1.0) * self.theta_cosine - beta_value * self.theta_sine;

                        self.filt_coeffs.set_coeffs((b0, b1, b2, a0, a1, a2))

                    },
                    (None, Some(_)) | (Some(_), None) | (None, None) => {
                        self.filt_coeffs.set_coeffs((0.0, 0.0, 0.0, 0.0, 0.0, 0.0))
                    }
                }
            },
            FilterType::BiquadType(BiquadFilterType::HpShelf) => {
                match (self.a, self.beta) {
                    (Some(a_value), Some(beta_value)) => {
                        let b0: f64 = a_value * ((a_value + 1.0) + (a_value - 1.0) * self.theta_cosine + beta_value * self.theta_sine);
                        let b1: f64 = -2.0 * a_value * ((a_value - 1.0) + (a_value + 1.0) * self.theta_cosine);
                        let b2: f64 = a_value * ((a_value + 1.0) + (a_value - 1.0) * self.theta_cosine - beta_value * self.theta_sine);
                        
                        let a0: f64 = (a_value + 1.0) - (a_value - 1.0) * self.theta_cosine + beta_value * self.theta_sine;
                        let a1: f64 = 2.0 * ((a_value - 1.0) - (a_value + 1.0) * self.theta_cosine);
                        let a2: f64 = (a_value + 1.0) - (a_value - 1.0) * self.theta_cosine - beta_value * self.theta_sine;

                        self.filt_coeffs.set_coeffs((b0, b1, b2, a0, a1, a2))

                    },
                    (None, Some(_)) | (Some(_), None) | (None, None) => {
                        self.filt_coeffs.set_coeffs((0.0, 0.0, 0.0, 0.0, 0.0, 0.0))
                    }
                }
            },
            _ => {}
        }
        
    }

}

fn _filt_sample(x: &f64, coeffs: &(f64, f64, f64, f64, f64, f64), x1: f64, x2: f64, y1: f64, y2: f64) -> (f64, f64, f64) {

    let y: f64 = (

        coeffs.0 * x + 
        coeffs.1 * x1 + 
        coeffs.2 * x2 - 
        coeffs.4 * y1 -
        coeffs.5 * y2

    ) / coeffs.3;

    (y, x1, y1)
}

#[pyclass]
pub struct Biquad {
    fs: f64,
    x1: f64,
    x2: f64,
    y1: f64,
    y2: f64
}

#[pymethods]
impl Biquad {
    
    #[new]
    
    ///
    /// INIT BIQUAD CLASS
    /// 
    /// Args
    /// ----
    ///     fs: f64
    ///         sampling rate
    /// 
    
    #[pyo3(text_signature = "(fs: float) -> None")]
    pub fn new(fs: f64) -> Self {
        Self { fs, x1: 0.0, x2: 0.0, y1: 0.0, y2: 0.0 }
    }
    
    ///
    /// GENERATE BIQUAD FILTER COEFFICIENTS
    ///
    /// Args
    /// ----
    ///     mode: &str
    ///         filter type:
    ///             lp = low pass
    ///             hp = high pass
    ///             bp0b = band pass peak = 0dB
    ///             bpsg = band pass skirt ghain peak = Q
    ///             notch = notch filter
    ///             peq = peaking EQ
    ///             ap = all pass filter
    ///             lps = low pass shelf
    ///             hps = high pass shelf
    ///     fc: f64
    ///         corner/cutoff frequency in Hz
    ///     q: f64
    ///         Q factor
    ///     dbgain: Optional<f64>
    ///         dB value for peaking and shelf filters
    /// 
    /// Return
    /// ------
    ///     tuple -> (f64, f64, f64, f64, f64, f64):
    ///         filter coefficients (b0, b1, b2, a0, a1, a2)
    ///         

    #[pyo3(text_signature = "(mode: str, fc: float, q: float, dbgain: float|None) -> tuple[float, float, float, float, float, float]")]
    pub fn design_filter(&self, mode: &str, fc: f64, q: f64, dbgain: Option<f64>) -> (f64, f64, f64, f64, f64, f64) {

        let filt_type: FilterType = match mode {
            "lp" => FilterType::BiquadType(BiquadFilterType::Lp),
            "hp" => FilterType::BiquadType(BiquadFilterType::Hp),
            "bp0db" => FilterType::BiquadType(BiquadFilterType::Bp0dB),
            "bpsg" => FilterType::BiquadType(BiquadFilterType::Bpsg),
            "notch" => FilterType::BiquadType(BiquadFilterType::Notch),
            "ap" => FilterType::BiquadType(BiquadFilterType::Ap),
            "peq" => FilterType::BiquadType(BiquadFilterType::Peq),
            "lps" => FilterType::BiquadType(BiquadFilterType::LpShelf),
            "hps" => FilterType::BiquadType(BiquadFilterType::HpShelf),
            _ => {
                println!("[ERROR] Filt mode not allowed!");
                std::process::exit(1)
            }
        };
    
        let mut design_filter: DesignBiquadFilter = DesignBiquadFilter::new(filt_type, fc, self.fs, q, dbgain);
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

        let (y, x2, y2) = _filt_sample(&sample, &coeffs, self.x1, self.x2, self.y1, self.y2);

        self.x1 = sample;
        self.x2 = x2;
        self.y1 = y;
        self.y2 = y2;

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
        self.x1 = 0.0;
        self.x2 = 0.0;
        self.y1 = 0.0;
        self.y2 = 0.0;
        println!("[DONE] cache cleared!")
    }


}