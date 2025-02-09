mod cache;
mod map;
mod status;

use cache::compare_and_update;
use map::convert_keys_to_array;
use postgres::{Client, Error, NoTls};
use status::Status;
use std::{collections::HashMap, thread::sleep, time::Duration};

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
