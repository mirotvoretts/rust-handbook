//! 09 (2x) - "Карающий" деструктор и paniking().
//!
//! Transaction обязана завершаться явно: commit или rollback. Забытая транзакция -
//! баг, и Drop наказывает паникой. НО: если Drop сработал во время УЖЕ идущей паники,
//! паниковать нельзя (паника-в-панике = abort) - проверяйте thread::panicking().
//!
//! mem::forget в commit/rollback обезвреживает деструктор (M06 §8, конспект: приём
//! "эмуляция линейных типов").

pub struct Transaction {
    pub committed_flag: bool, // для тестов; в жизни тут был бы дескриптор
}

impl Transaction {
    pub fn begin() -> Self {
        Transaction { committed_flag: false }
    }

    /// Явное успешное завершение: Drop не должен сработать.
    pub fn commit(self) -> bool {
        todo!("std::mem::forget(self); верните true")
    }

    /// Явная отмена: Drop тоже не должен сработать.
    pub fn rollback(self) -> bool {
        todo!()
    }
}

impl Drop for Transaction {
    fn drop(&mut self) {
        todo!("если !std::thread::panicking() - panic! про забытую транзакцию; иначе молча")
    }
}
