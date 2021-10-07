use calamine::{DataType, Range};
use chrono::NaiveDate;

use super::parsers::Result;
use crate::reader::common::parse_year;
use crate::reader::parsers::ParseError;
use crate::reader::types::try_parse_date;
use crate::reader::Expense;
use std::error::Error;

enum RowSecond {
    Expense {
        date: NaiveDate,
        title: String,
        amount: u32,
    },
    Empty,
}

fn parse_row_second(
    year: i32,
    date: Option<NaiveDate>,
    row: &[DataType],
) -> core::result::Result<RowSecond, Box<dyn Error>> {
    if row.len() == 0 {
        return Ok(RowSecond::Empty);
    }

    if row.len() < 4 {
        return Err("Non-empty row contains less than 4 elements.".into());
    }
    if row[0].is_empty() && row[1].is_empty() && row[2].is_empty() && row[3].is_empty() {
        return Ok(RowSecond::Empty);
    }
    if let DataType::String(val) = &row[0] {
        if val == "Дата" {
            return Ok(RowSecond::Empty);
        }
    }

    let parsed_date = match try_parse_date(year, &row[0]) {
        Some(val) => val,
        None => match date {
            Some(val) => val,
            None => return Err("No date set.".into()),
        },
    };

    let amount = match &row[3] {
        DataType::Int(val) => *val,
        DataType::Float(val) => val.round() as i64,
        DataType::String(val) => match val.parse::<f64>() {
            Ok(val) => val.round() as i64,
            Err(_) => return Err("Invalid string in amount cell.".into()),
        },
        DataType::Empty => 0,
        val => return Err(format!("Invalid datatype in amount cell: {}.", val).into()),
    };
    let amount = if amount < 0 {
        return Ok(RowSecond::Empty); // Один раз записал пополнение так
                                     // return Err("Invalid amount in amount cell.".into());
    } else {
        amount as u32
    };

    let title = match &row[1] {
        DataType::String(val) => val.clone(),
        DataType::Error(_) => return Err("Error value in title cell.".into()),
        _ => return Err("Unexpected datatype in title cell.".into()),
    };

    Ok(RowSecond::Expense {
        date: parsed_date,
        title,
        amount,
    })
}

// Ноябрь-декабрь 2020:
// - first column is date
// - second column is title
// - third column is category
// - fourth column is amount
pub fn parse_second(sheet_name: String, sheet: Range<DataType>) -> Result {
    let mut result: Vec<Expense> = Vec::new();
    let mut current_date: Option<NaiveDate> = Option::None;
    let year = match parse_year(&sheet_name) {
        Some(val) => val,
        None => return Ok(result),
    };

    for row in sheet.rows() {
        let parsed_row = match parse_row_second(year, current_date, row) {
            Ok(val) => val,
            Err(err) => {
                return Err(ParseError {
                    row: format!("{:?}", row),
                    message: err.to_string(),
                })
            }
        };
        let expense = match parsed_row {
            RowSecond::Expense {
                date,
                title,
                amount,
            } => Expense {
                date,
                title,
                amount,
            },
            RowSecond::Empty => continue,
        };
        current_date = Some(expense.date);

        result.push(expense);
    }

    Ok(result)
}
