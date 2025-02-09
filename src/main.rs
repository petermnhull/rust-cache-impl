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

fn compare_and_update(cache: &mut HashMap<String, Status>, new_key: &String, new_value: &Status) {
    if cache.contains_key(new_key) {
        let existing_value = cache.get(new_key).copied().unwrap();
        if *new_value == existing_value {
            println!("key {} matches, doing nothing", new_key);
            return;
        }

        println!("{} changed to {}", new_key, new_value.to_str());
        match *new_value {
            Status::Initialised => {
                println!("changing {} in cache to {}", new_key, new_value.to_str());
                cache.insert(new_key.clone(), new_value.clone());
            }
            Status::InProgress => {
                println!("changing {} in cache to {}", new_key, new_value.to_str());
                cache.insert(new_key.clone(), new_value.clone());

                // Run mock side-effect for thing in progress
                println!("doing a thing for {}", new_key)
            }
            Status::Finished => {
                println!("removing {} from cache", new_key);
                cache.remove(new_key);
            }
            Status::Unknown => {
                // Shouldn't happen due to DB query and not storing Unknown in cache
                println!("unknown value in db, ignoring");
            }
        }
        return;
    }

    println!("{} identified in state {}", new_key, new_value.to_str());
    match *new_value {
        Status::Initialised => {
            println!("inserting {} as {}", new_key, new_value.to_str());
            cache.insert(new_key.clone(), new_value.clone());
        }
        Status::InProgress => {
            println!("inserting {} as {}", new_key, new_value.to_str());
            cache.insert(new_key.clone(), new_value.clone());

            // Run mock side-effect for thing in progress
            println!("doing a thing for {}", new_key)
        }

        Status::Finished => {
            // Shouldn't happen as we're not retrieving rows with status Finished
            // in DB query if we don't already have the key in the cache
            println!("{} is set to Finished, can ignore", new_key);
        }
        Status::Unknown => {
            // Shouldn't happen due to DB query and not storing Unknown in cache
            println!("unknown value in db, ignoring");
        }
    }
}

fn update_cache(cache: &mut HashMap<String, Status>, client: &mut Client) {
    let known_ids = convert_keys_to_array(&cache);
    match get_data_from_db(client, &known_ids) {
        Ok(db_results) => {
            for (k, v) in &db_results {
                compare_and_update(cache, k, v);
            }
        }
        Err(e) => {
            println!("failed to get data from db: {}", e)
        }
    }
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

        update_cache(&mut cache, &mut client);
    }

    println!("completing");
    Ok(())
}
