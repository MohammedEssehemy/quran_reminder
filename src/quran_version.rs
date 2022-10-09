use rand::Rng;
use std::io::Error as IoError;

use crate::PageResult;

#[derive(Debug)]
pub enum QuranVersion {
    Madina,
    Tajweed,
}

const QURAN_NO_PAGES: u32 = 604;

impl QuranVersion {
    pub fn parse(text: String) -> Self {
        match text.to_lowercase().as_str() {
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
