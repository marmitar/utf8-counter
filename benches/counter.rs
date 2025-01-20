//! Comparison with other numerical sequences.

use criterion::{AxisScale, BenchmarkId, Criterion, PlotConfiguration, Throughput, criterion_group, criterion_main};

use utf8_counter::{SequenceGenerator, cumulative, factorial, fibonacci, utf8_counter};

/// Benchmark the extract of the `n`th element of a sequence.
///
/// Note: both construction and drop of the iteration are measured in each iteration of the benchmark. Additionaly,
/// all elements produced by the iterator are dropped inside the benchmark and are so considered in the measurement.
macro_rules! bench_iterator {
    ($bench:ident, $name:literal, $n:ident, $iter:expr) => {
        $bench.bench_with_input(BenchmarkId::new($name, $n), &$n, |b, &size| {
            b.iter(move || {
                $iter.nth(size);
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
    let mut group = c.benchmark_group("Sequence");
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));
    for n in [64, 128, 256, 512, 768, 1024, 1536, 2048] {
        let size = n.try_into().expect("cannot fit into u64");
        group.throughput(Throughput::Elements(size));

        // direct sequences
        bench_iterator!(group, "Factorial", n, factorial());
        bench_iterator!(group, "Fibonacci", n, fibonacci());
        bench_iterator!(group, "UTF8 Counter", n, utf8_counter());

        // cumulative sequences
        bench_iterator!(group, "Cumulative Factorial", n, cumulative(factorial()));
        bench_iterator!(group, "Cumulative Fibonacci", n, cumulative(fibonacci()));
        bench_iterator!(group, "Cumulative UTF8 Counter", n, cumulative(utf8_counter()));
    }
    group.finish();
}

criterion_group!(benches, sequence_iter);
criterion_main!(benches);
