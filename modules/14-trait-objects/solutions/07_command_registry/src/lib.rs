//! 07 (3x) — Реестр команд: плагины в миниатюре. Эталонное решение.

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
        self.total += arg;
        format!("total: {}", self.total)
    }
}

/// Эхо: отвечает "echo: N", состояния не имеет.
pub struct Echo;

impl Command for Echo {
    fn execute(&mut self, arg: i64) -> String {
        format!("echo: {arg}")
    }
}

pub struct Registry {
    commands: Vec<(String, Box<dyn Command>)>,
}

impl Registry {
    pub fn new() -> Self {
        Registry { commands: Vec::new() }
    }

    /// Регистрирует команду под именем.
    pub fn register(&mut self, name: &str, cmd: Box<dyn Command>) {
        self.commands.push((name.to_string(), cmd));
    }

    /// Выполняет команду по имени. None — если не зарегистрирована.
    pub fn dispatch(&mut self, name: &str, arg: i64) -> Option<String> {
        for (n, cmd) in self.commands.iter_mut() {
            if n == name {
                return Some(cmd.execute(arg));
            }
        }
        None
    }
}
