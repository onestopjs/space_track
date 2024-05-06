pub trait OrderByField {
    fn field(&self) -> &str;
}

pub enum Direction {
    Ascending,
    Descending,
}

pub struct OrderBy<T: OrderByField> {
    pub field: T,
    pub direction: Direction,
}

pub struct Config<T: OrderByField> {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub order_by: Vec<OrderBy<T>>,
    pub distinct: bool,
}

impl<T: OrderByField> Config<T> {
    pub fn new() -> Config<T> {
        Config {
            limit: Some(100),
            offset: None,
            order_by: Vec::new(),
            distinct: false,
        }
    }

    pub fn empty() -> Config<T> {
        Config {
            limit: None,
            offset: None,
            order_by: Vec::new(),
            distinct: false,
        }
    }

    pub fn limit(mut self, limit: u32) -> Config<T> {
        self.limit = Some(limit);
        self
    }

    pub fn offset(mut self, offset: u32) -> Config<T> {
        self.offset = Some(offset);
        self
    }

    pub fn order_by(mut self, field: T, direction: Direction) -> Config<T> {
        self.order_by.push(OrderBy { field, direction });
        self
    }

    pub fn distinct(mut self) -> Config<T> {
        self.distinct = true;
        self
    }
}
