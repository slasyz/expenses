pub fn parse_year(sheet_name: &String) -> Option<i32> {
    let arr = sheet_name.split(" ");
    let vec: Vec<_> = arr.collect();
    if vec.len() == 0 {
        return None;
    }

    let year = *vec.get(vec.len() - 1).unwrap();
    if let Ok(val) = year.parse::<i32>() {
        return Some(val);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_year_test() {
        struct TestCase {
            val: String,
            expected_year: Option<i32>,
        }
        let tests: Vec<TestCase> = vec![
            TestCase {
                val: String::from("Февраль 2021"),
                expected_year: Some(2021),
            },
            TestCase {
                val: String::from("Таиланд"),
                expected_year: None,
            },
        ];

        for test in tests {
            assert_eq!(parse_year(&test.val), test.expected_year)
        }
    }
}
