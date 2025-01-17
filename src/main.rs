//! Calculate how many possible UTF8 strings there are.

// Additional Errors
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(clippy::all)]
#![deny(clippy::allow_attributes)]
#![deny(clippy::allow_attributes_without_reason)]
#![deny(clippy::lossy_float_literal)]
// More Warnings
#![warn(clippy::alloc_instead_of_core)]
#![warn(clippy::as_underscore)]
#![warn(clippy::clone_on_ref_ptr)]
#![warn(clippy::create_dir)]
#![warn(clippy::decimal_literal_representation)]
#![warn(clippy::empty_drop)]
#![warn(clippy::exhaustive_enums)]
#![warn(clippy::exit)]
#![warn(clippy::filetype_is_file)]
#![warn(clippy::float_cmp_const)]
#![warn(clippy::fn_to_numeric_cast_any)]
#![warn(clippy::format_push_string)]
#![warn(clippy::if_then_some_else_none)]
#![warn(clippy::infinite_loop)]
#![warn(clippy::integer_division_remainder_used)]
#![warn(clippy::map_err_ignore)]
#![warn(clippy::map_with_unused_argument_over_ranges)]
#![warn(clippy::mem_forget)]
#![warn(clippy::missing_assert_message)]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(clippy::mixed_read_write_in_expression)]
#![warn(clippy::multiple_inherent_impl)]
#![warn(clippy::multiple_unsafe_ops_per_block)]
#![warn(clippy::mutex_atomic)]
#![warn(clippy::needless_raw_strings)]
#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]
#![warn(clippy::non_zero_suggestions)]
#![warn(clippy::panic_in_result_fn)]
#![warn(clippy::redundant_type_annotations)]
#![warn(clippy::ref_patterns)]
#![warn(clippy::rest_pat_in_fully_bound_structs)]
#![warn(clippy::self_named_module_files)]
#![warn(clippy::semicolon_outside_block)]
#![warn(clippy::str_to_string)]
#![warn(clippy::string_to_string)]
#![warn(clippy::tests_outside_test_module)]
#![warn(clippy::try_err)]
#![warn(clippy::undocumented_unsafe_blocks)]
#![warn(clippy::unnecessary_safety_comment)]
#![warn(clippy::unnecessary_safety_doc)]
#![warn(clippy::unneeded_field_pattern)]
#![warn(clippy::unseparated_literal_suffix)]
#![warn(clippy::unused_result_ok)]
#![warn(clippy::unwrap_in_result)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::wildcard_enum_match_arm)]
#![warn(clippy::unnecessary_self_imports)]

use std::process::ExitCode;

use clap::Parser;

mod counter;

use counter::utf8_counter;
use num_bigint::BigUint;

/// Calculate how many possible UTF8 strings there are.
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
struct Cli {
    /// String length to calculate UTF8 possibilies.
    length: usize,
    /// Print intermediate values.
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    verbose: bool,
}

/// Binary entrypoint.
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
        if cli.verbose {
            eprintln!("{i:>d$}: {num}");
        }
        last = num;
    }

    println!("{last}");
    ExitCode::SUCCESS
}

/// Number of decimal digits of number.
const fn digits(n: usize) -> usize {
    if n > 0 { n.ilog10() as usize + 1 } else { 1 }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn digits_calculation() {
        let m = usize::MAX;
        #[rustfmt::skip]
        let cases = [0, 1, 2, 5, 9, 10, 11, 55, 99, 100, 101, m - 5, m - 4, m - 3, m - 2, m - 1, m];

        for num in cases {
            assert_eq!(digits(num), num.to_string().len());
        }
    }
}
