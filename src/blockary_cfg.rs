use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Origin {
    pub path: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub origins: HashMap<String, Origin>,
}

pub fn load(toml_str: &str) -> Config {
    let config: Config = match toml::from_str(toml_str) {
        Ok(cfg) => cfg,
        Err(err) => panic!("Could not load the config file {}", err),
    };
    config
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loading_config_example() {
        let toml_str = r#"
        [origins]
        [origins.work]
        path = "/home/foo/worknotes"
        name = "Work"

        [origins.personal]
        path = "/home/bar/baz/personal"
        name = "Personal"
    "#;

        let cfg = load(toml_str);
        assert_eq!(cfg.origins.get("work").unwrap().name, "Work");
        assert_eq!(cfg.origins.get("work").unwrap().path, "/home/foo/worknotes");
        assert_eq!(cfg.origins.len(), 2);
    }

    #[test]
    #[should_panic]
    fn test_loading_config_with_missing_field() {
        let toml_str = r#"
        [origins]
        [origins.work]
        path = "/home/foo/worknotes"
    "#;

        let cfg = load(toml_str);
    }
}
