use std::fs;
use tempfile::tempdir;
use unport_cli::config::Config;

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
    let dir = tempdir().unwrap();
    let config_content = r#"{"domain": "myapp"}"#;
    fs::write(dir.path().join("unport.json"), config_content).unwrap();

    let config = Config::load(dir.path()).unwrap();
    assert_eq!(config.full_domain(), "myapp.localhost");
}

#[test]
fn test_full_domain_api() {
    let dir = tempdir().unwrap();
    let config_content = r#"{"domain": "api"}"#;
    fs::write(dir.path().join("unport.json"), config_content).unwrap();

    let config = Config::load(dir.path()).unwrap();
    assert_eq!(config.full_domain(), "api.localhost");
}

#[test]
fn test_full_domain_with_hyphen() {
    let dir = tempdir().unwrap();
    let config_content = r#"{"domain": "my-cool-app"}"#;
    fs::write(dir.path().join("unport.json"), config_content).unwrap();

    let config = Config::load(dir.path()).unwrap();
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

#[test]
fn test_domain_with_numbers() {
    let dir = tempdir().unwrap();
    let config_content = r#"{"domain": "app123"}"#;
    fs::write(dir.path().join("unport.json"), config_content).unwrap();

    let config = Config::load(dir.path()).unwrap();
    assert_eq!(config.domain, "app123");
    assert_eq!(config.full_domain(), "app123.localhost");
}

#[test]
fn test_domain_with_underscores() {
    let dir = tempdir().unwrap();
    let config_content = r#"{"domain": "my_app"}"#;
    fs::write(dir.path().join("unport.json"), config_content).unwrap();

    let config = Config::load(dir.path()).unwrap();
    assert_eq!(config.domain, "my_app");
}

#[test]
fn test_very_long_domain() {
    let dir = tempdir().unwrap();
    let long_domain = "a".repeat(63);
    let config_content = format!(r#"{{"domain": "{}"}}"#, long_domain);
    fs::write(dir.path().join("unport.json"), config_content).unwrap();

    let config = Config::load(dir.path()).unwrap();
    assert_eq!(config.domain.len(), 63);
}

#[test]
fn test_start_command_with_args() {
    let dir = tempdir().unwrap();
    let config_content = r#"{
        "domain": "api",
        "start": "node server.js --env=production --debug"
    }"#;
    fs::write(dir.path().join("unport.json"), config_content).unwrap();

    let config = Config::load(dir.path()).unwrap();
    assert_eq!(config.start, Some("node server.js --env=production --debug".to_string()));
}

#[test]
fn test_start_command_with_pipes() {
    let dir = tempdir().unwrap();
    let config_content = r#"{
        "domain": "api",
        "start": "npm run build && npm start"
    }"#;
    fs::write(dir.path().join("unport.json"), config_content).unwrap();

    let config = Config::load(dir.path()).unwrap();
    assert!(config.start.unwrap().contains("&&"));
}

#[test]
fn test_port_env_common_names() {
    let common_env_vars = vec!["PORT", "HTTP_PORT", "SERVER_PORT", "APP_PORT", "LISTEN_PORT"];

    for env_var in common_env_vars {
        let dir = tempdir().unwrap();
        let config_content = format!(r#"{{"domain": "app", "portEnv": "{}"}}"#, env_var);
        fs::write(dir.path().join("unport.json"), config_content).unwrap();

        let config = Config::load(dir.path()).unwrap();
        assert_eq!(config.port_env, Some(env_var.to_string()));
    }
}

#[test]
fn test_port_arg_common_formats() {
    let common_args = vec!["--port", "-p", "-P", "--listen", "--http-port"];

    for arg in common_args {
        let dir = tempdir().unwrap();
        let config_content = format!(r#"{{"domain": "app", "portArg": "{}"}}"#, arg);
        fs::write(dir.path().join("unport.json"), config_content).unwrap();

        let config = Config::load(dir.path()).unwrap();
        assert_eq!(config.port_arg, Some(arg.to_string()));
    }
}

#[test]
fn test_whitespace_in_domain_preserved() {
    let dir = tempdir().unwrap();
    let config_content = r#"{"domain": "  myapp  "}"#;
    fs::write(dir.path().join("unport.json"), config_content).unwrap();

    let config = Config::load(dir.path()).unwrap();
    assert_eq!(config.domain, "  myapp  ");
}

#[test]
fn test_null_optional_fields() {
    let dir = tempdir().unwrap();
    let config_content = r#"{
        "domain": "app",
        "start": null,
        "portEnv": null,
        "portArg": null
    }"#;
    fs::write(dir.path().join("unport.json"), config_content).unwrap();

    let config = Config::load(dir.path()).unwrap();
    assert_eq!(config.domain, "app");
    assert_eq!(config.start, None);
    assert_eq!(config.port_env, None);
    assert_eq!(config.port_arg, None);
}

#[test]
fn test_config_with_comments_fails() {
    let dir = tempdir().unwrap();
    let config_content = r#"{
        "domain": "app" // this is a comment
    }"#;
    fs::write(dir.path().join("unport.json"), config_content).unwrap();

    let result = Config::load(dir.path());
    assert!(result.is_err());
}

#[test]
fn test_config_with_trailing_comma_fails() {
    let dir = tempdir().unwrap();
    let config_content = r#"{
        "domain": "app",
    }"#;
    fs::write(dir.path().join("unport.json"), config_content).unwrap();

    let result = Config::load(dir.path());
    assert!(result.is_err());
}
