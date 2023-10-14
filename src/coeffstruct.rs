#![allow(clippy::wrong_self_convention)]
#![allow(clippy::new_without_default)]

pub struct BiquadCoeffs {
    pub b0: f64,
    pub b1: f64,
    pub b2: f64,
    pub a0: f64,
    pub a1: f64,
    pub a2: f64
}

impl BiquadCoeffs {
    pub fn new() -> Self {
        Self { b0: 0.0, b1: 0.0, b2: 0.0, a0: 0.0, a1: 0.0, a2: 0.0 }
    }

    pub fn set_coeffs(&mut self, coeffs: (f64, f64, f64, f64, f64, f64)) {
        self.b0 = coeffs.0;
        self.b1 = coeffs.1;
        self.b2 = coeffs.2;
        self.a0 = coeffs.3;
        self.a1 = coeffs.4;
        self.a2 = coeffs.5;
    }

}

pub struct OnePoleCoeffs {
    pub b0: f64,
    pub b1: f64,
    pub a1: f64
}

impl OnePoleCoeffs {
    pub fn new() -> Self {
        Self { b0: 0.0, b1: 0.0, a1: 0.0 }
    }

    pub fn set_coeffs(&mut self, coeffs: (f64, f64, f64)) {
        self.b0 = coeffs.0;
        self.b1 = coeffs.1;
        self.a1 = coeffs.2;
    }

}