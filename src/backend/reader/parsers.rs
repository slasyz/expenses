use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::BufReader;

use calamine::{DataType, Range, Reader, Xlsx, XlsxError};
use lazy_static::lazy_static;

use crate::reader::parser_first::parse_first;
use crate::reader::parser_second::parse_second;
use crate::reader::parser_third::parse_third;
use crate::reader::Expense;
use std::error::Error;

#[derive(Debug)]
pub struct ParseError {
    pub row: String,
    pub message: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error parsing row {}: {}", self.row, self.message)
    }
}

impl Error for ParseError {}

impl From<XlsxError> for ParseError {
    fn from(err: XlsxError) -> Self {
        ParseError::msg(err.to_string())
    }
}

impl ParseError {
    fn msg(val: String) -> ParseError {
        ParseError {
            message: val,
            row: String::from("<None>"),
        }
    }
}

pub type Result = core::result::Result<Vec<Expense>, ParseError>;

type Parser = fn(name: String, sheet: Range<DataType>) -> Result;

lazy_static! {
    static ref PARSERS: HashMap<&'static str, Parser> = {
        let mut map = HashMap::new();
        map.insert("", Parser::from(parse_first));
        map.insert("Ноябрь-декабрь 2020", Parser::from(parse_second));
        map.insert("Август-сентябрь 2021", Parser::from(parse_third));
        map
    };
}

pub fn parse_sheets(workbook: &mut Xlsx<BufReader<File>>) -> Result {
    let mut result: Vec<Expense> = Vec::new();
    let names: Vec<String> = Vec::from(workbook.sheet_names());
    let mut parser = PARSERS.get("").unwrap();

    for sheet_name in names {
        let sheet = workbook
            .worksheet_range(sheet_name.as_str())
            .ok_or(ParseError::msg(format!("Cannot find '{}'", sheet_name)))??;

        println!("reading sheet '{}'", sheet_name);

        parser = match PARSERS.get(sheet_name.as_str()) {
            Some(parser) => parser,
            None => parser,
        };

        let mut sheet_expenses: Vec<Expense> = parser(sheet_name, sheet)?;
        for expense in &sheet_expenses {
            println!("- {:?}", expense);
        }
        result.append(&mut sheet_expenses);
    }

    Ok(result)
}
