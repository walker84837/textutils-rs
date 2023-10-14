use std::{
    error::Error,
    io::{self, Read},
};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Options {
    /// Convert text to lowercase
    #[structopt(short, long)]
    lowercase: bool,

    /// Convert text to uppercase
    #[structopt(short, long)]
    uppercase: bool,

    /// Reverse text
    #[structopt(short, long)]
    reverse: bool,

    /// Count words
    #[structopt(short, long)]
    wordcount: bool,

    /// Count characters
    #[structopt(short, long)]
    charcount: bool,

    /// Display this help message
    #[structopt(short, long)]
    help: bool,

    /// The input text
    #[structopt(name = "TEXT")]
    input: Option<String>,
}

fn to_lowercase(input: &str) -> String {
    input.to_lowercase()
}

fn to_uppercase(input: &str) -> String {
    input.to_uppercase()
}

fn reverse_text(input: &str) -> String {
    input.chars().rev().collect()
}

fn count_words(input: &str) -> usize {
    input.split_whitespace().count()
}

fn count_characters(input: &str) -> usize {
    input.chars().count()
}

fn main() -> Result<(), Box<dyn Error>> {
    let options = Options::from_args();

    if options.help {
        Options::clap().print_help()?;
    } else if options.lowercase {
        let input = options.input.unwrap_or_else(|| {
            let mut buffer = String::new();
            io::stdin()
                .read_to_string(&mut buffer)
                .expect("Failed to read from stdin");
            buffer
        });
        println!("{}", to_lowercase(&input));
    } else if options.uppercase {
        let input = options.input.unwrap_or_else(|| {
            let mut buffer = String::new();
            io::stdin()
                .read_to_string(&mut buffer)
                .expect("Failed to read from stdin");
            buffer
        });
        println!("{}", to_uppercase(&input));
    } else if options.reverse {
        let input = options.input.unwrap_or_else(|| {
            let mut buffer = String::new();
            io::stdin()
                .read_to_string(&mut buffer)
                .expect("Failed to read from stdin");
            buffer
        });
        println!("{}", reverse_text(&input));
    } else if options.wordcount {
        let input = options.input.unwrap_or_else(|| {
            let mut buffer = String::new();
            io::stdin()
                .read_to_string(&mut buffer)
                .expect("Failed to read from stdin");
            buffer
        });
        println!("{}", count_words(&input));
    } else if options.charcount {
        let input = options.input.unwrap_or_else(|| {
            let mut buffer = String::new();
            io::stdin()
                .read_to_string(&mut buffer)
                .expect("Failed to read from stdin");
            buffer
        });
        println!("{}", count_characters(&input));
    } else {
        eprintln!("Invalid option");
        Options::clap().print_help()?;
    }

    Ok(())
}
