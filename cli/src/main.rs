use clap::Parser;
use sqler::{format_sql, Config};
use std::fs;
use std::io::{self, Read};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Input file, reads from stdin if not provided
    #[arg(short, long)]
    input: Option<String>,

    /// Output file, writes to stdout if not provided
    #[arg(short, long)]
    output: Option<String>,

    /// Number of spaces for indentation
    #[arg(short, long, default_value = "2")]
    indent: usize,

    /// Maximum line length
    #[arg(short, long, default_value = "80")]
    max_length: usize,

    /// Check if input is properly formatted without modifying it
    #[arg(short, long)]
    check: bool,
}

fn read_input(input: Option<String>) -> io::Result<String> {
    match input {
        Some(path) => fs::read_to_string(path),
        None => {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer)?;
            Ok(buffer)
        }
    }
}

fn write_output(output: Option<String>, content: &str) -> io::Result<()> {
    match output {
        Some(path) => fs::write(path, content),
        None => {
            print!("{}", content);
            Ok(())
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // read input
    let config = Config {
        indent_char: ' '.to_string(),
        indent_width: cli.indent,
        max_line_length: cli.max_length,
    };

    let input = read_input(cli.input)?;

    // format sql
    match format_sql(&input, &config) {
        Ok(formatted) => {
            if cli.check {
                // exit with status 1 if input isn't properly formatted
                if formatted != input {
                    eprintln!("Input is not properly formatted");
                    std::process::exit(1);
                }
            } else {
                write_output(cli.output, &formatted)?;
            }
        }
        Err(e) => {
            eprintln!("Error formatting SQL: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}
