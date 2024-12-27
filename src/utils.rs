use std::fs;

pub(crate) fn get_input(day: u8, full_input: bool) -> String {
    let filepath = if full_input {
        format!("input/day_{}.txt", day)
    } else {
        format!("input/day_{}_test.txt", day)
    };
    fs::read_to_string(filepath).expect("error parsing file")
}
