fn find_first_number(str: &str) -> String {
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

fn match_last_number_in_string(str: String) -> String {
    for (i, c) in str.chars().rev().enumerate() {
        let number = find_first_number(&str[str.len() - i..str.len()].to_string());

        if number != "" {
            return number;
        } else if c.is_numeric() {
            return c.to_string();
        }
    }

    return "".to_string();
}

fn match_first_number_in_string(str: String) -> String {
    for (i, c) in str.chars().enumerate() {
        let number = find_first_number(&str[0..i].to_string());
        if number != "" {
            return number;
        } else if c.is_numeric() {
            return c.to_string();
        }
    }

    return "".to_string();
}

fn get_first_last_number(row: &str) -> i32 {
    if row.len() == 0 {
        return 0;
    }

    let range = 0..row.len();
    let prev_range = (0..row.len()).rev();

    println!("range: {:?}", prev_range);

    let first = match_first_number_in_string(row.to_string());
    let last = match_last_number_in_string(row.to_string());

    format!("{}{}", first, last).parse::<i32>().unwrap()
}

pub fn run(data: String) {
    let rows = data.split("\n");

    let numbers: Vec<i32> = rows.map(|row| get_first_last_number(row)).collect();

    println!("Sum: {:?}", numbers.iter().sum::<i32>());
}
