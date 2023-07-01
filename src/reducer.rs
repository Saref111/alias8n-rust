pub struct ReducerConfig <'a>{
   pub src_string: &'a str,
   pub ctx: serde_json::Value,
   pub aliases: regex::CaptureMatches<'a, 'a>,
}

pub struct Reducer <'a>{
    config: ReducerConfig<'a>,
}

impl<'a> Reducer<'a> {
    pub fn new(config: ReducerConfig<'a>) -> Self {
        Reducer { config }
    }
}
