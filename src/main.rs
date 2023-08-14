use std::io::{stdout, Result, Write};
use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<()> {
    const BAR_LAB: &str = "-\\|/";

    for prec in 0..=100 {
        print!(
            "\r {} \u{1b}[42m{}\u{1b}[0m [ {}% ]",
            BAR_LAB.chars().nth(prec % 4).unwrap(),
            " ".repeat(prec / 2),
            prec
        );
        stdout().flush()?;
        sleep(Duration::from_micros(20_000));
    }
    println!();
    Ok(())
}