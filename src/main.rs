use crate::alias8n::{Config, alias8n};

mod alias8n;
mod lib;

fn main() {
    // alias8n --ctx ./ctx.json --source ./index.html --dest ./index-aliased.html
    // alias8n --ctx ./ctx.json --source ./index.html
    // alias8n --ctx ./ctx.json
    // alias8n
    let args: Vec<String> = std::env::args().collect();

    let config = get_config_from_args(args);
    alias8n(config);	
}

fn get_config_from_args(args: Vec<String>) -> Option<Config> {
    let mut config = Config::default();
    let mut i = 1;
    while i < args.len() {
        let arg = args.get(i).unwrap();
        match arg.as_str() {
            "--ctx" => {
                i += 1;
                config.ctx_path = Some(args.get(i).unwrap().to_string());
            },
            "--source" => {
                i += 1;
                config.source = Some(args.get(i).unwrap().to_string());
            },
            "--dest" => {
                i += 1;
                config.dest = Some(args.get(i).unwrap().to_string());
            },
            _ => {
                panic!("Invalid argument");
            },
        }
        i += 1;
    }
    Some(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_config_from_args() {
        let args = vec![
            String::from("alias8n"),
            String::from("--ctx"),
            String::from("./ctx.json"),
            String::from("--source"),
            String::from("./index.html"),
            String::from("--dest"),
            String::from("./index-aliased.html"),
        ];
        let config = get_config_from_args(args).unwrap();
        assert_eq!(config.ctx_path.unwrap(), "./ctx.json");
        assert_eq!(config.source.unwrap(), "./index.html");
        assert_eq!(config.dest.unwrap(), "./index-aliased.html");
    }

    #[test]
    fn test_get_config_from_args_with_default() {
        let args = vec![
            String::from("alias8n"),
            String::from("--ctx"),
            String::from("./ctx.json"),
        ];
        let config = get_config_from_args(args).unwrap();
        assert_eq!(config.ctx_path.unwrap(), "./ctx.json");
        assert_eq!(config.source.unwrap(), "./index.html");
        assert_eq!(config.dest.unwrap(), "./index-aliased.html");
    }

    #[test]
    fn test_get_config_from_args_with_default_2() {
        let args = vec![
            String::from("alias8n"),
            String::from("--ctx"),
            String::from("./ctx.json"),
            String::from("--source"),
            String::from("./index.html"),
        ];
        let config = get_config_from_args(args).unwrap();
        assert_eq!(config.ctx_path.unwrap(), "./ctx.json");
        assert_eq!(config.source.unwrap(), "./index.html");
        assert_eq!(config.dest.unwrap(), "./index-aliased.html");
    }

    #[test]
    fn test_get_config_from_args_with_default_3() {
        let args = vec![
            String::from("alias8n"),
            String::from("--ctx"),
            String::from("./ctx.json"),
            String::from("--dest"),
            String::from("./index-aliased.html"),
        ];
        let config = get_config_from_args(args).unwrap();
        assert_eq!(config.ctx_path.unwrap(), "./ctx.json");
        assert_eq!(config.source.unwrap(), "./index.html");
        assert_eq!(config.dest.unwrap(), "./index-aliased.html");
    }

    #[test]
    fn test_get_config_from_args_with_default_4() {
        let args = vec![
            String::from("alias8n"),
            String::from("--ctx"),
            String::from("./ctx.json"),
            String::from("--source"),
            String::from("./index.html"),
            String::from("--dest"),
            String::from("./index-aliased.html"),
            String::from("--ctx"),
            String::from("./ctx2.json"),
        ];
        let config = get_config_from_args(args).unwrap();
        assert_eq!(config.ctx_path.unwrap(), "./ctx2.json");
        assert_eq!(config.source.unwrap(), "./index.html");
        assert_eq!(config.dest.unwrap(), "./index-aliased.html");
    }
}
