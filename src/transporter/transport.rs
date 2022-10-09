use std::{error::Error, fmt::Debug};

pub trait Transport: Debug {
    fn send(&self, attachment: &str, language: &str) -> Result<(), Box<dyn Error>>;
}

pub trait TransportFromEnv: Sized {
    fn from_env() -> Result<Box<dyn Transport>, Box<dyn Error>>;
}