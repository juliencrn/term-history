use std::collections::{BTreeMap, HashMap};

/// Collect binary names and count them
pub fn parse<'a>(
    content: &'a str,
    aliases: &'a HashMap<String, String>,
) -> BTreeMap<&'a str, usize> {
    let mut binary_count = BTreeMap::<&'a str, usize>::new();

    for line in content.lines() {
        if let Some(name) = extract_name(line.clone()) {
            // Check if the name is a known alias
            let name = match aliases.get(&name.to_string()) {
                Some(n) => n,
                None => name,
            };

            // Insert new name or increment its counter value
            match binary_count.get(&name) {
                Some(prev_count) => binary_count.insert(&name, prev_count + 1),
                None => binary_count.insert(&name, 1),
            };
        }
    }

    return binary_count;
}

/// From `: 1653599072:0;python3 matrix_rain.py` to `python3`
fn extract_name<'a>(line: &'a str) -> Option<&'a str> {
    let splitted_line = line.split(";").collect::<Vec<&str>>();
    if let Some(&cmd) = splitted_line.get(1) {
        let splitted_command = cmd.split(" ").collect::<Vec<&str>>();

        return splitted_command.get(0).as_ref().map(|x| **x);
    }
    None
}
