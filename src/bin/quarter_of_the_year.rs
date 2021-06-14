//! Given a month as an integer from 1 to 12, return to which quarter of the year it belongs as an integer number.
//!
//! For example: month 2 (February), is part of the first quarter; month 6 (June), is part of the second quarter; and month 11 (November), is part of the fourth quarter.
use clap::{App, Arg};

pub struct Args {
    pub month: u8,
}

impl Args {
    fn parse() -> Self {
        let matches = App::new("Quarter of the year")
            .arg(Arg::with_name("month"))
            .get_matches();
        let month = matches
            .value_of("month")
            .unwrap()
            .to_string()
            .parse::<u8>()
            .unwrap();
        Self { month }
    }
}

fn main() {
    let args = Args::parse();
    let Args { month } = args;
    let quarter = quarter_of(month);
    println!("{:?}", month);
    println!("{:?}", quarter);
}

fn quarter_of(month: u8) -> u8 {
    (month - 1) / 3 + 1
}

#[cfg(test)]
mod tests {
    use super::quarter_of;
    use rstest::*;

    #[rstest(
        month,
        quarter,
        case(1, 1),
        case(2, 1),
        case(3, 1),
        case(4, 2),
        case(5, 2),
        case(6, 2),
        case(7, 3),
        case(8, 3),
        case(9, 3),
        case(10, 4),
        case(11, 4),
        case(12, 4)
    )]
    fn test_quarter_of(month: u8, quarter: u8) {
        assert_eq!(quarter_of(month), quarter);
    }
}
