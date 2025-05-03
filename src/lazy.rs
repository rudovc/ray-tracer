use std::cell::OnceCell;

#[derive(Debug, Clone, PartialEq)]
pub enum Lazy<T> {
    Lazy(OnceCell<T>),
    Eager(T),
}

impl<T: Copy> Lazy<T> {
    pub fn get_or_init(&self, value: T) -> T {
        match &self {
            Lazy::Lazy(inner) => *inner.get_or_init(|| value),
            Lazy::Eager(inner) => *inner,
        }
    }
}
