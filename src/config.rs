use std::collections::HashMap;
//use std::collections::hash_map::Entry;
use std::str::FromStr;
use std::fs;

fn config_storage(key: &str, val: Option<&str>) -> Option<&'static str> {
    static mut CONFIG: Option<HashMap<String, String>> = None;
    
    unsafe {
        if let Some(value) = val {
            if let Some(map) = CONFIG.as_mut() {
                map.insert(String::from(key), String::from(value));
            } else {
                let mut map = HashMap::new();
                map.insert(String::from(key), String::from(value));
                CONFIG = Some(map);
            }
            None
        } else {
            if let Some(map) = &CONFIG {
                Some(map.get(key).unwrap())
            } else {
                None
            }
        }
    }
}

pub fn config_load(filename: &str) {
    if let Ok(contents) = fs::read_to_string(filename) {
        let lines: Vec<&str> = contents.split('\n').collect();
        for i in lines {
            let strs: Vec<&str> = i.split(' ').collect();
            config_storage(strs[0], Some(strs[1]));
        }
    }
}

fn from_str_s<T: Default + FromStr>(str: &str) -> T {
    str.trim().parse().unwrap_or_default()
}

pub fn config_get<T: FromStr + Default>(key: &'static str) -> Option<T> {
    if let Some(val) = config_storage(key, None) {
        Some(from_str_s(val))
    } else {
        None
    }
}

pub fn config_get_value(key: &'static str) -> Option<&'static str> {
    if let Some(val) = config_storage(key, None) {
        Some(val)
    } else {
        None
    }
}

/*pub fn config_set(key: &str, val: &str) {
    config_storage(key, Some(val));
}*/
