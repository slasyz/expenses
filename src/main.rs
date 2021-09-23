mod reader;

use calamine::{open_workbook, Xlsx};
use postgres::{Client, NoTls};

use crate::reader::parsers::parse_sheets;
use std::error::Error;

fn things() -> Result<(), Box<dyn Error>> {
    let path = "/Users/slasyz/Dropbox/Расходы.xlsx";
    let mut workbook: Xlsx<_> = open_workbook(path)?;

    let mut postgres_client = Client::connect(
        "host=localhost port=30231 user=expenses password=expenses dbname=expenses",
        NoTls,
    )?;

    let expenses = parse_sheets(&mut workbook)?;

    postgres_client.execute("DELETE FROM expenses", &[])?;

    for expense in expenses.iter() {
        println!("inserting {:?}", expense);
        let amount = expense.amount as i64;
        postgres_client.execute(
            "INSERT INTO expenses(title, date, amount) VALUES($1, $2, $3)",
            &[&expense.title, &expense.date, &amount],
        )?;
    }

    Ok(())
}

fn main() {
    match things() {
        Ok(_) => return,
        Err(err) => panic!("Error: {}", err.to_string()),
    }
}
