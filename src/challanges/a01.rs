fn match_first_number_in_string(str: String, reverse: bool) -> String {
    let regex = regex::Regex::new(r"(one|two|three|four|five|six|seven|eight|nine)").unwrap();

    for (i, s) in str.chars().enumerate() {
        if reverse {
            println!("{} {}", str, str.chars().nth(str.len() - i - 1).unwrap());
        }
    }

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

    let rev_row = row.chars().rev().collect::<String>();

    println!("Row: {}, RevRow: {}", row, rev_row);

    let first = match_first_number_in_string(row.to_string(), false);
    let last = match_first_number_in_string(row.chars().rev().collect(), true);

    println!("First: {}, Last: {}", first, last);

    format!("{}{}", first, last).parse::<i32>().unwrap()
}

pub fn run(data: String) {
    let rows = data.split("\n");

    let numbers: Vec<i32> = rows.map(|row| get_first_last_number(row)).collect();

    println!("Sum: {:?}", numbers.iter().sum::<i32>());
}
