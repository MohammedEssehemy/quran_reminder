#[derive(Debug)]
pub enum QuranVersion {
    Madina,
    Tajweed,
}

impl QuranVersion {
    pub fn parse(text: String) -> Self {
        match text.to_lowercase().as_str() {
            "madina" => Self::Madina,
            "tajweed" => Self::Tajweed,
            _ => Self::Madina,
        }
    }

    pub fn get_path(&self) -> String {
        match self {
            Self::Madina => "madina".to_owned(),
            Self::Tajweed => "tajweed".to_owned(),
        }
    }
}
