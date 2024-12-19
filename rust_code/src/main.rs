mod serial;
mod utils;

use chrono::TimeDelta;
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

fn pair_up() {
    let base = read_json().expect("no errors");
    let ser = match base {
        Some(x) => Some(serial::some_serialize(x)),
        None => None,
    };

    for (k, v) in ser.unwrap().core_data() {
        let mut sum_time = vec![];
        println!("{}", k);
        for pair in v.chunks(2) {
            if pair.len() > 1 {
                let td = pair[1].give_date() - pair[0].give_date();
                sum_time.push(td);

                println!(
                    "hours {}, minutes {}",
                    td.num_hours(),
                    td.num_minutes() % 60
                )
            }
        }
        let thing: TimeDelta = sum_time.iter().sum();
        println!(
            "hours {}, minutes {}",
            thing.num_hours(),
            thing.num_minutes() % 60
        );
    }
}

fn main() {
    pair_up();
    // new_entry("start".to_string());
    // new_entry("stop".to_string());
}
