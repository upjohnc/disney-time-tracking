mod serial;
mod utils;

use chrono::{
    prelude::{DateTime, Utc},
    NaiveDate,
};
use serial::Entry;
use utils::read_json;

fn date_thing() -> NaiveDate {
    let now = Utc::now();
    let date = now.date_naive();
    println!("{:?}", date);
    date
}

fn add_date(dd: String, w: serial::BaseData) -> serial::BaseData {
    let entry = Entry::new(
        "big-guy".to_string(),
        "2022-11-28 00:43:58.512246 UTC".to_string(),
    );

    let mut core = w.core_data();

    match core.get_mut(&dd) {
        Some(x) => {
            x.push(entry);
            ()
        }
        _ => {
            core.insert(dd, vec![entry]);
            ()
        }
    }
    serial::BaseData::new(core)
}

fn main() {
    let base = read_json().expect("no errors");
    let dd = date_thing().to_string();
    let new_thing = add_date(dd, base);
    let dd = "2020-01-01".to_string();
    let other_thing = add_date(dd, new_thing);
    println!("{:?}", other_thing);
}
