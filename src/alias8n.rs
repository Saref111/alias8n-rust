use std::fs::{read_to_string, write};
use serde_json::Value;
use regex::Regex;
use crate::lib::{try_exists, captures_into_vec};


pub struct Config {
    pub ctx_path: Option<String>,
    pub source: Option<String>,
    pub dest: Option<String>,
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

#[derive(Debug)]
 pub enum AliasError {
    InvalidAlias(String),
    InvalidContext(String),
}

pub fn alias8n(config: Option<Config>) {
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

    let aliases = captures_into_vec(aliases);

    let aliased_src_string = replace_aliases(&src_string, &aliases, ctx);
    
    write(dest, aliased_src_string).unwrap();

}

fn replace_aliases(src_string: &str, aliases: &Vec<String>, ctx: Value) -> String {
    let mut result_string = String::from(src_string);
    for alias in aliases {
        let processed = process_alias(alias, &ctx);
        match processed {
            Ok(processed) => {
                result_string = result_string.replace(alias, &processed);
            },
            Err(e) => {
                println!("{:?}: {}", e, alias);
            },
        }
    }
    result_string
}

fn process_alias(alias: &str, ctx: &Value) -> Result<String, AliasError> {
    let raw_alias = alias.replace("a(:", "").replace(":)", "");

    let arguments = raw_alias.split(",").collect::<Vec<&str>>();

    let nesting = arguments.get(0).unwrap().split(".").collect::<Vec<&str>>();
    
    let mut value_from_ctx = get_value_from_ctx(nesting, ctx)?;

    if arguments.len() > 1 {
        for (i, arg) in arguments[1..].iter().enumerate() {
            value_from_ctx = value_from_ctx.replace(&format!("<{}>", i + 1), arg.trim());
        }
    }

    Ok(value_from_ctx)
}

fn get_value_from_ctx(nesting: Vec<&str>, ctx: &Value) -> Result<String, AliasError> {
    let mut value = ctx;
    for nest in nesting {
        value = match value.get(nest) {
            Some(v) => v,
            None => {
                return Err(AliasError::InvalidAlias(String::from("Invalid alias")));
            },
        };
    }
    
    if value.is_string() { // TODO: think about array handling
        let value = value.as_str().ok_or(AliasError::InvalidAlias(String::from("Invalid alias value")))?;
        return Ok(value[0..value.len()].to_string());
    } else if value.is_boolean() || value.is_number() {
        let value = value.to_string();
        return Ok(value);
    } else {
        return Err(AliasError::InvalidContext(String::from("Invalid context")));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_value_from_ctx() {
        let ctx = r#"
        {
            "name": "John",
            "age": 30,
            "cars": {
                "car1": "Ford",
                "car2": "BMW",
                "car3": "Fiat"
            }
        }
        "#;
        let ctx: Value = serde_json::from_str(&ctx).unwrap();
        let value = get_value_from_ctx(vec![&"name"], &ctx).unwrap();
        assert_eq!(value, "John");
        let value = get_value_from_ctx(vec![&"age"], &ctx).unwrap();
        assert_eq!(value, "30");
        let value = get_value_from_ctx(vec![&"cars", &"car1"], &ctx).unwrap();
        assert_eq!(value, "Ford");
        let value = get_value_from_ctx(vec![&"cars", &"car2"], &ctx).unwrap();
        assert_eq!(value, "BMW");
        let value = get_value_from_ctx(vec![&"cars", &"car3"], &ctx).unwrap();
        assert_eq!(value, "Fiat");
    }

    #[test]
    fn test_process_alias() {
        let ctx = r#"
        {
            "name": "John",
            "age": 30,
            "cars": {
                "car1": "Ford",
                "car2": "BMW",
                "car3": "Fiat"
            },
            "template": "Hello, <1>! You are <2> years old. You have <3>, <4> and <5> cars.",
        }
        "#;
        let ctx: Value = serde_json::from_str(&ctx).unwrap();
        let alias = "a(:name:)";
        let value = process_alias(alias, &ctx).unwrap();
        assert_eq!(value, "John");
        let alias = "a(:age:)";
        let value = process_alias(alias, &ctx).unwrap();
        assert_eq!(value, "30");
        let alias = "a(:cars.car1:)";
        let value = process_alias(alias, &ctx).unwrap();
        assert_eq!(value, "Ford");
        let alias = "a(:cars.car2:)";
        let value = process_alias(alias, &ctx).unwrap();
        assert_eq!(value, "BMW");
        let alias = "a(:cars.car3:)";
        let value = process_alias(alias, &ctx).unwrap();
        assert_eq!(value, "Fiat");
        let alias = "a(:template, Nick, 99, X, Y, Z:)";
        let value = process_alias(alias, &ctx).unwrap();
        assert_eq!(value, "Hello, Nick! You are 99 years old. You have X, Y and Z cars.");
    }
}
