//! 07 (3x) - Реестр команд: плагины в миниатюре.
//!
//! Команда - трейт с состоянием (&mut self!). Реестр хранит команды под именами и
//! диспетчеризует вызов по строке - набор команд открыт, тип стёрт. Это скелет любой
//! системы плагинов. HashMap появится в M17 - здесь Vec пар.

pub trait Command {
    /// Выполняет команду с аргументом, может менять своё состояние.
    fn execute(&mut self, arg: i64) -> String;
}

/// Счётчик: суммирует аргументы, отвечает "total: N".
pub struct Accumulator {
    pub total: i64,
}

impl Command for Accumulator {
    fn execute(&mut self, arg: i64) -> String {
        todo!()
    }
}

/// Эхо: отвечает "echo: N", состояния не имеет.
pub struct Echo;

impl Command for Echo {
    fn execute(&mut self, arg: i64) -> String {
        todo!()
    }
}

pub struct Registry {
    commands: Vec<(String, Box<dyn Command>)>,
}

impl Registry {
    pub fn new() -> Self {
        todo!()
    }

    /// Регистрирует команду под именем (перезапись существующей не требуется).
    pub fn register(&mut self, name: &str, cmd: Box<dyn Command>) {
        todo!()
    }

    /// Выполняет команду по имени. None - если не зарегистрирована.
    pub fn dispatch(&mut self, name: &str, arg: i64) -> Option<String> {
        todo!("найдите пару по имени; понадобится &mut на элемент - iter_mut")
    }
}
