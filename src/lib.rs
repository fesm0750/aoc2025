use std::{fmt::Display, time::Instant};

pub mod day02;

/// An execution and benchmarking harness for multi-stage puzzle solutions.
///
/// `Runner<T>` is a fluent builder that manages parsed data, execution timers, and
/// tabular `stdout` formatting across sequential execution stages. It enforces a linear
/// lifecycle via move semantics: initialization and parsing must precede stage execution,
/// and the pipeline terminates with [`finish`](Runner::finish).
///
/// It uses wall-clock timers via [`Instant::now`] to measure cumulative and step-level
/// execution times.
///
/// # Type Parameters
///
/// * `T` - The type of the structured dataset produced by the parser and consumed by each
///   stage.
pub struct Runner<T> {
    data: T,
    total_start: Instant,
}

impl Runner<()> {
    /// Constructs a new `Runner<T>` by loading input from the filesystem and executing
    /// the parser.
    ///
    /// # Arguments
    ///
    /// * `name` - A descriptive label for the challenge (e.g., `"Cube Conundrum"`).
    /// * `path` - Path to the input file.
    /// * `parser` - A closure or function pointer converting the raw file slice into `T`.
    ///
    /// # Panics
    ///
    /// Panics if `path` cannot be read from the filesystem via or `parser` panics.
    pub fn init<T>(name: &str, path: &str, parser: impl FnOnce(&str) -> T) -> Runner<T> {
        let raw = std::fs::read_to_string(path).unwrap_or_else(|_| panic!("Missing {path}"));
        let (_, day) = path.rsplit_once('/').unwrap_or(("", ""));

        let total_start = Instant::now();
        let start = Instant::now();
        let data = parser(&raw);
        let elapsed = start.elapsed();

        println!("\n{:^39}", format!("{day} {name}"));
        println!("|   step   |     time     |    answer");
        println!("| -------- | ------------ | ------------ ");
        println!("| Parse    | {:>12.4?} |", elapsed);

        Runner { data, total_start }
    }
}

impl<T> Runner<T> {
    /// Executes an individual computation stage `f`, times, and logs the result to
    /// stdout.
    ///
    /// Consumes and returns `self` by value to permit method chaining.
    pub fn part<R: Display>(self, name: &str, f: impl FnOnce(&T) -> R) -> Self {
        let start = Instant::now();
        let res = f(&self.data);
        println!("| {name:8} | {:>12.4?} | {res}", start.elapsed());

        self
    }

    /// Executes and logs part 1 solution.
    pub fn part1<R: Display>(self, f: impl FnOnce(&T) -> R) -> Self {
        self.part("pt1", f)
    }

    /// Executes and logs part 2 solution.
    pub fn part2<R: Display>(self, f: impl FnOnce(&T) -> R) -> Self {
        self.part("pt2", f)
    }

    /// Concludes the run, printing the total elapsed time and table footer.
    pub fn finish(self) {
        println!("| -------- | ------------ | ------------");
        println!("| Total    | {:>12.4?} |\n", self.total_start.elapsed());
    }
}
