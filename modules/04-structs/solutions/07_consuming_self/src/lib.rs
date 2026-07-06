//! 07 (1x) — Методы, потребляющие `self`. Эталонное решение.

pub struct Temperature {
    pub celsius: f64,
}

impl Temperature {
    pub fn new(celsius: f64) -> Self {
        Temperature { celsius }
    }

    pub fn into_fahrenheit(self) -> f64 {
        self.celsius * 9.0 / 5.0 + 32.0
    }

    pub fn into_kelvin(self) -> f64 {
        self.celsius + 273.15
    }
}

pub struct Wrapper {
    inner: String,
}

impl Wrapper {
    pub fn new(inner: String) -> Self {
        Wrapper { inner }
    }

    pub fn into_inner(self) -> String {
        self.inner
    }
}
