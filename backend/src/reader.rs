mod common;
mod parser_first;
mod parser_second;
mod parser_third;
pub mod parsers;
mod types;

use std::error::Error;

use calamine::{open_workbook, Xlsx};
use chrono::*;
use postgres::{Client, NoTls};

use parsers::parse_sheets;

#[derive(Debug)]
pub struct Expense {
    pub date: NaiveDate,
    pub title: String,
    pub amount: u32,
}

pub fn parse() -> Result<(), Box<dyn Error>> {
    let path = "/Users/slasyz/Dropbox/Расходы.xlsx";
    let mut workbook: Xlsx<_> = open_workbook(path)?;

    let mut postgres_client = Client::connect(
        "host=localhost port=30231 user=expenses password=expenses dbname=expenses",
        NoTls,
    )?;

    let expenses = parse_sheets(&mut workbook)?;

    postgres_client.execute("DELETE FROM expenses", &[])?;

    for expense in expenses.iter() {
        eprintln!("inserting {:?}", expense);
        let amount = expense.amount as i64;
        postgres_client.execute(
            "INSERT INTO expenses(title, date, amount) VALUES($1, $2, $3)",
            &[&expense.title, &expense.date, &amount],
        )?;
    }

    Ok(())
}
