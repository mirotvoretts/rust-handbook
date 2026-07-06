//! 04 (1x) — Замыкания за dyn: конвейер. Эталонное решение.

pub type Step = Box<dyn Fn(i64) -> i64>;

pub struct Pipeline {
    steps: Vec<Step>,
}

impl Default for Pipeline {
    fn default() -> Self {
        Self::new()
    }
}

impl Pipeline {
    pub fn new() -> Self {
        Pipeline { steps: Vec::new() }
    }

    /// Добавляет шаг в конец конвейера.
    pub fn add_step(&mut self, step: Step) {
        self.steps.push(step);
    }

    /// Шаг «прибавить k».
    pub fn add_offset(&mut self, k: i64) {
        self.add_step(Box::new(move |x| x + k));
    }

    /// Прогоняет x через все шаги по порядку.
    pub fn run(&self, x: i64) -> i64 {
        let mut acc = x;
        for step in &self.steps {
            acc = step(acc);
        }
        acc
    }
}
