#[macro_use]
extern crate lazy_static;
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

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"(?x)(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})").unwrap();
    static ref EMAIL_RE: Regex =
        Regex::new(r"(?x)^\w+@(:gmail|163|qq)\.(?:com|cn|com\.cn|net)$").unwrap();
}

fn regex_date(text: &str) -> regex::Captures {
    RE.captures(text).unwrap()
}

fn regex_email(text: &str) -> bool {
    EMAIL_RE.is_match(text)
}

fn lazy_static_sample() {
    let caps = regex_date("2018-01-01");
    assert_eq!("2018", &caps["year"]);
    let after = RE.replace_all(TO_SEARCH, "$month/$day/$year");
    println!("{}", after);
    println!("{}", regex_email("alex@gmail.com"));
}

fn main() {
    // let args = Args::parse();
    // println!("{:?}", args);
    lazy_static_sample();
}
