use std::fs;
use tracing::debug;

pub(crate) fn get_input(day: u8, full_input: bool) -> String {
    let filepath = if full_input {
        format!("input/day{}.txt", day)
    } else {
        format!("input/day{}_test.txt", day)
    };
    debug!("reading file: {}", filepath);
    fs::read_to_string(filepath).expect("error parsing file")
}
