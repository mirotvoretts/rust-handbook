//! 06 (2x) - типы нулевого размера на практике. Эталонное решение.
use std::collections::HashMap;
use std::hash::Hash;

pub struct Marker;

pub const fn is_zst<T>() -> bool {
    std::mem::size_of::<T>() == 0
}

pub struct TinySet<T> {
    map: HashMap<T, ()>,
}

impl<T: Hash + Eq> TinySet<T> {
    pub fn new() -> Self {
        TinySet { map: HashMap::new() }
    }
    pub fn insert(&mut self, value: T) -> bool {
        self.map.insert(value, ()).is_none()
    }
    pub fn contains(&self, value: &T) -> bool {
        self.map.contains_key(value)
    }
    pub fn len(&self) -> usize {
        self.map.len()
    }
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}

impl<T: Hash + Eq> Default for TinySet<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct UnitCounter {
    items: Vec<()>,
}

impl UnitCounter {
    pub fn new() -> Self {
        UnitCounter { items: Vec::new() }
    }
    pub fn tick(&mut self) {
        self.items.push(());
    }
    pub fn count(&self) -> usize {
        self.items.len()
    }
}

impl Default for UnitCounter {
    fn default() -> Self {
        Self::new()
    }
}
