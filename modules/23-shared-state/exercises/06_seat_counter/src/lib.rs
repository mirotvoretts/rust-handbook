//! 06 (2x) - compare_exchange: атомарный счётчик с потолком.
//!
//! Счётчик мест: try_take() пытается занять место и возвращает true при успехе,
//! false - если все `capacity` мест уже заняты. Инвариант: суммарно успешных
//! try_take() не больше capacity, даже когда потоки бьются наперегонки.
//!
//! Mutex использовать нельзя - только AtomicUsize и цикл compare_exchange (CAS).
//! Наивное "if taken < cap { taken += 1 }" через load/store содержит гонку: между
//! проверкой и записью другой поток успеет занять место. CAS-цикл закрывает окно:
//! читаем текущее значение, а замену на +1 делаем только если значение не
//! изменилось; при промахе повторяем с актуальным.

use std::sync::atomic::AtomicUsize;

pub struct SeatCounter {
    // TODO: taken: AtomicUsize, capacity: usize.
}

impl SeatCounter {
    /// Счётчик на `capacity` мест, все свободны.
    pub fn new(capacity: usize) -> Self {
        todo!()
    }

    /// Занять одно место. true - удалось, false - мест нет.
    pub fn try_take(&self) -> bool {
        todo!()
    }

    /// Сколько мест занято сейчас.
    pub fn taken(&self) -> usize {
        todo!()
    }
}
