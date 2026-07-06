//! 12 (3x) - Стек на `Vec<i32>`.
//!
//! Соберите свой тип-контейнер: структура владеет `Vec<i32>`, а методы дают безопасный
//! интерфейс. `push`/`pop` меняют состояние (`&mut self`), `peek`/`len`/`is_empty` - читают
//! (`&self`). `pop` и `peek` возвращают `Option`, потому что стек может быть пуст.

pub struct Stack {
    items: Vec<i32>,
}

impl Stack {
    /// Пустой стек.
    pub fn new() -> Self {
        todo!("Vec::new()")
    }

    /// Кладёт значение на вершину.
    pub fn push(&mut self, value: i32) {
        todo!()
    }

    /// Снимает значение с вершины: `Some(x)` или `None`, если пусто.
    pub fn pop(&mut self) -> Option<i32> {
        todo!("Vec::pop уже возвращает Option")
    }

    /// Заглядывает на вершину, не снимая: `Some(x)` или `None`.
    pub fn peek(&self) -> Option<i32> {
        todo!()
    }

    /// Число элементов.
    pub fn len(&self) -> usize {
        todo!()
    }

    /// Пуст ли стек.
    pub fn is_empty(&self) -> bool {
        todo!()
    }
}
