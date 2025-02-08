use postgres::{Client, Error, NoTls};
use std::fmt;
use std::{collections::HashMap, thread::sleep, time::Duration};

#[derive(Clone, Copy, PartialEq)]
enum Status {
    Initialised,
    InProgress,
    Finished,
    Unknown,
}

impl Status {
    fn from_str(status: &str) -> Status {
        match status.to_lowercase().as_str() {
            "initialised" => Status::Initialised,
            "inprogress" => Status::InProgress,
            "finished" => Status::Finished,
            _ => Status::Unknown,
        }
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Status::Initialised => write!(f, "Initialised"),
            Status::InProgress => write!(f, "Progressing"),
            Status::Finished => write!(f, "Finished"),
            Status::Unknown => write!(f, "Unknown"),
        }
    }
}

fn get_data_from_db() -> Result<HashMap<String, Status>, Error> {
    // Create map to store the stuff
    let mut status_map = HashMap::<String, Status>::new();
    let conn_str = "host=localhost user=postgres password=password dbname=rust-cache-impl";

    let mut client = Client::connect(conn_str, NoTls)?;

    client.execute("SET search_path TO testdata", &[])?;

    for row in client.query("SELECT newtable.id, newtable.status FROM newtable", &[])? {
        let id: String = row.get(0);
        let status_str: String = row.get(1);
        println!("id: {}, status: {}", id, status_str);

        let status = Status::from_str(&status_str);

        status_map.insert(id, status);
    }

    Ok(status_map)
}

fn main() {
    println!("starting to check cache and stay up to date");

    let mut cache = HashMap::<String, Status>::new();

    let continue_check: bool = true;
    while continue_check == true {
        sleep(Duration::from_secs(2));
        println!("starting");

        match get_data_from_db() {
            Ok(status_map) => {
                for (k, v) in &status_map {
                    println!("{}: {}", k, v);

                    if cache.contains_key(k) {
                        let existing_value = cache.get(k).copied().unwrap();
                        if *v == existing_value {
                            println!("key {} matches, doing nothing", k);
                        } else {
                            // TODO:
                            // - If Initialised, add to the cache
                            // - If InProgress, upsert to the cache and do a side-effect
                            // - If Finished, remove from the cache
                            println!("updating {} to match db", k);
                            cache.insert(k.clone(), v.clone());
                        }
                    } else {
                        println!("new key {}, inserting", k);
                        cache.insert(k.clone(), v.clone());

                        // TODO:
                        // - If Initialised, do nothing
                        // - If InProgress, add to the cache and do a side-effect
                        // - If Finished, remove from the cache
                    }
                }
            }
            Err(e) => {
                println!("failed to get data from db: {}", e)
            }
        }
    }

    println!("completing")
}
