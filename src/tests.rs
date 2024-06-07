use crate::parse;
use reqwest::blocking as reqwest;
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

    parse(&data, false).expect("should parse correctly!");
}
