use std::{fs::{read_to_string, write}, collections::HashMap};
use serde_json::{Result, Value};
use regex::Regex;
use reducer::{ReducerConfig, Reducer};

mod reducer;

struct Config {
    ctx_path: Option<String>,
    source: Option<String>,
    dest: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            ctx_path: Some(String::from("./ctx.json")),
            source: Some(String::from("./index.html")),
            dest: Some(String::from("./index-aliased.html")),
        }
    }
}

fn main() {
    alias8n(None);	
}


fn alias8n(config: Option<Config>) {
    let config = config.unwrap_or_default();
    let ctx_path = config.ctx_path.unwrap();
    let source = config.source.unwrap();
    let dest = config.dest.unwrap();

    if !ctx_path.ends_with(".json") {
        panic!("Context file should be a json file");
    }
    if !try_exists(&ctx_path) {
        panic!("No context file found by given path");
    }       
    if !try_exists(&source) {
        panic!("No source file found by given path");
    }

    let ctx = read_to_string(&ctx_path).unwrap();
    let ctx: Value = serde_json::from_str(&ctx).unwrap();
    let src_string = read_to_string(&source).unwrap();

    let re = Regex::new(r"a\(:.*?:\)").unwrap();

    let aliases = re.captures_iter(&src_string);

    let mut reducer = Reducer::new(ReducerConfig {
        src_string: src_string.to_owned(),
        ctx: json_value_into_hashmap(ctx),
        aliases: captures_into_vec(aliases),
    });

    let aliased_src_string = reducer.init();
    
    write(dest, aliased_src_string).unwrap();
}

fn try_exists(path: &str) -> bool{
    let result = std::fs::metadata(path);
    match result {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn captures_into_vec(captures: regex::CaptureMatches) -> Vec<String> {
    let mut vec = Vec::new();
    for capture in captures {
        vec.push(capture.get(0).unwrap().as_str().to_string());
    }
    vec
}

fn json_value_into_hashmap(value: Value) -> HashMap<String, Value> {
    let mut map = HashMap::new();
    match value {
        Value::Object(obj) => {
            for (key, value) in obj {
                map.insert(key, value);
            }
        }
        Value::Array(arr) => {
            for (index, value) in arr.iter().enumerate() {
                map.insert(index.to_string(), value.clone());
            }
        }
        _ => {}
    }
    map
}
