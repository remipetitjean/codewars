//! Write a simple parser that will parse and run Deadfish.
//!
//! Deadfish has 4 commands, each 1 character long:
//!
//! i increments the value (initially 0)
//! d decrements the value
//! s squares the value
//! o outputs the value into the return array
//! Invalid characters should be ignored.
use clap::{App, Arg};

pub struct Args {
    pub code: String,
}

impl Args {
    fn parse() -> Self {
        let matches = App::new("Deadfish")
            .arg(Arg::with_name("code"))
            .get_matches();
        let code = matches.value_of("code").unwrap().to_string();
        Self { code }
    }
}

fn main() {
    let args = Args::parse();
    let Args { code } = args;
    let result = deadfish(&code.to_string());
    println!("{:?}", result);
}

fn deadfish(code: &str) -> Vec<i32> {
    let mut i: i32 = 0;
    let mut output = Vec::<i32>::new();
    for c in code.chars() {
        match c {
            'i' => i = i + 1,
            'd' => i = i - 1,
            's' => i = i * i,
            'o' => output.push(i),
            _ => {}
        }
    }
    return output;
}

#[cfg(test)]
mod tests {
    use super::deadfish;
    use rstest::*;

    #[rstest(
        code,
        expected,
        case("iiisdoso", vec![8, 64]),
        case("iiisdosodddddiso", vec![8, 64, 3600]),
    )]
    fn test_deadfish(code: &str, expected: Vec<i32>) {
        assert_eq!(deadfish(code), expected);
    }
}
