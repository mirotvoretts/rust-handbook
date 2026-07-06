//! 04 (1x) — Замыкания за dyn: конвейер.
//!
//! Тип каждого замыкания уникален — в Vec их складывают за Box<dyn Fn>. Pipeline
//! хранит шаги-преобразования и прогоняет число через все по порядку. Замыкания
//! подробно в M18; здесь достаточно Box::new(move |x| ...).

pub type Step = Box<dyn Fn(i64) -> i64>;

pub struct Pipeline {
    steps: Vec<Step>,
}

impl Pipeline {
    pub fn new() -> Self {
        todo!()
    }

    /// Добавляет шаг в конец конвейера.
    pub fn add_step(&mut self, step: Step) {
        todo!()
    }

    /// Шаг «прибавить k» (постройте замыкание сами: move |x| ...).
    pub fn add_offset(&mut self, k: i64) {
        todo!("self.add_step(Box::new(move |x| x + k))")
    }

    /// Прогоняет x через все шаги по порядку.
    pub fn run(&self, x: i64) -> i64 {
        todo!()
    }
}
