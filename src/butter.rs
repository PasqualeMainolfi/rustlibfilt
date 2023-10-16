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
        let twopi = 2.0 * std::f64::consts::PI;
        let wc = twopi * fc;
        let ts = 1.0 / fs;
        let wtan = 2.0 * (wc * ts / 2.0).tan();
        match mode {
            ButterFilterType::Lp => {
                let b0 = wtan / (2.0 + wtan);
                let b1 = b0;
                let a1 = (wtan - 2.0) / (2.0 + wtan);

                self.filt_coeffs.set_coeffs((b0, b1, a1))
            },
            ButterFilterType::Hp => {
                let b0 = 2.0 / (2.0 + wtan);
                let b1 = -b0;
                let a1 = (wtan - 2.0) / (2.0 + wtan);

                self.filt_coeffs.set_coeffs((b0, b1, a1))
            },
            ButterFilterType::Bp => {},
            ButterFilterType::Notch => {},
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
    order: usize,
    xtemp: Vec<f64>,
    ytemp: Vec<f64>,
    x1: Vec<f64>,
    y1: Vec<f64>,
    index: usize
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
            xtemp: vec![0.0; filt_order], 
            ytemp: vec![0.0; filt_order], 
            x1: vec![0.0; filt_order], 
            y1: vec![0.0; filt_order],
            index: 0
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
                design_filter.coeffs(ButterFilterType::Lp, fc, self.fs);
                let b0 = design_filter.filt_coeffs.b0; 
                let b1 = design_filter.filt_coeffs.b1;
                let a1 = design_filter.filt_coeffs.a1;

                vec![b0, b1, a1]
            },
            "hp" => {
                design_filter.coeffs(ButterFilterType::Hp, fc, self.fs);
                let b0 = design_filter.filt_coeffs.b0; 
                let b1 = design_filter.filt_coeffs.b1;
                let a1 = design_filter.filt_coeffs.a1;

                vec![b0, b1, a1]
            },
            "bp" => { 
                match bw { 
                    Some(bw_value) => { 
                        design_filter.coeffs(ButterFilterType::Lp, fc + bw_value / 2.0, self.fs);
                        let b0lp = design_filter.filt_coeffs.b0; 
                        let b1lp = design_filter.filt_coeffs.b1;
                        let a1lp = design_filter.filt_coeffs.a1;
        
                        design_filter.coeffs(ButterFilterType::Hp, fc - bw_value / 2.0, self.fs);
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
                        design_filter.coeffs(ButterFilterType::Lp, fc - bw_value / 2.0, self.fs);
                        let b0lp = design_filter.filt_coeffs.b0; 
                        let b1lp = design_filter.filt_coeffs.b1;
                        let a1lp = design_filter.filt_coeffs.a1;
        
                        design_filter.coeffs(ButterFilterType::Hp, fc + bw_value / 2.0, self.fs);
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

        // let (y, xtemp, ytemp, x1, y1) = if self.mode.eq("bp") {
        //     let _ytemp = _filt_sample(&sample, &coeffs[0..3], self.xtemp, self.ytemp);
        //     let _y = _filt_sample(&_ytemp, &coeffs[3..6], self.x1, self.y1);
        //     (_y, sample, _ytemp, _ytemp, _y)
        // } else if self.mode.eq("br") {
        //     let _ylp = _filt_sample(&sample, &coeffs[0..3], self.xtemp, self.ytemp);
        //     let _yhp = _filt_sample(&sample, &coeffs[3..6], self.x1, self.y1);
        //     let _y = _ylp + _yhp;
        //     (_y, sample, _ylp, sample, _yhp)
        // } else {
        //     let _y = _filt_sample(&sample, &coeffs, self.x1, self.y1);
        //     (_y, 0.0, 0.0, sample, _y)
        // };


        let mut x1 = sample;
        let mut x2 = sample;
        let mut y: f64 = 0.0;
        
        if self.mode.eq("bp") {
            for _ in 0..self.order {
                let _ytemp = _filt_sample(&x1, &coeffs[0..3], self.xtemp[self.index], self.ytemp[self.index]);
                y = _filt_sample(&_ytemp, &coeffs[0..6], self.x1[self.index], self.y1[self.index]);
                self.xtemp[self.index] = x1;
                self.ytemp[self.index] = _ytemp;
                self.x1[self.index] = _ytemp;
                self.y1[self.index] = y;
                x1 = y;
                self.index += 1;
                self.index %= self.order; 
            }
            y
        } else if self.mode.eq("br") {
            for _ in 0..self.order {
                let _ylp = _filt_sample(&x1, &coeffs[0..3], self.xtemp[self.index], self.ytemp[self.index]);
                let _yhp = _filt_sample(&x2, &coeffs[0..6], self.x1[self.index], self.y1[self.index]);
                self.xtemp[self.index] = x1;
                self.ytemp[self.index] = _ylp;
                self.x1[self.index] = x2;
                self.y1[self.index] = _yhp;
                y = _ylp + _yhp;
                x1 = _ylp;
                x2 = _yhp;
                self.index += 1;
                self.index %= self.order; 
            }
            y
        } else {
            for _ in 0..self.order {
                y = _filt_sample(&x1, &coeffs, self.x1[self.index], self.y1[self.index]);
                self.x1[self.index] = x1;
                self.y1[self.index] = y;
                self.index += 1;
                self.index %= self.order;
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
        self.xtemp = vec![0.0; self.order];
        self.ytemp = vec![0.0; self.order];
        self.x1 = vec![0.0; self.order];
        self.y1 = vec![0.0; self.order];
        self.index = 0;
        println!("[DONE] cache cleared!")
    }
}