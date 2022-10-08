use std::{error::Error, fmt::Debug};
use crate::page_result::PageResult;

pub trait Transport: Debug {
    fn send(&self, page: &PageResult, language: &str) -> Result<(), Box<dyn Error>>;
}

pub trait TransportFromEnv: Sized {
    fn from_env() -> Result<Box<dyn Transport>, Box<dyn Error>>;
}