mod serial;
mod utils;

use utils::{add_new_entry, read_json, write_json};

fn new_entry(entry_type: String) {
    let base = read_json().expect("no errors");
    let ser = match base {
        Some(x) => Some(serial::some_serialize(x)),
        None => None,
    };
    let new_stuff = add_new_entry(entry_type, ser);
    let out = serial::some_deserialize(new_stuff);
    let _ = write_json(&out);
}

fn main() {
    new_entry("start".to_string());
    new_entry("stop".to_string());
}
