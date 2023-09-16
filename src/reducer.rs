use std::collections::HashMap;

#[derive(Clone)]
pub struct ReducerConfig <'a>{
   pub src_string: &'a str,
   pub ctx: HashMap<String, serde_json::Value>,
   pub aliases: Vec<String>,
}

#[derive(Clone)]
pub struct Reducer <'a>{
    config: ReducerConfig<'a>,
}


impl<'a> Reducer<'a> {
    pub fn new(config: ReducerConfig<'a>) -> Self {
        Reducer { config }
    }

    pub fn init(&self) -> String {
        let ctx = self.config.ctx.clone();
        let aliases = self.config.aliases.clone();

        for raw_alias in aliases {
            let (
                alias_name,
                alias_args,
                nesting
            ) = process_alias(&raw_alias);

            if ctx[&alias_name.to_owned()].is_null() {
                println!("Alias {} not found in context", alias_name);
                continue;
            }

            self.reduce();
        }

        String::from("test")
    }

    fn reduce(&self) {
        todo!()
    }

    fn reduceArray(self) {
        todo!()
    }

    fn reduceObject(self) {
        todo!()
    }

    fn reduceArgs(self) {
        todo!()
    }

    fn mutateString(self) {
        todo!()
    }

    fn checkType(self) {
        todo!()
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
