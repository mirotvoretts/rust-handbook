//! 07 (3x) - Капстоун: шина событий на `Vec<Box<dyn FnMut(&str)>>`.
//!
//! Объединяет M14 (trait-объекты, стирание типа) и M18 (замыкания как значения).
//! Разные подписчики - разные невыразимые типы замыканий, поэтому хранить их
//! в одном Vec можно только за `Box<dyn ...>`. Подписчик - FnMut(&str): при
//! рассылке ему дают сообщение, и он вправе менять своё окружение (копить лог,
//! считать вызовы). Отсюда `&mut self` в publish и `&mut` при обходе.

/// Подписчик: замыкание, принимающее сообщение и, возможно, меняющее окружение.
type Subscriber = Box<dyn FnMut(&str)>;

/// Шина событий: хранит подписчиков и рассылает им сообщения.
#[derive(Default)]
pub struct EventBus {
    subscribers: Vec<Subscriber>,
}

impl EventBus {
    /// Пустая шина.
    pub fn new() -> Self {
        todo!("Self::default() или Self { subscribers: Vec::new() }")
    }

    /// Регистрирует подписчика. Граница `'static`: замыкание переживёт вызов
    /// и будет жить внутри шины, поэтому не должно заимствовать локальные данные.
    pub fn subscribe<F: FnMut(&str) + 'static>(&mut self, f: F) {
        todo!("self.subscribers.push(Box::new(f))")
    }

    /// Рассылает msg всем подписчикам в порядке регистрации.
    pub fn publish(&mut self, msg: &str) {
        todo!("for s in &mut self.subscribers { s(msg); }")
    }

    /// Сколько сейчас подписчиков.
    pub fn subscriber_count(&self) -> usize {
        todo!("self.subscribers.len()")
    }
}
