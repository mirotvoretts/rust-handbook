//! 07 (1x) - Приватные поля + публичные методы (инкапсуляция). Эталонное решение.

pub mod bank {
    pub struct Account {
        balance: u64,
    }

    impl Account {
        pub fn new(initial: u64) -> Account {
            Account { balance: initial }
        }

        pub fn balance(&self) -> u64 {
            self.balance
        }

        pub fn deposit(&mut self, amount: u64) {
            self.balance += amount;
        }
    }
}
