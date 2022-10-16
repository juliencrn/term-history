use ansi_term::{Color, Colour};

use crate::cli;

fn colorize(color: Colour, text: &str, display_color: bool) -> String {
    match display_color {
        true => format!("{}", color.paint(text)),
        false => text.to_string(),
    }
}

fn format_line(rank: u8, (count, name): &(&usize, &str), colors: bool) -> String {
    let rank = if rank < 10 {
        format!(" {}", rank)
    } else {
        format!("{}", rank)
    };

    format!(
        "{}. {} ({} times)",
        rank,
        colorize(Color::Green, name, colors),
        count
    )
}

pub fn print_results(results: &Vec<(&usize, &str)>, args: &cli::Args) {
    let mut results = results.iter();

    for rank in 1..=args.head {
        if let Some(result) = results.next() {
            println!("{}", format_line(rank, result, args.colors))
        } else {
            break;
        }
    }
}
