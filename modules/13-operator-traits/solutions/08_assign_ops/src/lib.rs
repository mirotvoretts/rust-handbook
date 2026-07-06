//! 08 (2x) - Составные присваивания. Эталонное решение.

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Resources(pub u32);

impl std::ops::AddAssign<u32> for Resources {
    fn add_assign(&mut self, amount: u32) {
        self.0 += amount;
    }
}

impl std::ops::SubAssign<u32> for Resources {
    fn sub_assign(&mut self, amount: u32) {
        self.0 = self.0.saturating_sub(amount);
    }
}
