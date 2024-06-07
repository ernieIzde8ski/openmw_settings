use crate::parse;
use reqwest::blocking as reqwest;
use rstest::rstest;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

const SETTINGS_DEFAULT_URL_LOCATION: &'static str = "https://gitlab.com/OpenMW/openmw/-/raw/24d8accee7847f29a7115683eb9887bc157c4343/files/settings-default.cfg";

#[test]
fn parse_default_settings() {
    let path = PathBuf::from(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/",
        "assets/",
        "settings-default.cfg"
    ));

    let data = match path.exists() {
        true => {
            eprintln!("Reading from path: {path:?}");
            fs::read_to_string(&path).expect("should be able to read settings-default.cfg")
        }
        false => {
            let parent = path.parent().expect("should find the assets directory");
            if !parent.exists() {
                fs::create_dir_all(parent).expect("should be allowed to make directory");
            }

            let data = reqwest::get(SETTINGS_DEFAULT_URL_LOCATION)
                .expect("should get a response")
                .text()
                .expect("response should be parseable");

            fs::write(&path, &data).expect("should be able to write to assets directory");
            data
        }
    };

    parse(&data, false).expect("should parse correctly");
}

const DUPLICATES: &'static str = "
[Main]
duplicate-key = foo
duplicate-key = bar
";

#[test]
fn permit_overrides() {
    let left: HashMap<(String, String), String> =
        parse(DUPLICATES, true).expect("should parse correctly");

    let right: HashMap<(String, String), String> = HashMap::from([(
        ("Main".to_string(), "duplicate-key".to_string()),
        "bar".to_string(),
    )]);
    assert_eq!(left, HashMap::from(right))
}

#[rstest]
#[case(
    "[ unterminated-category",
    r#"L#1: unterminated category: "unterminated-category""#
)]
#[case("key = value", "L#1: empty category name")]
#[case(
    "[ Main ] unterminated-setting ",
    r#"L#1: unterminated setting name: "unterminated-setting""#
)]
#[case(DUPLICATES, "L#4: duplicate setting: Main.duplicate-key")]
fn parse_failures(#[case] value: &str, #[case] error_message: String) {
    let err = parse(value, false).expect_err("should fail to parse");
    assert_eq!(err.to_string(), error_message)
}
