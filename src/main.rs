/*
 * Terminal history lives in files
 * Examples:
 * - .zsh_history
 * - .bash_history
 *
 * Get the used history file:
 * $ echo $HISTFILE
 *
 * Also, some tools retrieve term history
 * - On Unix system:
 * $ history
 */

use clap::Parser;
use std::collections::{BTreeMap, HashMap};
use std::{fs, path, str};

#[derive(Parser, Debug)]
struct Cli {
    history_file: path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let history = fs::read_to_string(&args.history_file).expect("Could not read file");
    let binary_count = parse_history(&history);
    let sorted_binary_count = sort_by_values(&binary_count);
    let results = Vec::from_iter(sorted_binary_count);

    for (i, (count, name)) in results.into_iter().rev().enumerate() {
        println!("{}. {} ({} times)", i, name, count);
    }
}

/// Collect binary names and count them
fn parse_history(content: &str) -> BTreeMap<&str, usize> {
    let mut binary_count = BTreeMap::<&str, usize>::new();

    for line in content.lines() {
        if let Some(binary_name) = extract_binary_name(line) {
            // Insert new name or increment its counter value
            match binary_count.get(&binary_name) {
                Some(prev_count) => binary_count.insert(&binary_name, prev_count + 1),
                None => binary_count.insert(&binary_name, 1),
            };
        }
    }

    return binary_count;
}

/// BTreeMap are built-in sorted by keys
/// So reversing a <key: str, count: usize> results into a sorted by count BtreeMap ✨
fn sort_by_values<'a>(map: &'a BTreeMap<&str, usize>) -> BTreeMap<&'a usize, &'a str> {
    map.iter().map(|(k, v)| (v, k.clone())).collect()
}

/// From `: 1653599072:0;python3 matrix_rain.py` to `python3`
fn extract_binary_name(line: &str) -> Option<&str> {
    let splitted_line = line.split(";").collect::<Vec<&str>>();
    if let Some(&cmd) = splitted_line.get(1) {
        let splitted_command = cmd.split(" ").collect::<Vec<&str>>();

        return splitted_command.get(0).as_ref().map(|x| **x);
    }
    None
}
