//! 06 (2x) - type-state builder с обязательным url.
//!
//! Построй билдер запроса, где обязательное поле `url` кодируется на уровне типа:
//! метод `build()` доступен только тогда, когда url уже задан. Метки состояния -
//! `Yes` и `No`. Тип `Request` и структура `RequestBuilder<HasUrl>` с приватными
//! полями уже даны.
//!
//! Реализуй сам (impl-блоки не даны):
//! - `RequestBuilder::<No>::new()` (по умолчанию `method` = `"GET"`, `body` = "");
//! - на любом `RequestBuilder<H>`: `method(self, &str) -> Self`,
//!   `body(self, &str) -> Self`, `url(self, &str) -> RequestBuilder<Yes>`;
//! - на `RequestBuilder<Yes>`: `build(self) -> Request`.
//!
//! Подсказки:
//! - `url` меняет тип билдера с `<No>` на `<Yes>`, поэтому возвращает новый
//!   `RequestBuilder<Yes>`, перенося уже накопленные поля;
//! - поле `_has_url: PhantomData<HasUrl>` инициализируется значением `PhantomData`.
//!
//! Пока impl-блоки не написаны, крейт не компилируется - это нормальное состояние
//! нерешённого упражнения.
//! Про PhantomData:
//! <https://doc.rust-lang.org/std/marker/struct.PhantomData.html>.
use std::marker::PhantomData;

pub struct Yes;
pub struct No;

#[derive(Debug, PartialEq)]
pub struct Request {
    pub url: String,
    pub method: String,
    pub body: String,
}

pub struct RequestBuilder<HasUrl> {
    url: Option<String>,
    method: String,
    body: String,
    _has_url: PhantomData<HasUrl>,
}
