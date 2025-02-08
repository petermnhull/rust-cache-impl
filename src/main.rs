use std::fmt;
use std::{collections::HashMap, thread::sleep, time::Duration};

#[derive(Clone, Copy, PartialEq)]
enum Status {
    Initialised,
    InProgress,
    Finished,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Status::Initialised => write!(f, "Started"),
            Status::InProgress => write!(f, "Progressing"),
            Status::Finished => write!(f, "Ended"),
        }
    }
}

fn get_data_from_db() -> HashMap<String, Status> {
    let mut data = HashMap::<String, Status>::new();

    // Insert some placeholder data
    // Replace later with a DB call
    data.insert("a".to_string(), Status::Initialised);
    data.insert("b".to_string(), Status::InProgress);
    data.insert("c".to_string(), Status::Finished);

    data
}

fn main() {
    println!("starting to check cache and stay up to date");

    let mut cache = HashMap::<String, Status>::new();

    let mut counter: i32 = 0;
    let mut continue_check: bool = true;

    while continue_check == true {
        sleep(Duration::from_secs(2));
        println!("starting");

        let d = get_data_from_db();
        for (k, v) in &d {
            println!("{}: {}", k, v);

            if cache.contains_key(k) {
                let existing_value = cache.get(k).copied().unwrap();
                if *v == existing_value {
                    println!("key {} matches, doing nothing", k);
                } else {
                    // - If Initialised, add to the cache
                    // - If InProgress, add to the cache and do a thing
                    // - If Finished, remove from the cache
                    //
                    println!("updating {} to match db", k);
                    cache.insert(k.clone(), v.clone());
                }
            } else {
                println!("new key {}, inserting", k);
                cache.insert(k.clone(), v.clone());

                // Need to run side-effect:
                // - If Initialised, do nothing
                // - If InProgress, do a thing
                // - If Finished, remove from the cache
            }
        }

        counter += 1;

        if counter > 5 {
            continue_check = false;
        }
    }

    println!("completing")
}
