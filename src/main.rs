use anyhow::{anyhow, Result};
use std::io::{self, Read};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Options {
    #[structopt(short, long, help = "Convert text to lowercase")]
    lowercase: bool,

    #[structopt(short, long, help = "Convert text to uppercase")]
    uppercase: bool,

    #[structopt(short, long, help = "Reverse text")]
    reverse: bool,

    #[structopt(short, long, help = "Count words")]
    words: bool,

    #[structopt(short, long, help = "Count characters")]
    chars: bool,

    #[structopt(help = "The input text")]
    input: Option<String>,
}

struct TextOperations {
    input: String,
}

impl TextOperations {
    pub fn new(input: String) -> TextOperations {
        TextOperations { input }
    }

    pub fn lowercase(&self) -> String {
        self.input.to_lowercase()
    }

    pub fn uppercase(&self) -> String {
        self.input.to_uppercase()
    }

    pub fn reverse_text(&self) -> String {
        self.input.chars().rev().collect()
    }

    pub fn count_words(&self) -> usize {
        self.input.split_whitespace().count()
    }

    pub fn count_characters(&self) -> usize {
        self.input.chars().count()
    }
}

pub fn read_from_stdin() -> String {
    let mut buffer = String::new();
    let _ = io::stdin()
        .read_to_string(&mut buffer)
        .map_err(|err| anyhow!("An error occurred while reading from stdin: {err}"));
    buffer
}

fn main() -> Result<()> {
    let options = Options::from_args();

    let opers = TextOperations::new(options.input.unwrap_or_else(read_from_stdin));

    if options.lowercase {
        println!("{}", opers.lowercase());
    } else if options.uppercase {
        println!("{}", opers.uppercase());
    } else if options.reverse {
        println!("{}", opers.reverse_text());
    } else if options.words {
        println!("{}", opers.count_words());
    } else if options.chars {
        println!("{}", opers.count_characters());
    } else {
        eprintln!("Invalid options.");
        Options::clap().print_help()?;
        println!("\n");
        return Err(anyhow!("Please provide a correct argument."));
    }

    Ok(())
}
