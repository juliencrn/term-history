pub mod cli;
pub mod format;
pub mod parser;

use std::collections::BTreeMap;

pub fn inverse_btree_map<'a>(map: &'a BTreeMap<&str, usize>) -> BTreeMap<&'a usize, &'a str> {
    map.iter().map(|(k, v)| (v, k.clone())).collect()
}
