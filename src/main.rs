//! Calculate how many possible UTF8 strings there are.

use std::process::ExitCode;

use clap::Parser;
use rug::Integer;

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

    let mut result = Integer::ZERO;
    for (i, num) in utf8_counter().take(length).enumerate() {
        if cli.cumulative {
            result += num;
        } else {
            result = num;
        }

        if cli.verbose {
            eprintln!("{i:>d$}: {result}");
        }
    }

    println!("{}", with_thousands_separator(result));
    ExitCode::SUCCESS
}

/// Number of decimal digits of number.
const fn digits(n: usize) -> usize {
    if n > 0 { n.ilog10() as usize + 1 } else { 1 }
}

/// Add underscore separator at each 3 bytes from the end.
///
/// Should word with simple ASCII text, including digits.
#[must_use]
#[expect(clippy::needless_pass_by_value, reason = "Trait can be converted to reference by the caller")]
fn with_thousands_separator(item: impl ToString) -> String {
    fn add_thousands_separator(text: &str) -> String {
        let bytes = text
            .as_bytes()
            .rchunks(3)
            .rev()
            .enumerate()
            .flat_map(|(idx, bytes)| {
                let sep = (idx > 0).then_some(b'_');
                sep.into_iter().chain(bytes.iter().copied())
            })
            .collect();

        String::from_utf8(bytes).expect("non ASCII characters given")
    }

    add_thousands_separator(&item.to_string())
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use rug::ops::Pow;

    use super::*;

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1000))]

        #[test]
        fn digits_calculation(num: usize) {
            prop_assert_eq!(digits(num), num.to_string().len());
        }
    }

    #[test]
    fn thousands_separator() {
        assert_eq!(with_thousands_separator(0), "0");
        assert_eq!(with_thousands_separator(10), "10");
        assert_eq!(with_thousands_separator(1234), "1_234");
        assert_eq!(with_thousands_separator(654_321), "654_321");
        assert_eq!(with_thousands_separator(9_876_543_210_u64), "9_876_543_210");
        assert_eq!(with_thousands_separator(Integer::from(10).pow(25)), "10_000_000_000_000_000_000_000_000");
    }
}
