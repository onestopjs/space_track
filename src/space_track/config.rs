pub struct Config {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

impl Config {
    pub fn new() -> Config {
        Config {
            limit: Some(100),
            offset: None,
        }
    }

    pub fn empty() -> Config {
        Config {
            limit: None,
            offset: None,
        }
    }

    pub fn limit(mut self, limit: u32) -> Config {
        self.limit = Some(limit);
        self
    }

    pub fn offset(mut self, offset: u32) -> Config {
        self.offset = Some(offset);
        self
    }
}
