//! 02 (0x) - единицы измерения через фантомный параметр. Эталонное решение.
use std::marker::PhantomData;

pub struct Meters;
pub struct Feet;

pub struct Length<U> {
    value: f64,
    _unit: PhantomData<U>,
}

impl<U> Length<U> {
    pub fn new(value: f64) -> Self {
        Length { value, _unit: PhantomData }
    }
    pub fn value(&self) -> f64 {
        self.value
    }
    pub fn add(&self, other: &Length<U>) -> Length<U> {
        Length::new(self.value + other.value)
    }
}

const FEET_PER_METER: f64 = 3.280_839_895;

impl Length<Meters> {
    pub fn to_feet(&self) -> Length<Feet> {
        Length::new(self.value * FEET_PER_METER)
    }
}

impl Length<Feet> {
    pub fn to_meters(&self) -> Length<Meters> {
        Length::new(self.value / FEET_PER_METER)
    }
}
