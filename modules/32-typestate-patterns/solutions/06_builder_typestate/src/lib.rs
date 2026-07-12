//! 06 (2x) - type-state builder с обязательным url. Эталонное решение.
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

impl RequestBuilder<No> {
    pub fn new() -> Self {
        RequestBuilder {
            url: None,
            method: "GET".to_string(),
            body: String::new(),
            _has_url: PhantomData,
        }
    }
}

impl Default for RequestBuilder<No> {
    fn default() -> Self {
        Self::new()
    }
}

impl<HasUrl> RequestBuilder<HasUrl> {
    pub fn method(mut self, m: &str) -> Self {
        self.method = m.to_string();
        self
    }
    pub fn body(mut self, b: &str) -> Self {
        self.body = b.to_string();
        self
    }
    pub fn url(self, u: &str) -> RequestBuilder<Yes> {
        RequestBuilder {
            url: Some(u.to_string()),
            method: self.method,
            body: self.body,
            _has_url: PhantomData,
        }
    }
}

impl RequestBuilder<Yes> {
    pub fn build(self) -> Request {
        Request {
            url: self.url.unwrap(),
            method: self.method,
            body: self.body,
        }
    }
}
