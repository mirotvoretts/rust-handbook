//! 07 (3x) - Шина событий на Vec<Box<dyn FnMut(&str)>>. Эталонное решение.

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
        Self::default()
    }

    /// Регистрирует подписчика ('static: живёт внутри шины).
    pub fn subscribe<F: FnMut(&str) + 'static>(&mut self, f: F) {
        self.subscribers.push(Box::new(f));
    }

    /// Рассылает msg всем подписчикам по порядку.
    pub fn publish(&mut self, msg: &str) {
        for s in &mut self.subscribers {
            s(msg);
        }
    }

    /// Число подписчиков.
    pub fn subscriber_count(&self) -> usize {
        self.subscribers.len()
    }
}
