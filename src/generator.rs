use std::collections::HashSet;
use std::io::{self, Write};

use rand::seq::SliceRandom;
use rand::{Rng, thread_rng};

const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const DIGITS: &str = "0123456789";
const SYMBOLS: &str = "!@#$%^&*()_-+=[]{}|;:,.<>?";

/// Configure information for password generate
pub struct GeneratorConfig<'a> {
    pub length: usize,
    pub count: usize,
    pub use_lower: bool,
    pub use_upper: bool,
    pub use_digits: bool,
    pub use_symbols: bool,
    pub custom_chars: Option<String>,
    pub output: Box<dyn Write + 'a>,
}

struct PasswordGenerator<'a> {
    final_charset: Vec<char>,
    required_sets: Vec<Vec<char>>,
    length: usize,
    count: usize,
    output: Box<dyn Write + 'a>,
}

impl<'a> PasswordGenerator<'a> {
    fn build_required_sets(config: &GeneratorConfig<'a>) -> Vec<Vec<char>> {
        let mut required_sets = Self::build_default_sets(config);

        if let Some(custom_chars) = &config.custom_chars {
            if !custom_chars.is_empty() {
                required_sets.push(custom_chars.chars().collect());
            }
        }
        required_sets
    }

    fn build_default_sets(config: &GeneratorConfig<'a>) -> Vec<Vec<char>> {
        [
            (config.use_lower, LOWERCASE),
            (config.use_upper, UPPERCASE),
            (config.use_digits, DIGITS),
            (config.use_symbols, SYMBOLS),
        ]
        .iter()
        .filter_map(|(use_set, set_str)| {
            if *use_set {
                Some(set_str.chars().collect())
            } else {
                None
            }
        })
        .collect()
    }

    fn new(config: GeneratorConfig<'a>) -> Result<Self, Box<dyn std::error::Error>> {
        let required_sets = Self::build_required_sets(&config);
        let final_charset: HashSet<char> = required_sets
            .iter()
            .flat_map(|set| set.iter())
            .cloned()
            .collect();

        if config.length < required_sets.len() {
            return Err(format!(
                "Password length ({}) is too short to include one character from each selected set ({}).",
                config.length,
                required_sets.len()
            ).into());
        }

        if final_charset.is_empty() {
            return Err(
                "No character sets selected. Please use --all or select at least one category."
                    .into(),
            );
        }

        Ok(Self {
            final_charset: final_charset.into_iter().collect(),
            required_sets,
            length: config.length,
            count: config.count,
            output: config.output,
        })
    }

    fn create_one_password(&self, rng: &mut impl Rng) -> String {
        let mut password_chars: Vec<char> = Vec::with_capacity(self.length);

        // Ensure at least one character from each required set.
        for req_set in &self.required_sets {
            password_chars.push(*req_set.choose(rng).unwrap());
        }

        // Fill the rest of the password with characters from the final character set.
        for _ in 0..(self.length.saturating_sub(password_chars.len())) {
            password_chars.push(*self.final_charset.choose(rng).unwrap());
        }

        password_chars.shuffle(rng);

        password_chars.into_iter().collect()
    }

    fn println(&mut self, s: &str) -> io::Result<()> {
        writeln!(self.output, "{}", s)
    }
}

/// Generate passwords and print them to stdout.
pub fn generate_passwords(config: GeneratorConfig) -> Result<(), Box<dyn std::error::Error>> {
    let mut generator = PasswordGenerator::new(config)?;
    let mut rng = thread_rng();

    for _ in 0..generator.count {
        let password = generator.create_one_password(&mut rng);
        generator.println(&password)?;
    }
    Ok(())
}
