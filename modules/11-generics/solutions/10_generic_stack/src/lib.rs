//! 10 (3x) - Обобщённый стек. Эталонное решение.

pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Stack<T> {
    /// Пустой стек.
    pub fn new() -> Self {
        Stack { items: Vec::new() }
    }

    /// Кладёт значение на вершину.
    pub fn push(&mut self, value: T) {
        self.items.push(value);
    }

    /// Снимает значение с вершины (None, если пусто).
    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    /// Смотрит на вершину, не снимая.
    pub fn peek(&self) -> Option<&T> {
        self.items.last()
    }

    /// Изменяемая ссылка на вершину.
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.items.last_mut()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Преобразует каждый элемент, сохраняя порядок: Stack<T> -> Stack<U>.
    pub fn map<U>(self, f: impl Fn(T) -> U) -> Stack<U> {
        let mut items = Vec::new();
        for item in self.items {
            items.push(f(item));
        }
        Stack { items }
    }
}
