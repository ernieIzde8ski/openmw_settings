use super::Settings;

use anyhow::{bail, Result};
use peeking_take_while::PeekableExt as _;

use std::iter::Peekable;
use std::path::Path;
use std::str::Chars;

pub fn parse(value: &str, override_existing_keys: bool) -> Result<Settings> {
    // ripped off from the source:
    //   https://gitlab.com/OpenMW/openmw/-/blob/master/components/settings/parser.cpp#L14
    let mut current_category: String = String::new();
    let mut resp: Settings = Settings::new();

    let lines = value.lines().map(str::chars).map(Iterator::peekable);
    let lines = (1..).zip(lines);

    for (i, mut line) in lines {
        if !skip_leading_whitespace(&mut line) {
            continue;
        };

        // skip comment
        if line.next_if_eq(&'#').is_some() {
            continue;
        }

        if line.next_if(|c| *c == '[').is_some() {
            let new_category_name: String = line
                .peeking_take_while(|c| *c != ']')
                .collect::<String>()
                .trim()
                .to_string();

            if !line.next().is_some_and(|c| c == ']') {
                bail!("L#{i}: unterminated category: {new_category_name:?}")
            }

            current_category = new_category_name;
        }

        if !skip_leading_whitespace(&mut line) {
            continue;
        }

        if current_category.is_empty() {
            bail!("L#{i}: empty category name")
        }

        let setting_name: String = line
            .peeking_take_while(|n| *n != '=')
            .collect::<String>()
            .trim()
            .to_string();

        if !line.next().is_some_and(|c| c == '=') {
            bail!("L#{i}: unterminated setting name: {setting_name:?}")
        }

        let setting_value: String = line.collect::<String>().trim().to_string();

        let previous_value: Option<String> = resp.insert(
            (current_category.clone(), setting_name.clone()),
            setting_value,
        );

        if !override_existing_keys && previous_value.is_some() {
            bail!("L#{i}: duplicate setting: {current_category}.{setting_name}")
        }
    }

    Ok(resp)
}

pub fn read_from_path(path: &Path, override_existing_keys: bool) -> Result<Settings> {
    let text = std::fs::read_to_string(path)?;
    parse(&text as &str, override_existing_keys)
}

/// Strips leading whitespace. Returns `true` if line contains additional characters.
fn skip_leading_whitespace(line: &mut Peekable<Chars>) -> bool {
    while let Some(_c) = line.next_if(|c| c.is_whitespace()) {}

    line.peek().is_some()
}
