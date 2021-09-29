use std::env;
use std::error::Error;

use calamine::{open_workbook, Xlsx};
use postgres::{Client, NoTls};

use crate::reader::parsers::parse_sheets;

fn parse_and_insert() -> Result<(), Box<dyn Error>> {
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

fn serve() {
    // TODO: start backend
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Result::Err(format!("Usage: {} [parse|serve]", args.get(0).unwrap()).into());
    }

    match args.get(1).unwrap().as_str() {
        "parse" => parse_and_insert(),
        // "serve" => TODO
        _ => Result::Err("Unknown subcommand.".into()),
    }
}
