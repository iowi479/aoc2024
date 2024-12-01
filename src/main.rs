mod challenge;
mod days;

use anyhow::Result;
fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <day>", args[0]);
        std::process::exit(1);
    }

    let day = args[1].parse::<u32>()?;
    days::run(day)
}
