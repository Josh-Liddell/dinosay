use clap::Parser;
use std::error::Error;
use std::fmt::Write;
use std::process;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    text: String,
}

fn main() {
    let args = Args::parse();

    let textbox = make_box(&args.text).unwrap_or_else(|e| {
        eprintln!("Error making textbox: {e}");
        process::exit(1);
    });

    println!("The text to dinosay:\n{}", textbox);
}

fn make_box(text: &str) -> Result<String, Box<dyn Error>> {
    let mut lines = Vec::new();
    let mut line = String::new();
    let words: Vec<&str> = text.split_whitespace().collect();

    for word in words {
        if line.len() == 0 {
            write!(&mut line, "{word}")?;
            continue;
        }
        if line.len() + (word.len() + 1) <= 41 {
            // append the word to the line
            write!(&mut line, " {word}")?;
        } else {
            // push the line and reset the line, with the word
            lines.push(line);
            line = word.to_string();
        }
    }

    if !line.is_empty() {
        lines.push(line);
    }

    let max = lines.iter().map(|line| line.len()).max().unwrap();
    let border = "-".repeat(max + 2) + "\n";
    let border_lines = lines
        .iter()
        .map(|line| {
            let mut spacer = String::new(); // can this be fixed?
            if line.len() < max {
                spacer = " ".repeat(max - line.len());
            }
            format!("|{line}{spacer}|\n")
        })
        .collect::<Vec<_>>()
        .join("");

    Ok(format!("{border}{border_lines}{border}"))
}
