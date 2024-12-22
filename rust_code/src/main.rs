mod serial;
mod utils;

use chrono::TimeDelta;
use utils::{add_new_entry, read_json, write_json};

const TAB_SPACE: &str = "  ";

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

    let the_data = ser.unwrap().core_data();
    let mut keys: Vec<String> = the_data.clone().into_keys().collect();
    keys.sort_unstable();

    for k in keys {
        let v = the_data.get(&k).unwrap();
        let mut sum_time = vec![];
        // print the date
        println!("{}", k);
        for pair in v.chunks(2) {
            if pair.len() > 1 {
                let td = pair[1].give_date() - pair[0].give_date();
                sum_time.push(td);

                // print the start-stop pair
                println!(
                    "{}{}{}:{}",
                    TAB_SPACE,
                    TAB_SPACE,
                    td.num_hours(),
                    td.num_minutes() % 60
                )
            }
        }
        let thing: TimeDelta = sum_time.iter().sum();
        // print days total
        println!("{}Days Total", TAB_SPACE);
        println!(
            "{}{}{}:{}",
            TAB_SPACE,
            TAB_SPACE,
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
