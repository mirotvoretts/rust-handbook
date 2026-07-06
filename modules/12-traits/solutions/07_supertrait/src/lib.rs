//! 07 (2x) — Супертрейты. Эталонное решение.

pub trait Animal {
    fn name(&self) -> String;
}

pub trait Pet: Animal {
    fn owner(&self) -> String;

    /// Default: "{name} (хозяин: {owner})".
    fn tag(&self) -> String {
        format!("{} (хозяин: {})", self.name(), self.owner())
    }
}

pub struct Dog {
    pub nickname: String,
    pub owner_name: String,
}

impl Animal for Dog {
    fn name(&self) -> String {
        self.nickname.clone()
    }
}

impl Pet for Dog {
    fn owner(&self) -> String {
        self.owner_name.clone()
    }
}
