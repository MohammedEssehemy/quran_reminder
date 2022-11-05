use std::{
    error::Error,
    fmt::{Debug, Formatter},
};

pub trait Transport: Debug {
    fn send(&self, attachment: &str, language: &str) -> Result<(), Box<dyn Error>>;
    fn from_env() -> Result<Box<Self>, Box<dyn Error>>
    where
        Self: Sized;
}

pub fn format_secret<T>(_: &T, f: &mut Formatter) -> Result<(), std::fmt::Error> {
    f.write_str(r#""******""#)
}
