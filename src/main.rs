use std::fs::{read_to_string, write};
use serde_json::Value;
use regex::Regex;

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

#[derive(Debug)]
enum AliasError {
    InvalidAlias(String),
    InvalidContext(String),
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

    let aliases = captures_into_vec(aliases);



    let aliased_src_string = replace_aliases(&src_string, &aliases, ctx);
    
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
    
    if value.is_string() { // TODO: value can be a number, bool, string,
        let value = value.as_str().ok_or(AliasError::InvalidAlias(String::from("Invalid alias value")))?;
        return Ok(value[0..value.len()].to_string());
    } else {
        return Err(AliasError::InvalidContext(String::from("Invalid context")));
    }
}
