mod generator;

use clap::Parser;

use std::fs::File;
use std::io::{self, BufWriter, Write};

/// A command-line password generator.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Include all default character categories: lowercase, uppercase, digits, and symbols.
    #[arg(
        short = 'a',
        long,
        action = clap::ArgAction::SetTrue,
        help = "Include all default character categories:\nlowercase, uppercase, digits, and symbols."
    )]
    all: bool,

    /// Specify an additional set of characters to include in the password.
    #[arg(
        short = 'c',
        long,
        value_name = "CHARS",
        help = "Specify an additional set of characters to include in the password.",
        value_parser = parse_chars
    )]
    chars: Option<String>,

    /// The number of passwords to generate.
    #[arg(short = 'C', long, default_value_t = 1)]
    count: usize,

    /// The total length of the password to be generated.
    #[arg(short = 'L', long, default_value_t = 8, value_parser = parse_length)]
    length: usize,

    /// Include lowercase letters (a-z) in the password.
    #[arg(short = 'l', long, action = clap::ArgAction::SetTrue)]
    use_lower: bool,

    /// Include uppercase letters (A-Z) in the password.
    #[arg(short = 'u', long, action = clap::ArgAction::SetTrue)]
    use_upper: bool,

    /// Include digits (0-9) in the password.
    #[arg(short = 'd', long, action = clap::ArgAction::SetTrue)]
    use_digits: bool,

    /// Include symbols or special characters (e.g., !@#) in the password.
    #[arg(short = 's', long, action = clap::ArgAction::SetTrue)]
    use_symbols: bool,

    /// The output file path. If not specified, output to stdout.
    #[arg(short = 'o', long, value_name = "FILE")]
    output: Option<String>,
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

// Validate length option
fn parse_length(s: &str) -> Result<usize, String> {
    const MIN_LENGTH: usize = 6;

    let length: usize = s
        .parse()
        .map_err(|_| format!("'{}' is not a valid number.", s))?;

    if length < MIN_LENGTH {
        Err(format!(
            "The password length must be at least {}.",
            MIN_LENGTH
        ))
    } else {
        Ok(length)
    }
}

// Validate chars option
fn parse_chars(s: &str) -> Result<String, String> {
    if s.is_empty() {
        return Err(
            "The custom character set cannot be empty. Please provide at least one character."
                .to_string(),
        );
    }

    if s.chars().any(|c| c.is_whitespace()) {
        return Err("The custom character set cannot contain whitespace characters.".to_string());
    }

    if s.chars().any(|c| c.is_control()) {
        return Err("The custom character set cannot contain control characters.".to_string());
    }

    if !s.is_ascii() {
        return Err("The custom character set can only contain ASCII characters.".to_string());
    }

    Ok(s.to_string())
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Determine if any character type flag is explicitly set.
    let any_flag_set = args.use_lower
        || args.use_upper
        || args.use_digits
        || args.use_symbols
        || args.chars.is_some();

    // Use all categories if '--all' is specified, or if no specific category is chosen.
    let use_all = args.all || !any_flag_set;

    // Determine the output destination.
    // Use a BufWriter for better performance with file I/O.
    let output_path = args.output.clone(); // Clone for error reporting context
    let output: Box<dyn Write> = match args.output {
        Some(path) => {
            let file = File::create(path)?;
            Box::new(BufWriter::new(file))
        }
        None => Box::new(io::stdout()),
    };

    // Build GeneratorConfig from Args
    let config = generator::GeneratorConfig {
        length: args.length,
        count: args.count,
        use_lower: args.use_lower || use_all, // If `use_all` is true, enable this category.
        use_upper: args.use_upper || use_all,
        use_digits: args.use_digits || use_all,
        use_symbols: args.use_symbols || use_all,
        custom_chars: args.chars,
        output: output,
    };

    generator::generate_passwords(config).map_err(|err| -> Box<dyn std::error::Error> {
        if let Some(path) = output_path {
            // Add context if the error is related to file output
            Box::from(format!("Failed to write to file '{}': {}", path, err))
        } else {
            // For stdout, the original error is sufficient
            Box::from(format!("An output error occurred: {}", err))
        }
    })?;
    Ok(())
}
