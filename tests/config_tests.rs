use std::fs;
use std::path::Path;
use tempfile::tempdir;

#[derive(Debug, serde::Deserialize, PartialEq)]
struct Config {
    domain: String,
    start: Option<String>,
    #[serde(rename = "portEnv")]
    port_env: Option<String>,
    #[serde(rename = "portArg")]
    port_arg: Option<String>,
}

impl Config {
    fn load(dir: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = dir.join("unport.json");
        let content = fs::read_to_string(&config_path)?;
        let config: Config = serde_json::from_str(&content)?;
        Ok(config)
    }

    fn full_domain(&self) -> String {
        format!("{}.localhost", self.domain)
    }
}

#[test]
fn test_load_minimal_config() {
    let dir = tempdir().unwrap();
    let config_content = r#"{"domain": "myapp"}"#;
    fs::write(dir.path().join("unport.json"), config_content).unwrap();

    let config = Config::load(dir.path()).unwrap();
    assert_eq!(config.domain, "myapp");
    assert_eq!(config.start, None);
    assert_eq!(config.port_env, None);
    assert_eq!(config.port_arg, None);
}

#[test]
fn test_load_full_config() {
    let dir = tempdir().unwrap();
    let config_content = r#"{
        "domain": "api",
        "start": "npm run start",
        "portEnv": "SERVER_PORT",
        "portArg": "--port"
    }"#;
    fs::write(dir.path().join("unport.json"), config_content).unwrap();

    let config = Config::load(dir.path()).unwrap();
    assert_eq!(config.domain, "api");
    assert_eq!(config.start, Some("npm run start".to_string()));
    assert_eq!(config.port_env, Some("SERVER_PORT".to_string()));
    assert_eq!(config.port_arg, Some("--port".to_string()));
}

#[test]
fn test_load_config_with_custom_start() {
    let dir = tempdir().unwrap();
    let config_content = r#"{
        "domain": "backend",
        "start": "python app.py"
    }"#;
    fs::write(dir.path().join("unport.json"), config_content).unwrap();

    let config = Config::load(dir.path()).unwrap();
    assert_eq!(config.domain, "backend");
    assert_eq!(config.start, Some("python app.py".to_string()));
}

#[test]
fn test_load_config_with_port_env() {
    let dir = tempdir().unwrap();
    let config_content = r#"{
        "domain": "service",
        "portEnv": "HTTP_PORT"
    }"#;
    fs::write(dir.path().join("unport.json"), config_content).unwrap();

    let config = Config::load(dir.path()).unwrap();
    assert_eq!(config.port_env, Some("HTTP_PORT".to_string()));
}

#[test]
fn test_load_config_with_port_arg() {
    let dir = tempdir().unwrap();
    let config_content = r#"{
        "domain": "service",
        "portArg": "-p"
    }"#;
    fs::write(dir.path().join("unport.json"), config_content).unwrap();

    let config = Config::load(dir.path()).unwrap();
    assert_eq!(config.port_arg, Some("-p".to_string()));
}

#[test]
fn test_full_domain() {
    let config = Config {
        domain: "myapp".to_string(),
        start: None,
        port_env: None,
        port_arg: None,
    };

    assert_eq!(config.full_domain(), "myapp.localhost");
}

#[test]
fn test_full_domain_api() {
    let config = Config {
        domain: "api".to_string(),
        start: None,
        port_env: None,
        port_arg: None,
    };

    assert_eq!(config.full_domain(), "api.localhost");
}

#[test]
fn test_full_domain_with_hyphen() {
    let config = Config {
        domain: "my-cool-app".to_string(),
        start: None,
        port_env: None,
        port_arg: None,
    };

    assert_eq!(config.full_domain(), "my-cool-app.localhost");
}

#[test]
fn test_missing_config_file() {
    let dir = tempdir().unwrap();
    let result = Config::load(dir.path());
    assert!(result.is_err());
}

#[test]
fn test_invalid_json() {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("unport.json"), "{ invalid json }").unwrap();

    let result = Config::load(dir.path());
    assert!(result.is_err());
}

#[test]
fn test_missing_required_field() {
    let dir = tempdir().unwrap();
    // Missing "domain" field
    fs::write(dir.path().join("unport.json"), r#"{"start": "npm run dev"}"#).unwrap();

    let result = Config::load(dir.path());
    assert!(result.is_err());
}

#[test]
fn test_extra_fields_ignored() {
    let dir = tempdir().unwrap();
    let config_content = r#"{
        "domain": "myapp",
        "extra_field": "should be ignored",
        "another_field": 123
    }"#;
    fs::write(dir.path().join("unport.json"), config_content).unwrap();

    let config = Config::load(dir.path()).unwrap();
    assert_eq!(config.domain, "myapp");
}

#[test]
fn test_empty_domain() {
    let dir = tempdir().unwrap();
    let config_content = r#"{"domain": ""}"#;
    fs::write(dir.path().join("unport.json"), config_content).unwrap();

    let config = Config::load(dir.path()).unwrap();
    assert_eq!(config.domain, "");
    assert_eq!(config.full_domain(), ".localhost");
}

#[test]
fn test_unicode_domain() {
    let dir = tempdir().unwrap();
    let config_content = r#"{"domain": "my-app-测试"}"#;
    fs::write(dir.path().join("unport.json"), config_content).unwrap();

    let config = Config::load(dir.path()).unwrap();
    assert_eq!(config.domain, "my-app-测试");
}
