//! 07 (2x) — Супертрейты.
//!
//! `Pet: Animal` означает «каждый Pet обязан быть Animal» (не наследование данных!).
//! Default-метод `tag` супертрейта Pet может вызывать методы Animal. Для типа `Dog`
//! нужны ДВА отдельных impl: `impl Animal for Dog` и `impl Pet for Dog`.

pub trait Animal {
    fn name(&self) -> String;
}

pub trait Pet: Animal {
    fn owner(&self) -> String;

    /// Default: "{name} (хозяин: {owner})".
    fn tag(&self) -> String {
        todo!("методы Animal доступны через self")
    }
}

pub struct Dog {
    pub nickname: String,
    pub owner_name: String,
}

// todo!(): impl Animal for Dog (name -> nickname)
impl Animal for Dog {
    fn name(&self) -> String {
        todo!()
    }
}

// todo!(): impl Pet for Dog (owner -> owner_name)
impl Pet for Dog {
    fn owner(&self) -> String {
        todo!()
    }
}
