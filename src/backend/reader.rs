use chrono::*;

mod common;
mod parser_first;
mod parser_second;
mod parser_third;
pub mod parsers;
mod types;

#[derive(Debug)]
pub struct Expense {
    pub date: NaiveDate,
    pub title: String,
    pub amount: u32,
}
