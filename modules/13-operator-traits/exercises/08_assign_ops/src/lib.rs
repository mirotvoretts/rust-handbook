//! 08 (2x) - Составные присваивания.
//!
//! `+=` - это НЕ автоматический сахар над Add: AddAssign - отдельный трейт с &mut self.
//! Ресурсный счётчик: add_assign докладывает ресурсы, sub_assign списывает С НАСЫЩЕНИЕМ
//! (не ниже нуля - saturating_sub).

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Resources(pub u32);

impl std::ops::AddAssign<u32> for Resources {
    fn add_assign(&mut self, amount: u32) {
        todo!("мутируем self.0")
    }
}

impl std::ops::SubAssign<u32> for Resources {
    fn sub_assign(&mut self, amount: u32) {
        todo!("saturating_sub: списание ниже нуля даёт ноль")
    }
}
