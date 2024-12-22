mod serial;
mod utils;

fn main() {
    utils::pair_up();
    utils::new_entry("start".to_string());
    utils::new_entry("stop".to_string());
}
