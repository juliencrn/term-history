use clap::Parser;
use std::fs;
use term_history::{cli, format, inverse_btree_map, parser};

fn main() {
    let args = cli::Args::parse();

    println!("{:?}", args);

    let history = fs::read_to_string(&args.history_file).expect("Could not read file");
    let binary_count = parser::parse(&history);

    // BTreeMap is natively sorted by key.
    // Inverse `key` with `value` to be sorted by count
    let sorted_binary_count = inverse_btree_map(&binary_count);

    // Then reverse the Vec to have the most used programs at the beginning
    let results = Vec::from_iter(sorted_binary_count)
        .into_iter()
        .rev()
        .collect::<Vec<(&usize, &str)>>();

    format::print_results(&results, &args);
}
