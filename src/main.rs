use postgres::{Client, Error, NoTls};
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
    fn to_str(&self) -> &'static str {
        match *self {
            Status::Initialised => "Initialised",
            Status::InProgress => "InProgress",
            Status::Finished => "Finished",
            Status::Unknown => "Unknown",
        }
    }
}

fn get_data_from_db(client: &mut Client, ids: &[String]) -> Result<HashMap<String, Status>, Error> {
    // Create map to store the stuff
    let mut status_map = HashMap::<String, Status>::new();

    let mut counter = 0;
    for row in client.query(
        "SELECT tasks.id, tasks.status FROM tasks WHERE tasks.status IN ('Initialised', 'InProgress') OR tasks.id = ANY($1)",
        &[&ids],
    )? {
        let id: String = row.get(0);
        let status_str: String = row.get(1);

        let status = Status::from_str(&status_str);

        status_map.insert(id, status);
        counter += 1;
    }

    println!("found {} relevant rows", counter);

    Ok(status_map)
}

fn convert_keys_to_array(map: &HashMap<String, Status>) -> Vec<String> {
    map.keys().cloned().collect()
}

fn main() -> Result<(), Error> {
    println!("starting to check cache and stay up to date");

    let conn_str = "host=localhost user=postgres password=password dbname=rust-cache-impl";
    let mut client = Client::connect(conn_str, NoTls)?;
    client.execute("SET search_path TO testdata", &[])?;
    println!("initialised db client");

    let mut cache = HashMap::<String, Status>::new();

    let continue_check: bool = true;
    while continue_check == true {
        sleep(Duration::from_secs(2));
        println!("starting");

        let known_ids = convert_keys_to_array(&cache);
        match get_data_from_db(&mut client, &known_ids) {
            Ok(db_results) => {
                for (k, v) in &db_results {
                    if cache.contains_key(k) {
                        let existing_value = cache.get(k).copied().unwrap();
                        if *v == existing_value {
                            println!("key {} matches, doing nothing", k);
                        } else {
                            println!("{} changed to {}", k, v.to_str());
                            match *v {
                                Status::Initialised => {
                                    println!("changing {} in cache to {}", k, v.to_str());
                                    cache.insert(k.clone(), v.clone());
                                }
                                Status::InProgress => {
                                    println!("changing {} in cache to {}", k, v.to_str());
                                    cache.insert(k.clone(), v.clone());

                                    // Run mock side-effect for thing in progress
                                    println!("doing a thing for {}", k)
                                }
                                Status::Finished => {
                                    println!("removing {} from cache", k);
                                    cache.remove(k);
                                }
                                Status::Unknown => {
                                    println!("unknown value in db, ignoring")
                                }
                            }
                        }
                    } else {
                        println!("{} identified in state {}", k, v.to_str());
                        match *v {
                            Status::Initialised => {
                                println!("inserting {} as {}", k, v.to_str());
                                cache.insert(k.clone(), v.clone());
                            }
                            Status::InProgress => {
                                println!("inserting {} as {}", k, v.to_str());
                                cache.insert(k.clone(), v.clone());

                                // Run mock side-effect for thing in progress
                                println!("doing a thing for {}", k)
                            }
                            Status::Finished => {
                                println!("{} is set to Finished, can ignore", k);
                            }
                            Status::Unknown => {
                                println!("unknown value in db, ignoring");
                            }
                        }
                    }
                }
            }
            Err(e) => {
                println!("failed to get data from db: {}", e)
            }
        }
    }

    println!("completing");
    Ok(())
}
