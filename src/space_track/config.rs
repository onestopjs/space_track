use std::{fmt::Display, ops::Add};

use serde::{Deserialize, Serialize};

pub trait OrderByField {
    fn field(&self) -> &str;
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Direction {
    Ascending,
    Descending,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderBy<T: OrderByField> {
    pub field: T,
    pub direction: Direction,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Predicate<T: OrderByField> {
    pub field: T,
    pub value: String,
}

impl<T: OrderByField> Predicate<T> {
    pub fn build_range_list<V: Ord + Display + Clone + Add<u32, Output = V>>(
        field: T,
        mut values: Vec<V>,
    ) -> Self {
        values.sort();
        let mut ranges = Vec::new();
        let mut current = (values[0].clone(), values[0].clone());
        for value in values.into_iter().skip(1) {
            if value == current.1.clone() + 1 {
                current.1 = value;
            } else {
                ranges.push(Self::range(&current.0, &current.1));
                current = (value.clone(), value);
            }
        }
        ranges.push(Self::range(&current.0, &current.1));

        Self {
            field,
            value: ranges.join(","),
        }
    }

    fn range<V: Ord + Display>(start: &V, end: &V) -> String {
        if start == end {
            start.to_string()
        } else {
            format!("{}--{}", start, end)
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config<T: OrderByField> {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub order_by: Vec<OrderBy<T>>,
    pub predicates: Vec<Predicate<T>>,
    pub distinct: bool,
}

impl<T: OrderByField> Config<T> {
    pub fn new() -> Config<T> {
        Config {
            limit: Some(100),
            offset: None,
            order_by: Vec::new(),
            predicates: Vec::new(),
            distinct: false,
        }
    }

    pub fn empty() -> Config<T> {
        Config {
            limit: None,
            offset: None,
            order_by: Vec::new(),
            predicates: Vec::new(),
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

    pub fn predicate(mut self, predicate: Predicate<T>) -> Config<T> {
        self.predicates.push(predicate);
        self
    }
}
