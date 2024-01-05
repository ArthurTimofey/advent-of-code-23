mod challanges;
mod utils;

fn main() {
    let challange = "a01";

    let data = utils::load_file::run(challange.to_string()).expect("Valid path");

    match challange {
        "a01" => challanges::a01::run(data),

        _ => println!("Challange not found!"),
    }
}
