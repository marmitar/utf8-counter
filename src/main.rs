//! Calculate how many possible UTF8 strings there are.

use std::borrow::Cow;
use std::process::ExitCode;

use clap::Parser;
use rug::Integer;

use utf8_counter::{SequenceGenerator, Utf8Counter, utf8_counter};

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

    let mut counter = utf8_counter();
    let result = last(&mut counter, cli.length, cli.cumulative, cli.verbose);
    println!("{}", with_thousands_separator(result));
    ExitCode::SUCCESS
}

/// Iterate up to `length` bytes possible.
fn last(counter: &mut Utf8Counter, length: usize, cumulative: bool, verbose: bool) -> Cow<'_, Integer> {
    let digits = min_digits(length);

    let mut sum = Integer::ZERO;
    counter.until_n(length, |i, current| {
        let target = if cumulative {
            sum += current;
            &sum
        } else {
            current
        };

        if verbose {
            eprintln!("{i:>digits$}: {target}");
        }
    });

    if cumulative {
        Cow::Owned(sum)
    } else {
        Cow::Borrowed(counter.current())
    }
}

/// Number of decimal digits of number.
const fn min_digits(n: usize) -> usize {
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
    use pretty_assertions::{assert_eq, assert_matches};
    use proptest::prelude::*;
    use rug::ops::Pow;

    use super::*;

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1000))]

        #[test]
        fn digits_calculation(num: usize) {
            prop_assert_eq!(min_digits(num), num.to_string().len());
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

    #[test]
    fn last_gets_correct_value() {
        let mut counter = utf8_counter();
        let result = last(&mut counter, 10, false, false);
        assert_matches!(result, Cow::Borrowed(_));
        assert_eq!(with_thousands_separator(result), "3_498_720_990_639_933_095_936");

        let mut counter = utf8_counter();
        let result = last(&mut counter, 10, true, false);
        assert_matches!(result, Cow::Owned(_));
        assert_eq!(with_thousands_separator(result), "3_523_090_815_835_167_586_305");
    }
}
