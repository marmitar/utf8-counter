//! Calculate how many possible UTF8 strings there are.

use std::process::ExitCode;

use clap::Parser;
use num_bigint::BigUint;
use num_format::{CustomFormat, ToFormattedString};

use utf8_counter::utf8_counter;

/// Calculate how many possible UTF8 strings there are.
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
struct Cli {
    /// String length to calculate UTF8 possibilies.
    length: usize,
    /// Consider all lengths from `0` to `LENGTH`.
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    cumulative: bool,
    /// Print intermediate values.
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    verbose: bool,
}

/// Binary entrypoint.
///
/// # Panics
///
/// On invalid formatter.
#[must_use]
pub fn main() -> ExitCode {
    let cli = Cli::parse();
    let Some(length) = cli.length.checked_add(1) else {
        eprintln!("value too big: {}", cli.length);
        return ExitCode::FAILURE;
    };

    let d = digits(cli.length);

    let mut last = BigUint::ZERO;
    for (i, num) in utf8_counter().take(length).enumerate() {
        if cli.cumulative {
            last += num;
        } else {
            last = num;
        }

        if cli.verbose {
            eprintln!("{i:>d$}: {last}");
        }
    }

    let format = CustomFormat::builder().separator("_").build().expect("valid formatter");
    println!("{}", last.to_formatted_string(&format));
    ExitCode::SUCCESS
}

/// Number of decimal digits of number.
const fn digits(n: usize) -> usize {
    if n > 0 { n.ilog10() as usize + 1 } else { 1 }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    proptest! {
        #![proptest_config(ProptestConfig {
            cases: 1000, ..ProptestConfig::default()
        })]
        #[test]
        fn digits_calculation(num: usize) {
            prop_assert_eq!(digits(num), num.to_string().len());
        }
    }
}
