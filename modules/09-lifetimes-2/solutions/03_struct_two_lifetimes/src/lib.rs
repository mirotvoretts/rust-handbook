//! 03 (1x) - Структура с двумя независимыми временами жизни. Эталонное решение.

pub struct Report<'a, 'b> {
    pub title: &'a str,
    pub data: &'b [i32],
}

impl<'a, 'b> Report<'a, 'b> {
    pub fn new(title: &'a str, data: &'b [i32]) -> Report<'a, 'b> {
        Report { title, data }
    }

    pub fn title(&self) -> &str {
        self.title
    }

    pub fn sum(&self) -> i32 {
        self.data.iter().sum()
    }

    pub fn header(&self) -> String {
        format!("{} ({})", self.title, self.data.len())
    }
}
