use std::error::Error;

use calamine::{DataType, Range};
use chrono::NaiveDate;

use super::parsers::Result;
use crate::reader::common::parse_year;
use crate::reader::parsers::ParseError;
use crate::reader::types::try_parse_date;
use crate::reader::Expense;

enum RowFirst {
    Date(NaiveDate),
    Expense { title: String, amount: u32 },
    Empty,
}

fn parse_row_first(year: i32, row: &[DataType]) -> core::result::Result<RowFirst, Box<dyn Error>> {
    if row.len() == 0 {
        return Ok(RowFirst::Empty);
    }

    let parsed_date = try_parse_date(year, &row[0]);
    if let Some(parsed_date) = parsed_date {
        return Ok(RowFirst::Date(parsed_date));
    };

    if row.len() < 3 {
        return Err("Non-empty row contains less than 3 elements.".into());
    }
    if row[0].is_empty() && row[1].is_empty() && row[2].is_empty() {
        return Ok(RowFirst::Empty);
    }

    let amount = match &row[2] {
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
        return Err("Invalid amount in amount cell.".into());
    } else {
        amount as u32
    };

    let title = match &row[0] {
        DataType::String(val) => val.clone(),
        DataType::Error(_) => return Err("Error value in title cell.".into()),
        _ => return Err("Unexpected datatype in title cell.".into()),
    };

    Ok(RowFirst::Expense { title, amount })
}

// Start:
// - date on separate line
// - first column is title
// - second is category
// - third is amount
pub fn parse_first(sheet_name: String, sheet: Range<DataType>) -> Result {
    let mut result: Vec<Expense> = Vec::new();
    let mut date: Option<NaiveDate> = None;
    let year = match parse_year(&sheet_name) {
        Some(val) => val,
        None => return Ok(result),
    };

    for row in sheet.rows() {
        let parsed_row = match parse_row_first(year, row) {
            Ok(val) => val,
            Err(err) => {
                return Err(ParseError {
                    row: format!("{:?}", row),
                    message: err.to_string(),
                })
            }
        };
        let (title, amount) = match parsed_row {
            RowFirst::Expense { title, amount } => (title, amount),
            RowFirst::Date(val) => {
                date = Some(val);
                continue;
            }
            RowFirst::Empty => continue,
        };

        if let Some(date) = date {
            result.push(Expense {
                title,
                date,
                amount,
            });
        }
    }

    Ok(result)
}
