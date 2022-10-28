use std::{error::Error, fmt::Debug};

pub trait Transport: Debug {
    fn send(&self, attachment: &str, language: &str) -> Result<(), Box<dyn Error>>;
    fn from_env() -> Result<Box<Self>, Box<dyn Error>>
    where
        Self: Sized;
}
