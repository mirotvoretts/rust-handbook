//! 02 (0x) - единицы измерения через фантомный параметр.
//!
//! Реализуй тип `Length<U>` с фантомной единицей измерения `U`. Складывать
//! разрешено только длины одинаковой единицы (сигнатура `add` уже это гарантирует),
//! а конверсия между единицами вынесена в отдельные методы `to_feet`/`to_meters`,
//! доступные только на соответствующей единице.
//!
//! Тебе нужно заполнить тела `new`, `value`, `add`, `to_feet`, `to_meters`.
//!
//! Подсказки:
//! - поле `_unit: PhantomData<U>` инициализируется значением `PhantomData`;
//! - для конверсии используй уже заданную константу `FEET_PER_METER`.
//!
//! Заглушки помечены `todo!()` - крейт компилируется, тесты падают, пока не решено.
//! Про PhantomData: <https://doc.rust-lang.org/nomicon/phantom-data.html>.
use std::marker::PhantomData;

pub struct Meters;
pub struct Feet;

pub struct Length<U> {
    value: f64,
    _unit: PhantomData<U>,
}

impl<U> Length<U> {
    pub fn new(_value: f64) -> Self {
        todo!()
    }
    pub fn value(&self) -> f64 {
        todo!()
    }
    pub fn add(&self, _other: &Length<U>) -> Length<U> {
        todo!()
    }
}

const FEET_PER_METER: f64 = 3.280_839_895;

impl Length<Meters> {
    pub fn to_feet(&self) -> Length<Feet> {
        todo!()
    }
}

impl Length<Feet> {
    pub fn to_meters(&self) -> Length<Meters> {
        todo!()
    }
}
