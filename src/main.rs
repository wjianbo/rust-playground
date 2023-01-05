extern crate regex;
use clap::Parser;
use regex::Regex;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    task: String,

    #[arg(short, long)]
    password: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    number: u8,
}

const TO_SEARCH: &'static str = "
On 2017-12-31, happy. On 2018-01-01, New Year.
";

fn regex_sample() {
    let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
    for caps in re.captures_iter(TO_SEARCH) {
        println!(
            "year: {}, month: {}, day: {}",
            caps.get(1).unwrap().as_str(),
            caps.get(2).unwrap().as_str(),
            caps.get(3).unwrap().as_str(),
        );
    }
}

fn named_regex_sample() {
    let re = Regex::new(r"(?x)(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})").unwrap();
    let caps = re.captures(TO_SEARCH).unwrap();
    println!(
        "year: {}, month: {}, day: {}",
        &caps["year"], &caps["month"], &caps["day"],
    );
    let after = re.replace_all(TO_SEARCH, "$month/$day/$year");
    println!("{}", after);
}

fn main() {
    // let args = Args::parse();
    // println!("{:?}", args);
    println!("Hello, world!");
    named_regex_sample();
}
