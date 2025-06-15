//! Comparison with other numerical sequences.

use criterion::{AxisScale, BenchmarkId, Criterion, PlotConfiguration, Throughput, criterion_group, criterion_main};

use rug::integer::IntegerExt64;
use utf8_counter::{SequenceGenerator, cumulative, factorial, fibonacci, utf8_counter};

/// Select throughput measurement.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[expect(dead_code, reason = "Only one of the options is selected")]
enum ThroughputTarget {
    /// Number of iterations or elements yielded.
    Iterations,
    /// Number of bytes yielded over all elements.
    ///
    /// This is the sum of the minimum of bytes required to represent each element in the sequence.
    TotalBytes,
}

/// Calculate target throughput for the given sequence, as either number of elements or total bytes yielded.
fn calculate_throughput(mut sequence: impl SequenceGenerator, elements: usize, target: ThroughputTarget) -> Throughput {
    match target {
        ThroughputTarget::Iterations => {
            let total = elements.try_into().expect("cannot fit into u64");
            Throughput::Elements(total)
        }
        ThroughputTarget::TotalBytes => {
            let mut output_bytes = 0;
            sequence.until_n(elements, |_, element| {
                let bits = element.significant_bits_64();
                output_bytes += bits.div_ceil(8);
            });
            Throughput::Bytes(output_bytes)
        }
    }
}

/// Benchmark the extract of the `n`th element of a sequence.
///
/// Note: both construction and drop of the iteration are measured in each iteration of the benchmark. Additionaly,
/// all elements produced by the iterator are dropped inside the benchmark and are so considered in the measurement.
macro_rules! bench_sequence {
    ($bench:ident, $name:literal, $n:ident, $seq:expr, $tput:ident) => {
        $bench.throughput(calculate_throughput($seq, $n, $tput));
        $bench.bench_with_input(BenchmarkId::new($name, $n), &$n, |b, &size| {
            b.iter(move || {
                $seq.nth(size);
            });
        });
    };
}

/// Compare different large sequence iterators.
///
/// # Panics
///
/// On memory allocation failures (and possibly on internal conversion errors, which shouldn't happen).
pub fn sequence_iter(c: &mut Criterion) {
    let throughput = ThroughputTarget::TotalBytes;

    let mut group = c.benchmark_group("Sequence");
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));
    for n in [64, 128, 256, 512, 768, 1024, 1536, 2048] {
        // direct sequences
        bench_sequence!(group, "Factorial", n, factorial(), throughput);
        bench_sequence!(group, "Fibonacci", n, fibonacci(), throughput);
        bench_sequence!(group, "UTF8 Counter", n, utf8_counter(), throughput);

        // cumulative sequences
        bench_sequence!(group, "Cumulative Factorial", n, cumulative(factorial()), throughput);
        bench_sequence!(group, "Cumulative Fibonacci", n, cumulative(fibonacci()), throughput);
        bench_sequence!(group, "Cumulative UTF8 Counter", n, cumulative(utf8_counter()), throughput);
    }
    group.finish();
}

criterion_group!(benches, sequence_iter);
criterion_main!(benches);
