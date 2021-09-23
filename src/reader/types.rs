use calamine::DataType;
use chrono::*;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref MONTHS_RU: HashMap<&'static str, u8> = {
        let mut map = HashMap::new();
        map.insert("января", 1);
        map.insert("февраля", 2);
        map.insert("марта", 3);
        map.insert("апреля", 4);
        map.insert("мая", 5);
        map.insert("июня", 6);
        map.insert("июля", 7);
        map.insert("августа", 8);
        map.insert("сентября", 9);
        map.insert("октября", 10);
        map.insert("ноября", 11);
        map.insert("декабря", 12);
        map
    };
    static ref MONTHS_EN: HashMap<&'static str, u8> = {
        let mut map = HashMap::new();
        map.insert("january", 1);
        map.insert("february", 2);
        map.insert("march", 3);
        map.insert("april", 4);
        map.insert("may", 5);
        map.insert("june", 6);
        map.insert("july", 7);
        map.insert("august", 8);
        map.insert("september", 9);
        map.insert("october", 10);
        map.insert("november", 11);
        map.insert("december", 12);
        map
    };
}

fn float_hour_to_time(val: f64) -> NaiveTime {
    let s = (val * 24.0 * 60.0 * 60.0).round() as u32;
    let (m, s) = (s / 60, s % 60);
    let (h, m) = (m / 60, m % 60);
    NaiveTime::from_hms(h, m, s)
}

fn excel_to_datetime(excel_date: f64) -> NaiveDateTime {
    let start = NaiveDate::from_ymd(1900, 1, 1).num_days_from_ce();
    let result_num_days_from_ce = start + excel_date as i32 - 2;
    let result_date = NaiveDate::from_num_days_from_ce(result_num_days_from_ce);

    let time = float_hour_to_time(excel_date - excel_date.trunc());
    result_date.and_time(time)
}

fn string_to_date(year: i32, str: &str) -> Option<NaiveDate> {
    // TODO: декабрь-январь 2021 должен записывать январские даты в 2022
    let arr = str.split(" ");
    let vec: Vec<_> = arr.collect();

    if vec.len() < 2 {
        return None;
    }
    let day = *vec.get(0).unwrap();
    let day = match day.parse::<u8>() {
        Ok(val) => val,
        Err(_) => return None,
    };

    let month = vec.get(1).unwrap().to_lowercase();
    if let Some(month) = MONTHS_RU.get(month.as_str()) {
        return Some(NaiveDate::from_ymd(year, *month as u32, day as u32));
    }

    if let Some(month) = MONTHS_EN.get(month.as_str()) {
        return Some(NaiveDate::from_ymd(year, *month as u32, day as u32));
    }

    None
}

pub fn try_parse_date(year: i32, cell: &DataType) -> Option<NaiveDate> {
    match cell {
        DataType::DateTime(val) => Some(excel_to_datetime(*val).date()),
        DataType::Float(val) => Some(excel_to_datetime(*val).date()),
        DataType::String(val) => string_to_date(year, val.as_str()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn float_hour_to_time_test() {
        struct TestCase {
            excel: f64,
            expected_time: NaiveTime,
        }
        let tests: Vec<TestCase> = vec![TestCase {
            excel: 0.5515625,
            expected_time: NaiveTime::from_hms(13, 14, 15),
        }];

        for test in tests {
            assert_eq!(float_hour_to_time(test.excel), test.expected_time)
        }
    }

    #[test]
    fn excel_to_datetime_test() {
        struct TestCase {
            excel: f64,
            expected_datetime: NaiveDateTime,
        }
        let tests: Vec<TestCase> = vec![
            TestCase {
                excel: 42139.00966435185185185,
                expected_datetime: NaiveDate::from_ymd(2015, 5, 15).and_hms(0, 13, 55),
            },
            TestCase {
                excel: 1233.0,
                expected_datetime: NaiveDate::from_ymd(1903, 5, 17).and_hms(0, 0, 0),
            },
        ];

        for test in tests {
            assert_eq!(excel_to_datetime(test.excel), test.expected_datetime)
        }
    }

    #[test]
    fn try_parse_date_test() {
        struct TestCase {
            year: i32,
            excel: DataType,
            expected_date: Option<NaiveDate>,
        }
        let tests: Vec<TestCase> = vec![
            TestCase {
                year: 2021,
                excel: DataType::String(String::from("20 фЕвРаЛя")),
                expected_date: Some(NaiveDate::from_ymd(2021, 2, 20)),
            },
            TestCase {
                year: 2021,
                excel: DataType::DateTime(42139.00966435185185185),
                expected_date: Some(NaiveDate::from_ymd(2015, 5, 15)),
            },
            TestCase {
                year: 2020,
                excel: DataType::DateTime(44155.0),
                expected_date: Some(NaiveDate::from_ymd(2020, 11, 20)),
            },
        ];

        for test in tests {
            assert_eq!(try_parse_date(test.year, &test.excel), test.expected_date)
        }
    }
}
