#![allow(clippy::wrong_self_convention)]
#![allow(clippy::new_without_default)]

use super::{filtertype::{FilterType, OnePoleFilterType}, onepole::DesignOnePoleFilter, coeffstruct::OnePoleCoeffs, delayline::DelayLine};
use pyo3::prelude::*;

fn _filt_sample_lowpass(x: &f64, coeffs: &(f64, f64), y1: f64) -> f64 {
    coeffs.0 * x + coeffs.1 * y1
}

fn _filt_sample_comb(mode: &str, x: &f64, g: &f64, _sample: f64) -> f64 {
    match mode {
        "fir" => x + g * _sample,
        "fir_freev" => -x + (1.0 + g) * _sample,
        "iir" => x - g * _sample,
        _ => {
            println!("[ERROR] Filt mode not allowed!");
            std::process::exit(1)
        }
    }
}

fn _filt_sample_comb_lp(x: &f64, g: &f64, &lp_coeffs: &(f64, f64), x_lp: f64, y_lp: f64) -> (f64, f64) {
    let y_low_pass = _filt_sample_lowpass(&x_lp, &lp_coeffs, y_lp);
    let y = x - g * y_low_pass;
    (y, y_low_pass)
}

fn _filt_sample_allpass(mode: &str, x: &f64, g: &f64, x1: f64, y1: f64) -> f64 {
    match mode {
        "freev" => -x + (1.0 + g) * x1 - g * y1,
        "naive" => g * x + x1 - g * y1,
        _ => {
            println!("[ERROR] Filt mode not allowed!");
            std::process::exit(1)
        }
    }
}

fn _filt_sample_allpass_lp(x: &f64, g: &f64, &lp_coeffs: &(f64, f64), x1: f64, x_lp: f64, y_lp: f64) -> (f64, f64) {
    let y_low_pass = _filt_sample_lowpass(&x_lp, &lp_coeffs, y_lp);
    let y = g * x + x1 - g * y_low_pass;
    (y, y_low_pass)
}

#[pyclass]
pub struct Harmonic {
    fs: f64,
    buffer_delay: usize,
    mode: String,
    g: f64,
    x: DelayLine,
    y: DelayLine,
    ylp: DelayLine,
    low_pass_coeffs: OnePoleCoeffs
}

#[pymethods]
impl Harmonic {
    #[new]

    ///
    /// INIT HARMONIC FILTER
    ///
    /// Args
    /// ----
    ///     mode: &str
    ///         filter type:
    ///             combf = forward comb filter
    ///             combfreev = freeverb forward comb filter 
    ///             combi = feedback comb filter
    ///             lpcombi = feedback low pass comb filter
    ///             allpass = all pass filter
    ///             allpassfreev = freeverb allpass filter
    ///             lpallpass = low pass allpass filter
    ///     buffer_delay: f64
    ///         delay length in samples
    ///     fs: f64
    ///         sampling rate
    ///
    
    #[pyo3(text_signature = "(mode: str, buffer_delay: int, fs: float) -> None")]
    pub fn new(mode: &str, buffer_delay: usize, fs: f64) -> Self {
        let mode = String::from(mode);
        let g = 0.0;
        let low_pass_coeffs = OnePoleCoeffs::new();

        Self {
            fs,
            buffer_delay,
            mode,
            g,
            x: DelayLine::new(buffer_delay),
            y: DelayLine::new(buffer_delay),
            ylp: DelayLine::new(1),
            low_pass_coeffs
        }
    }

    ///
    /// GENERATE HARMONIC FILTER
    ///
    /// Args
    /// ----
    ///     t60: f64
    ///         reverb time in sec.
    ///     fc: Optional<f64>
    ///         low pass cut off frequency (optional, only for lpcombi and lpallpass)
    ///

    #[pyo3(text_signature = "(t60: float, fc: float|None) -> None")]
    pub fn design_filter(&mut self, t60: f64, fc: Option<f64>) {
        let d_time: f64 = (self.buffer_delay as f64) / self.fs;
        self.g = 10.0_f64.powf(-3.0 * d_time / t60);

        let (b0, a1) = match fc {
            Some(cutoff) => {
                let mut lowpass_coeffs = DesignOnePoleFilter::new(FilterType::OnePoleType(OnePoleFilterType::LowPass), cutoff, self.fs);
                lowpass_coeffs.coeffs();
                (lowpass_coeffs.filt_coeffs.b0, lowpass_coeffs.filt_coeffs.a1)
            },
            None => { (0.0, 0.0) }             
        };
        self.low_pass_coeffs.set_coeffs((b0, 0.0, a1));
    }

    ///
    /// APPLY FILTER SAMPLE BY SAMPLE
    ///
    /// Args
    /// ----
    ///     sample: f64
    /// 
    /// Return
    /// ------
    ///     f64
    ///         filtered sample
    ///
    ///

    #[pyo3(text_signature = "(sample: float) -> float")]
    pub fn filt_sample(&mut self, sample: f64) -> f64 {
        
        let lp_coeffs = (self.low_pass_coeffs.b0, self.low_pass_coeffs.a1);
        
        let (yout, ylpass) = match &self.mode[..] {
            "combf" => { (_filt_sample_comb("fir", &sample, &self.g, self.x.read()), 0.0) },
            "combfreev" => { (_filt_sample_comb("fir_freev", &sample, &self.g, self.x.read()), 0.0) },
            "combi" => { (_filt_sample_comb("iir", &sample, &self.g, self.y.read()), 0.0) },
            "allpass" => { (_filt_sample_allpass("naive", &sample, &self.g, self.x.read(), self.y.read()), 0.0) },
            "allpassfreev" => { (_filt_sample_allpass("freev", &sample, &self.g, self.x.read(), self.y.read()), 0.0) },
            "lpcombi" => {
                let (y_out, y_out_lp) = _filt_sample_comb_lp(&sample, &self.g, &lp_coeffs, self.y.read(), self.ylp.read());
                (y_out, y_out_lp)
            },
            "lpallpass" => {
                let (y_out, y_out_lp) = _filt_sample_allpass_lp(&sample, &self.g, &lp_coeffs, self.x.read(), self.y.read(), self.ylp.read());
                (y_out, y_out_lp)
            },
            _ => {
                println!("[ERROR] filt mode not allowed!");
                std::process::exit(1);
            }
        };
        
        self.ylp.write_and_advance(&ylpass);

        self.x.write_and_advance(&sample);
        self.y.write_and_advance(&yout);

        yout

    }

    ///
    /// APPLY FILTER ON FRAME OR SIGNAL
    ///
    /// Args
    /// ----
    ///     sample: f64
    ///         input sample
    ///
    /// Return
    /// ------
    ///     Vec<f64>
    ///         filtered frame
    ///
    ///

    #[pyo3(text_signature = "(frame: list[float]) -> list[float]")]
    pub fn filt_frame(&mut self, frame: Vec<f64>) -> Vec<f64> {
        
        let y = frame
            .iter()
            .map(|&x| self.filt_sample(x))
            .collect();

        y
    }

    ///
    /// CLEAR DELAYED SAMPLES CACHE
    /// set buffer and delayed low pass sample to zero
    ///

    pub fn clear_delayed_samples_cache(&mut self) {
        self.x.clear();
        self.y.clear();
        self.ylp.clear();
        println!("[DONE] cache cleared!")
    }
}