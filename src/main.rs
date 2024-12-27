use clap::Parser;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use aoc2024::*;

#[derive(Parser, Debug)]
struct Cli {
    /// Specify the day number
    #[arg(long)]
    day: u8,

    /// Whether to show trace logs
    #[arg(long, short)]
    trace: bool,

    /// Whether to show debug logs
    #[arg(long, short)]
    debug: bool,

    /// Whether to run the full file or just the example
    #[arg(long, short, default_value = "false")]
    full: bool,
}

fn setup_logging(trace: bool, debug: bool) {
    // Determine the logging level based on flags
    let max_level = if trace {
        Level::TRACE
    } else if debug {
        Level::DEBUG
    } else {
        Level::INFO
    };

    // Initialize the logging subscriber
    FmtSubscriber::builder()
        .with_max_level(max_level)
        .with_target(debug) // Enable log targets only in debug mode
        .init();
}

fn main() {
    let args = Cli::parse();
    setup_logging(args.trace, args.debug);

    info!("Starting the application");

    match args.day {
        1 => day1::run(!args.full),
        2 => day2::run(!args.full),
        3 => day3::run(!args.full),
        4 => day4::run(!args.full),
        5 => day5::run(!args.full),
        6 => day6::run(!args.full),
        7 => day7::run(!args.full),
        8 => day8::run(!args.full),
        9 => day9::run(!args.full),
        _ => println!("Day not implemented."),
    }
}
