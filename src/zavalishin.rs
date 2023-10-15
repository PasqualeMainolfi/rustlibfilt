#![allow(clippy::wrong_self_convention)]
#![allow(clippy::new_without_default)]

use pyo3::prelude::*;
use super::filtertype::ZavalishinFilterType;

fn filt_sample(sample: &f64, g: f64, _z: f64) -> (f64, f64, f64) {
    let v = (sample - _z) * g;
    let lp = v + _z;
    let hp = sample - lp;
    let z = lp + v;
    (lp, hp, z)
}

fn filt_sample_svf(sample: &f64, coeffs: (f64, f64, f64), _z: f64, _s: f64) -> (f64, f64, f64, f64) {
    let hp = (sample - 2.0 * coeffs.2 * _z - coeffs.0 * _z - _s) / coeffs.1;
    let bp = coeffs.0 * hp + _z;
    let lp = coeffs.0 * bp + _s;
    let br = sample - 2.0 * coeffs.2 * bp;
    (lp, hp, bp, br)
}


#[pyclass]
pub struct Zavalishin {
    fs: f64,
    g: f64,
    g1: f64,
    r: f64,
    z_sample: f64,
    s_sample: f64,
    filt_type: Option<ZavalishinFilterType>
}

#[pymethods]
impl Zavalishin {
    #[new]

    ///
    /// INIT ZAVALISHIN CLASS
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
            g: 0.0, 
            g1: 0.0, 
            r: 0.0, 
            z_sample: 0.0, 
            s_sample: 0.0, 
            filt_type: None

        }
    }
    
    ///
    /// DESIGN FILTER
    ///
    /// Args
    /// ----
    ///     mode: &str
    ///         filt mode:
    ///             - zdf = zero delay feedback one pole filter (generate low pass, high pass, allpass)
    ///             - naive = naive one pole filter (generate low pass, high pass)
    ///             - trap = trapezoidal integration (generate low pass, high pass)
    ///             - svf = state variable (generate low pass, high pass, band pass, band reject)
    ///     fc: f64
    ///         cut-off frequency in Hz
    ///     fc_spread: Option<f64>
    ///         frequency spread in Hz (fc + fc_spread = fhigh for SVF)
    ///
    
    #[pyo3(text_signature = "(fc: float, fc_spread: float|None) -> None")]
    pub fn design_filter(&mut self, mode: &str, fc: f64, fc_spread: Option<f64>) {
        let twopi = 2.0 * std::f64::consts::PI;
        let wc = twopi * fc;
        let ts = 1.0 / self.fs;
        let mut wa = (2.0 / ts) * (wc * ts / 2.0).tan();
        match mode {
            "zdf" => { 
                self.filt_type = Some(ZavalishinFilterType::OnePoleZeroDelay);
                self.g = wa * ts / 2.0
            },
            "naive" => { 
                self.filt_type = Some(ZavalishinFilterType::NaiveOnePole);
                self.g = wa * ts
            },
            "trap" => { 
                self.filt_type = Some(ZavalishinFilterType::TrapIntOnePole);
                self.g = wa * ts / 2.0
            },
            "svf" => { 
                match fc_spread {
                    Some(spread) => {
                        self.filt_type = Some(ZavalishinFilterType::StateVariable);
                        let w = twopi * (fc + spread);
                        let w_sqrt = (wc * w).sqrt();
                        self.r = ((wc + w) / 2.0) / w_sqrt;
                        wa = (2.0 / ts) * (w_sqrt * ts / 2.0).tan();
                        self.g = wa * ts / 2.0;
                        self.g1 = 1.0 + (2.0 * self.r * self.g) + self.g.powf(2.0)
                    },
                    None => {
                        println!("[ERROR] In SVF mode you habe to specify fc_spread!");
                        std::process::exit(1)
                    }
                    
                }
            },
            _ => {
                println!("[ERROR] Filter mode not allowed!");
                std::process::exit(1)
            }
        }
    }
    
    ///
    /// APPLY FILTER SAMPLE BY SAMPLE
    ///
    /// Args
    /// ----
    ///     sample: f64
    ///         sample in
    ///
    /// Return
    /// ------
    ///     (f64, f64, f64, f64, f64, f64) -> (low_pass, high_pass, allpass, band pass, band reject)
    ///     
    ///
    
    #[pyo3(text_signature = "(sample: float) -> tuple[float, float, float, float, float]")]
    pub fn filt_sample(&mut self, sample: f64) -> (f64, f64, f64, f64, f64) {
        let (lp, hp, ap, bp, br, z, s) = match &self.filt_type {
            Some(t) => { match t {
                ZavalishinFilterType::OnePoleZeroDelay => {
                    let (_lp, _hp, _z) = filt_sample(&sample, self.g, self.z_sample);
                    let _ap = _lp - _hp;
                    (_lp, _hp, _ap, 0.0, 0.0, _z, 0.0)
                },
                ZavalishinFilterType::NaiveOnePole => {
                    let (_lp, _hp, _z) = filt_sample(&sample, self.g, self.z_sample);
                    (_lp, _hp / 2.0, 0.0, 0.0, 0.0, _lp, 0.0)
                },
                ZavalishinFilterType::TrapIntOnePole => {
                    let (_lp, _hp, _z) = filt_sample(&sample, self.g, self.z_sample);
                    (_lp, _hp, 0.0, 0.0, 0.0, _z, 0.0)
                },
                ZavalishinFilterType::StateVariable => {
                    let (_lp, _hp, _bp, _br) = filt_sample_svf(&sample, (self.g, self.g1, self.r), self.z_sample, self.s_sample);
                    let _z = self.g * _hp + _bp;
                    let _s = self.g * _bp + _lp;
                    (_lp, _hp, 0.0, _bp, _br, _z, _s)
                    
                },
            }
        },
            None => (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0)
        };
        self.z_sample = z;
        self.s_sample = s;
        (lp, hp, ap, bp, br)
    }

    ///
    /// CLEAR DELAYED SAMPLES CACHE
    /// set:
    ///     z_sample = 0.0
    ///     s_sample = 0.0
    ///

    pub fn clear_delayed_samples_cache(&mut self) {
        self.z_sample = 0.0;
        self.s_sample = 0.0;
        println!("[DONE] cache cleared!")
    }

}