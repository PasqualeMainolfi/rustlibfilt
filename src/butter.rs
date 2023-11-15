#![allow(clippy::wrong_self_convention)]
#![allow(clippy::new_without_default)]

use pyo3::prelude::*;
use super::{coeffstruct::OnePoleCoeffs, filtertype::{FilterType, ButterFilterType}, delayline::DelayLine};

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

    fn coeffs(&mut self, mode: FilterType, fc: f64, fs: f64) {
        let twopi = 2.0 * std::f64::consts::PI;
        let wc = twopi * fc;
        let ts = 1.0 / fs;
        let wtan = 2.0 * (wc * ts / 2.0).tan();
        match mode {
            FilterType::ButterType(ButterFilterType::Lp) => {
                let b0 = wtan / (2.0 + wtan);
                let b1 = b0;
                let a1 = (wtan - 2.0) / (2.0 + wtan);

                self.filt_coeffs.set_coeffs((b0, b1, a1))
            },
            FilterType::ButterType(ButterFilterType::Hp) => {
                let b0 = 2.0 / (2.0 + wtan);
                let b1 = -b0;
                let a1 = (wtan - 2.0) / (2.0 + wtan);

                self.filt_coeffs.set_coeffs((b0, b1, a1))
            },
            _ => {}
        }
    }
}


#[pyclass]
pub struct Butter {
    mode: String,
    fs: f64,
    order: usize,
    xtemp: DelayLine,
    ytemp: DelayLine,
    x1: DelayLine,
    y1: DelayLine,
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
            mode: String::from(""), 
            fs, 
            order: filt_order, 
            xtemp: DelayLine::new(filt_order), 
            ytemp: DelayLine::new(filt_order), 
            x1: DelayLine::new(filt_order), 
            y1: DelayLine::new(filt_order),
        }
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
    ///         br = notch from hp + lp
    ///     fc: f64
    ///         cut off frequency in Hz
    ///
    /// Return
    /// ------
    ///     Vec<f64>
    ///         [b0, b1, a1] or [b0lp, b1lp, a1lp, b0hp, b1hp, a1hp] or [b0hp, b1hp, a1hp, b0lp, b1lp, a1lp]
    ///
    ///

    pub fn design_filter(&mut self, mode: &str, fc: f64, bw: Option<f64>) -> Vec<f64> {
        
        let mut design_filter = DesignButterFilter::new();

        let coeffs = match mode {
            "lp" => { 
                design_filter.coeffs(FilterType::ButterType(ButterFilterType::Lp), fc, self.fs);
                let b0 = design_filter.filt_coeffs.b0; 
                let b1 = design_filter.filt_coeffs.b1;
                let a1 = design_filter.filt_coeffs.a1;

                vec![b0, b1, a1]
            },
            "hp" => {
                design_filter.coeffs(FilterType::ButterType(ButterFilterType::Hp), fc, self.fs);
                let b0 = design_filter.filt_coeffs.b0; 
                let b1 = design_filter.filt_coeffs.b1;
                let a1 = design_filter.filt_coeffs.a1;

                vec![b0, b1, a1]
            },
            "bp" => { 
                match bw { 
                    Some(bw_value) => { 
                        design_filter.coeffs(FilterType::ButterType(ButterFilterType::Lp), fc + bw_value / 2.0, self.fs);
                        let b0lp = design_filter.filt_coeffs.b0; 
                        let b1lp = design_filter.filt_coeffs.b1;
                        let a1lp = design_filter.filt_coeffs.a1;
        
                        design_filter.coeffs(FilterType::ButterType(ButterFilterType::Hp), fc - bw_value / 2.0, self.fs);
                        let b0hp = design_filter.filt_coeffs.b0; 
                        let b1hp = design_filter.filt_coeffs.b1;
                        let a1hp = design_filter.filt_coeffs.a1;
                        
                        vec![b0lp, b1lp, a1lp, b0hp, b1hp, a1hp]  
                    }
                
                    None => {
                        println!("[ERROR] Band width not defined!");
                        std::process::exit(1)
                    }
                }
            },
            "br" => { 
                match bw { 
                    Some(bw_value) => { 
                        design_filter.coeffs(FilterType::ButterType(ButterFilterType::Lp), fc - bw_value / 2.0, self.fs);
                        let b0lp = design_filter.filt_coeffs.b0; 
                        let b1lp = design_filter.filt_coeffs.b1;
                        let a1lp = design_filter.filt_coeffs.a1;
        
                        design_filter.coeffs(FilterType::ButterType(ButterFilterType::Hp), fc + bw_value / 2.0, self.fs);
                        let b0hp = design_filter.filt_coeffs.b0; 
                        let b1hp = design_filter.filt_coeffs.b1;
                        let a1hp = design_filter.filt_coeffs.a1;
                        
                        vec![b0lp, b1lp, a1lp, b0hp, b1hp, a1hp]  
                    }
                
                    None => {
                        println!("[ERROR] Band width not defined!");
                        std::process::exit(1)
                    }
                }
            },
            _ => {
                println!("[ERROR] Filter mode not allowed!");
                std::process::exit(1)
            }
        };

        self.mode = String::from(mode);

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

        let mut x1 = sample;
        let mut x2 = sample;
        let mut y: f64 = 0.0;
        
        if self.mode.eq("bp") {
            for _ in 0..self.order {
                let _ytemp = coeffs[0] * x1 + coeffs[1] * self.xtemp.read() - coeffs[2] * self.ytemp.read(); 
                y = coeffs[3] * _ytemp + coeffs[4] * self.x1.read() - coeffs[5] * self.y1.read();
                self.xtemp.write_and_advance(&x1);
                self.ytemp.write_and_advance(&_ytemp);
                self.x1.write_and_advance(&_ytemp);
                self.y1.write_and_advance(&y);
                x1 = _ytemp;
            }
            y
        } else if self.mode.eq("br") {
            for _ in 0..self.order {
                let _ylp = coeffs[0] * x1 + coeffs[1] * self.xtemp.read() - coeffs[2] * self.ytemp.read();
                let _yhp = coeffs[3] * x2 + coeffs[4] * self.x1.read() - coeffs[5] * self.y1.read();
                self.xtemp.write_and_advance(&x1);
                self.ytemp.write_and_advance(&_ylp);
                self.x1.write_and_advance(&x2);
                self.y1.write_and_advance(&_yhp);
                y = _ylp + _yhp;
                x1 = _ylp;
                x2 = _yhp;
            }
            y
        } else {
            for _ in 0..self.order {
                y = coeffs[0] * x1 + coeffs[1] * self.x1.read() - coeffs[2] * self.y1.read();
                self.x1.write_and_advance(&x1);
                self.y1.write_and_advance(&y);
                x1 = y;
            }
            y
        }

        // self.xtemp = xtemp;
        // self.ytemp = ytemp;
        // self.x1 = x1;
        // self.y1 = y1;
        
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
    ///     xtemp[n - 1] = 0.0
    ///     ytemp[n - 1] = 0.0
    ///     x1[n - 1] = 0.0
    ///     y1[n - 1] = 0.0
    ///

    pub fn clear_delayed_samples_cache(&mut self) {
        self.xtemp.clear();
        self.ytemp.clear();
        self.x1.clear();
        self.y1.clear();
        println!("[DONE] cache cleared!")
    }
}