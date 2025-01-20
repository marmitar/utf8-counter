//! Calculate how many possible UTF8 strings there are.

mod counter;
mod sequence;

pub use counter::{Utf8Counter, utf8_counter};
pub use sequence::{SequenceGenerator, cumulative, factorial, fibonacci};
