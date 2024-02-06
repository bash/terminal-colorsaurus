use clap::Parser;
use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::OpenOptions;
use std::hint::black_box;
use std::io::{self, Write as _};
use std::time::{Duration, Instant};
use terminal_colorsaurus::{color_scheme, QueryOptions, Result};

#[derive(Parser, Debug)]
struct Args {
    term: String,
    #[arg(short = 'I', long, default_value_t = 10_000)]
    iterations: u32,
}

fn main() -> Result<()> {
    let args = Args::parse();

    #[cfg(debug_assertions)]
    eprintln!(
        "{}{}",
        style("warning").yellow().bold(),
        style(": you should run this example in release mode").bold()
    );

    eprintln!(
        "Running benchmark with {} iterations",
        style(args.iterations).bold()
    );

    let bar = ProgressBar::new(args.iterations as u64)
        .with_style(ProgressStyle::default_bar().progress_chars("██░"));

    let measurements = (0..args.iterations)
        .into_iter()
        .map(|_| bench())
        .inspect(|_| bar.inc(1))
        .collect::<Result<Vec<_>>>()?;
    bar.finish();

    save_results(&measurements, args.term)?;

    Ok(())
}

fn bench() -> Result<Duration> {
    let start = Instant::now();
    let _ = black_box(color_scheme(QueryOptions::default()))?;
    Ok(start.elapsed())
}

fn save_results(results: &[Duration], term: String) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("benchmark.tsv")?;
    writeln!(
        file,
        "{}\t{}\t{}\t{}\t{}",
        term,
        results.len(),
        results.iter().min().unwrap().as_nanos(),
        results.iter().max().unwrap().as_nanos(),
        (results.iter().sum::<Duration>() / results.len() as u32).as_nanos(),
    )?;
    Ok(())
}
