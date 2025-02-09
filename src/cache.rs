use std::collections::HashMap;

use crate::status::Status;

pub fn compare_and_update(
    cache: &mut HashMap<String, Status>,
    new_key: &String,
    new_value: &Status,
) {
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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{compare_and_update, Status};

    #[test]
    fn test_compare_and_update_new_value_added() {
        let mut cache: HashMap<String, Status> = HashMap::new();
        cache.insert("a".to_string(), Status::Initialised);
        cache.insert("b".to_string(), Status::InProgress);

        let new_key = "c".to_string();
        let new_value = Status::InProgress;

        compare_and_update(&mut cache, &new_key, &new_value);

        assert_eq!(cache.contains_key(&new_key), true);
    }

    #[test]
    fn test_compare_and_update_changed_value_upserted() -> Result<(), String> {
        let mut cache: HashMap<String, Status> = HashMap::new();
        cache.insert("a".to_string(), Status::Initialised);
        cache.insert("b".to_string(), Status::InProgress);

        let existing_key = "a".to_string();
        let new_value = Status::InProgress;

        compare_and_update(&mut cache, &existing_key, &new_value);

        assert_eq!(cache.contains_key(&existing_key), true);
        let out = cache.get(&existing_key);
        match out {
            Some(v) => assert_eq!(v, &Status::InProgress),
            None => return Err("value missing from cache".into()),
        }
        Ok(())
    }

    #[test]
    fn test_compare_and_update_changed_value_removed_from_cache() {
        let mut cache: HashMap<String, Status> = HashMap::new();
        cache.insert("a".to_string(), Status::Initialised);
        cache.insert("b".to_string(), Status::InProgress);

        let existing_key = "b".to_string();
        let new_value = Status::Finished;

        compare_and_update(&mut cache, &existing_key, &new_value);

        assert_eq!(cache.contains_key(&existing_key), false);
    }
}
