use std::collections::HashMap;

pub fn convert_keys_to_array<K, V>(map: &HashMap<K, V>) -> Vec<K>
where
    K: Ord + Clone,
{
    let mut out: Vec<K> = map.keys().cloned().collect();
    out.sort();

    out
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::map::convert_keys_to_array;

    #[test]
    fn test_convert_keys_to_array() {
        let mut map: HashMap<String, i32> = HashMap::new();
        map.insert("a".to_string(), 1);
        map.insert("b".to_string(), 2);

        let out = convert_keys_to_array(&map);
        let expected = vec!["a".to_string(), "b".to_string()];
        assert_eq!(out, expected);
    }
}
