use rand::Rng;
use std::{io::Error as IoError, env};

use crate::PageResult;

#[derive(Debug)]
pub enum QuranVersion {
    Madina,
    Tajweed,
}

const QURAN_NO_PAGES: u32 = 604;

impl QuranVersion {
    pub fn from_env() -> Self {
        let env_value = env::var("QURAN_VERSION").unwrap_or_default();
        Self::parse(&env_value)
    }

    pub fn parse(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "madina" => Self::Madina,
            "tajweed" => Self::Tajweed,
            _ => Self::Madina,
        }
    }

    pub fn get_page_path(&self, page_no: u32) -> String {
        match self {
            Self::Madina => format!("./assets/madina/{page_no}.jpeg").to_owned(),
            Self::Tajweed => format!("./assets/tajweed/{page_no}.png").to_owned(),
        }
    }

    pub fn get_random_page(&self) -> Result<PageResult, IoError> {
        let page_no = Self::get_random_page_no();
        let file_path = self.get_page_path(page_no);
        Ok(PageResult {
            no: page_no,
            file_path,
        })
    }

    fn get_random_page_no() -> u32 {
        let mut rng = rand::thread_rng();
        let page_no = rng.gen_range(1..QURAN_NO_PAGES);
        return page_no;
    }
}



 #[cfg(test)] 
 mod tests {
    use super::*;

    #[test]
    fn should_parse_correctly() {
        let version = QuranVersion::parse("madina");
        assert!(matches!(version, QuranVersion::Madina));

        let version = QuranVersion::parse("tajweed");
        assert!(matches!(version, QuranVersion::Tajweed));

        // default is Madina
        let version = QuranVersion::parse("");
        assert!(matches!(version, QuranVersion::Madina));
    }

    #[test]
    fn should_get_correct_path() {
        let madina = QuranVersion::Madina;
        assert_eq!(madina.get_page_path(10_u32), "./assets/madina/10.jpeg");

        let madina = QuranVersion::Tajweed;
        assert_eq!(madina.get_page_path(10_u32), "./assets/tajweed/10.png");
    }

    #[test]
    fn should_get_random_page() {
        let madina = QuranVersion::Madina;
        let page = madina.get_random_page().unwrap();
        let valid_range = 1..=QURAN_NO_PAGES;
        assert!(valid_range.contains(&page.no));
        assert_eq!(page.file_path, format!("./assets/madina/{}.jpeg", page.no));
    }

 }