use std::fmt::{Display, Debug, Formatter};
use std::time::{Duration, Instant};

pub const SAMPLES: usize = 15;

/// A function time benchmark.
#[derive(Debug)]
pub struct Benchmark<T> {
    time: Duration,
    result: T
}

impl<T: Debug> Display for Benchmark<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Returned '{:?}' in {:?}.", self.result, self.time)
    }
}

impl<T: Debug> Benchmark<T> {
    /// Returns the benchmark message.
    pub fn msg(&self, label: &str) -> String {
        format!("{label}: {}", self)
    }
}

/// Returns the execution time of a given function `F`.
pub fn exec_time<F, T>(f: &F) -> Benchmark<T>
where
    F: Fn() -> T,
{
    let start = Instant::now();
    let result = f();
    let time = start.elapsed();
    Benchmark { time, result }
}

/// Returns the median benchmark of the given function `F`.
pub fn med_exec_time<F, T>(f: &F) -> Benchmark<T>
where
    F: Fn() -> T,
{
    let mut benchmarks = (0..SAMPLES)
        .map(|_| exec_time(f))
        .collect::<Vec<_>>();
    benchmarks.sort_by_key(|a| a.time);

    let median_index = (SAMPLES / 2) + 1;
    benchmarks.swap_remove(median_index)
}
