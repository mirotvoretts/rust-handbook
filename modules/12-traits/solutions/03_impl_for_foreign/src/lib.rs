//! 03 (0x) - Свой трейт для чужих типов. Эталонное решение.

pub trait Weight {
    /// Неотрицательный "вес" значения.
    fn weight(&self) -> u64;
}

impl Weight for i32 {
    fn weight(&self) -> u64 {
        self.unsigned_abs() as u64
    }
}

impl Weight for String {
    fn weight(&self) -> u64 {
        self.len() as u64
    }
}

impl Weight for &str {
    fn weight(&self) -> u64 {
        self.len() as u64
    }
}

impl Weight for Vec<i32> {
    fn weight(&self) -> u64 {
        let mut total = 0u64;
        for x in self {
            total += x.weight();
        }
        total
    }
}
