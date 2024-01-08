fn get_end_start_numbers(str: String) -> String {
    let regex = regex::Regex::new(r"(\+d|one|two|three|four|five|six|seven|eight|nine)").unwrap();

    let mut test_str = String::from("");
    let mut start = String::from("");
    let mut end = String::from("");

    for (i, c) in str.chars().rev().enumerate() {
        match regex.find(test_str.as_str()) {
            Some(m) => {
                end = m.as_str().to_string();
                break;
            }
            None() => {}
        }
        test_str = c.to_string() + &test_str.to_owned()
    }

    println!("Test: {}", test_str);

    return str;
}

fn match_first_number_in_string(str: String) -> String {
    let regex = regex::Regex::new(r"(one|two|three|four|five|six|seven|eight|nine)").unwrap();

    match regex.find(&str) {
        Some(m) => match m.as_str() {
            "one" => "1".to_string(),
            "two" => "2".to_string(),
            "three" => "3".to_string(),
            "four" => "4".to_string(),
            "five" => "5".to_string(),
            "six" => "6".to_string(),
            "seven" => "7".to_string(),
            "eight" => "8".to_string(),
            "nine" => "9".to_string(),
            _ => m.as_str().to_string(),
        },
        None => "".to_string(),
    }
}

fn get_first_last_number(row: &str) -> i32 {
    if row.len() == 0 {
        return 0;
    }

    let first = match_first_number_in_string(row.to_string());
    let last = match_first_number_in_string(row.to_string());

    // println!("First: {}, Last: {}", first, last);

    format!("{}{}", first, last).parse::<i32>().unwrap()
}

pub fn run(data: String) {
    let rows = data.split("\n");

    get_end_start_numbers("one".to_string());

    let numbers: Vec<i32> = rows.map(|row| get_first_last_number(row)).collect();

    println!("Sum: {:?}", numbers.iter().sum::<i32>());
}
