mod parse;

pub use parse::{parse, read_from_path};

#[cfg(test)]
mod tests;

use std::collections::HashMap;

pub type Settings = HashMap<(String, String), String>;
