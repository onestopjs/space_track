pub enum Direction {
    Ascending,
    Descending,
}

pub struct OrderBy {
    pub field: String,
    pub direction: Direction,
}

pub struct Config {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub order_by: Vec<OrderBy>,
}

impl Config {
    pub fn new() -> Config {
        Config {
            limit: Some(100),
            offset: None,
            order_by: Vec::new(),
        }
    }

    pub fn empty() -> Config {
        Config {
            limit: None,
            offset: None,
            order_by: Vec::new(),
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

    pub fn order_by(mut self, field: String, direction: Direction) -> Config {
        self.order_by.push(OrderBy { field, direction });
        self
    }
}
