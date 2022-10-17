use anyhow;
use regex::Regex;

use std::{collections::HashMap, process::Command};

/// For now, we only support zsh
/// @dev: wrap get_aliases and return an empty HashMap if anything goes wrong
pub fn get() -> HashMap<String, String> {
    let aliases = match get_zsh_aliases() {
        Ok(aliases) => aliases,
        Err(e) => {
            println!("Warn: Could not fetch aliases\n{:?}", e);
            let map: HashMap<String, String> = HashMap::new();
            map
        }
    };

    // Reduce list by remove nested alias
    let aliases = aliases
        .iter()
        .map(|tuple| recursive_replace(tuple.clone(), &aliases))
        .map(|(key, val)| (key.to_string(), val.to_string()))
        .collect::<HashMap<String, String>>();

    return aliases;
}

fn get_zsh_aliases() -> anyhow::Result<HashMap<String, String>> {
    let mut shell = Command::new("zsh");
    let command = shell.arg("-c").arg(". ~/.zshrc; alias");
    let alias_raw = String::from_utf8(command.output()?.stdout)?;
    let re_valid_name = Regex::new(r"^\w+=").unwrap();
    let re_start_with_quote = Regex::new(r"^'").unwrap();
    let mut map: HashMap<String, String> = HashMap::new();

    for line in alias_raw.clone().lines() {
        if re_valid_name.is_match(line) {
            let split_line = line.split("=").collect::<Vec<&str>>();

            if let Some(&alias) = split_line.get(0) {
                if let Some(&command) = split_line.get(1) {
                    let split_cmd = command.split(" ").collect::<Vec<&str>>();

                    if let Some(&program) = split_cmd.get(0) {
                        // Some scripts are wrapped in quote, remove it
                        let program = match re_start_with_quote.is_match(program) {
                            true => &program[1..program.len()],
                            false => program,
                        };

                        map.insert(alias.to_owned(), program.to_owned());
                    }
                }
            }
        }
    }

    return Ok(map);
}

/// replace alias by the program name, nested way
fn recursive_replace<'a>(
    initial: (&'a String, &'a String),
    aliases: &'a HashMap<String, String>,
) -> (&'a String, &'a String) {
    let mut result = initial;

    loop {
        // replace alias by the program name
        let new = match aliases.get(&result.1.to_owned()) {
            Some(new_program) => (result.0, new_program),
            None => result,
        };

        if new == result {
            break;
        }

        result = new;
    }

    result
}
