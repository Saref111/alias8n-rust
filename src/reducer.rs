use std::collections::HashMap;
#[derive(Clone)]
pub struct ReducerConfig {
   pub src_string: String,
   pub ctx: HashMap<String, serde_json::Value>,
   pub aliases: Vec<String>,
}

#[derive(Clone)]
pub struct Reducer {
    config: ReducerConfig,
}

#[derive(Clone)]
struct CurrentAlias {
    raw_alias: String,
    nesting: Vec<String>,
    alias_args: Vec<String>,
    alias_name: String,
}


impl Reducer {
    pub fn new(config: ReducerConfig) -> Self {
        Reducer { config }
    }

    pub fn init(&mut self) -> String {
        let ctx = self.config.ctx.clone();
        let aliases = self.config.aliases.clone();

        for raw_alias in aliases {
            let (
                alias_name,
                alias_args,
                nesting
            ) = process_alias(&raw_alias);

            if let None = ctx.get(&alias_name.to_owned()) {
                println!("Alias {} not found in context", alias_name);
                continue;
            }

            self.reduce(alias_name, alias_args, nesting, raw_alias);
        }

        self.config.src_string.clone()
    }

    fn reduce(&mut self, alias_name: String, alias_args: Vec<String>, nesting: Vec<String>, raw_alias: String) {
        let current_alias = CurrentAlias {
            raw_alias: raw_alias.clone(),
            nesting: nesting.clone(),
            alias_args: alias_args.clone(),
            alias_name: alias_name.clone(),
        };

        let nesting = nesting.clone();
        let raw_alias = raw_alias.clone();
        let raw_alias_regex = raw_alias.clone();
        let alias_args = alias_args.clone();
        let alias_name = alias_name.clone();

        let alias_value = self.config.ctx[&alias_name.to_owned()].clone();

        self.check_type(alias_value, current_alias);
    }

    fn reduce_array(&mut self, alias: Vec<serde_json::Value>, current_alias: CurrentAlias) {
        alias.iter().for_each(|value| {
            self.check_type(value.clone(), current_alias.clone());
        });
    }

    fn reduce_object(&mut self, current_alias: CurrentAlias) {
        let value: serde_json::Value = current_alias.nesting.iter().fold(  serde_json::Value::Null, |acc, key| {
            self.config.ctx.clone().get(key).unwrap().clone()
        });

        self.check_type(value, current_alias);
    }

    fn reduce_args(&mut self, current_alias: CurrentAlias) {
        current_alias.alias_args.iter().enumerate().for_each(|(i, arg)| {
            if i < 1 {
                return;
            }

            let r = regex::Regex::new(&format!("<{}>", i)).unwrap();
            let alias_value = self.config.ctx[&current_alias.alias_name.to_owned()].to_string().replace(r.as_str(), arg.trim());
            self.mutate_string(alias_value, current_alias.clone());
        });
    }

    fn mutate_string(&mut self, alias_value: String, current_alias: CurrentAlias) {
        let modified_string = self.config.src_string.replace(current_alias.raw_alias.as_str(), &alias_value);

        self.config.src_string = modified_string.clone();
    }

    fn check_type(&mut self, alias_value: serde_json::Value, current_alias: CurrentAlias) {
        match alias_value {
            serde_json::Value::Array(v) => self.reduce_array(v, current_alias),
            serde_json::Value::Object(_) => self.reduce_object(current_alias),
            serde_json::Value::String(string_value) => {
                println!("Reduced alias: {:?}", self.config.aliases);
                if current_alias.alias_args.len() < 2 {
                    self.mutate_string(string_value, current_alias)
                } else {
                    self.reduce_args(current_alias)
                }
            },
            _ => (),
        }
    }
}

fn process_alias(raw_alias: &str) -> (String, Vec<String>, Vec<String>) {
    let alias = raw_alias.replace("a(:", "").replace(":)", "");
    
    let alias_args: Vec<String> = alias.split(',').map(String::from).collect();
    let mut alias_name = alias_args[0].clone();

    let nesting: Vec<String> = alias_name.split('.').map(String::from).collect();
    alias_name = nesting[0].clone();

    (alias_name, alias_args, nesting)
}
