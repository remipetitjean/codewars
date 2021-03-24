use clap::{App, Arg};

pub struct Args {
    pub word: String,
    pub ending: String,
}

impl Args {
    fn parse() -> Self {
        let matches = App::new("string_ends_with")
            .arg(Arg::with_name("word"))
            .arg(Arg::with_name("ending"))
            .get_matches();
        let word = matches.value_of("word").unwrap().to_string();
        let ending = matches.value_of("ending").unwrap().to_string();
        Self { word, ending }
    }
}

fn main() {
    let args = Args::parse();
    let Args { word, ending } = args;
    let result = string_starts_with(&word.to_string(), &ending.to_string());
    println!("yo {} | {} | {}", word, ending, result);
}

fn string_starts_with(word: &str, ending: &str) -> bool {
    str::ends_with(word, ending)
}

#[cfg(test)]
mod tests {
    use super::string_starts_with;
    use rstest::*;

    #[rstest(
        word,
        ending,
        expected,
        case("test", "t", true),
        case("test", "a", false)
    )]
    fn test_string_starts_with(word: &str, ending: &str, expected: bool) {
        assert_eq!(string_starts_with(word, ending), expected);
    }
}
