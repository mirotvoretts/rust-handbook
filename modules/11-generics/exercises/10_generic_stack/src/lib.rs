//! 10 (3x) - Обобщённый стек.
//!
//! Классическая обёртка над Vec<T> с полным набором методов. Обратите внимание на формы
//! self: `&self` для просмотра, `&mut self` для мутации, `self` для map (стек
//! пересобирается в НОВЫЙ тип элементов Stack<U>). Порядок элементов в map сохраняется.

pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    /// Пустой стек.
    pub fn new() -> Self {
        todo!()
    }

    /// Кладёт значение на вершину.
    pub fn push(&mut self, value: T) {
        todo!()
    }

    /// Снимает значение с вершины (None, если пусто).
    pub fn pop(&mut self) -> Option<T> {
        todo!("Vec::pop делает ровно это")
    }

    /// Смотрит на вершину, не снимая.
    pub fn peek(&self) -> Option<&T> {
        todo!("Vec::last")
    }

    /// Изменяемая ссылка на вершину.
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        todo!("Vec::last_mut")
    }

    pub fn len(&self) -> usize {
        todo!()
    }

    pub fn is_empty(&self) -> bool {
        todo!()
    }

    /// Преобразует каждый элемент, сохраняя порядок: Stack<T> -> Stack<U>.
    pub fn map<U>(self, f: impl Fn(T) -> U) -> Stack<U> {
        todo!("пройдитесь по self.items, соберите новый Vec")
    }
}
