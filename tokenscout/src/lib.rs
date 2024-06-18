pub mod importer;
pub mod networking;

impl tokenscout {
    pub fn new() -> Self {
        tokenscout {}
    }
}

trait Tokenscout {
    fn new() -> Self;

    fn start(&self) -> Result<(), Box<dyn std::error::Error>>;
}

#[derive(Debug, Default, Clone)]
pub struct tokenscout {}
