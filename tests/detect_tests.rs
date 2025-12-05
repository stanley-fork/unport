use std::fs;
use tempfile::tempdir;
use unport_cli::detect::{detect, PortStrategy};

#[test]
fn test_detect_nextjs() {
    let dir = tempdir().unwrap();
    let package_json = r#"{
        "dependencies": {
            "next": "13.0.0",
            "react": "18.0.0"
        }
    }"#;
    fs::write(dir.path().join("package.json"), package_json).unwrap();

    let result = detect(dir.path()).unwrap();
    assert_eq!(result.framework, "Next.js");
    assert_eq!(result.start_command, "npm run dev");
    assert_eq!(result.port_strategy, PortStrategy::EnvVar("PORT".into()));
}

#[test]
fn test_detect_vite_dependency() {
    let dir = tempdir().unwrap();
    let package_json = r#"{
        "devDependencies": {
            "vite": "4.0.0"
        }
    }"#;
    fs::write(dir.path().join("package.json"), package_json).unwrap();

    let result = detect(dir.path()).unwrap();
    assert_eq!(result.framework, "Vite");
    assert_eq!(result.start_command, "npm run dev --");
    assert_eq!(result.port_strategy, PortStrategy::CliFlag("--port".into()));
}

#[test]
fn test_detect_vite_in_scripts() {
    let dir = tempdir().unwrap();
    let package_json = r#"{
        "scripts": {
            "dev": "vite"
        }
    }"#;
    fs::write(dir.path().join("package.json"), package_json).unwrap();

    let result = detect(dir.path()).unwrap();
    assert_eq!(result.framework, "Vite");
}

#[test]
fn test_detect_create_react_app() {
    let dir = tempdir().unwrap();
    let package_json = r#"{
        "dependencies": {
            "react-scripts": "5.0.0"
        }
    }"#;
    fs::write(dir.path().join("package.json"), package_json).unwrap();

    let result = detect(dir.path()).unwrap();
    assert_eq!(result.framework, "Create React App");
    assert_eq!(result.start_command, "npm start");
}

#[test]
fn test_detect_express() {
    let dir = tempdir().unwrap();
    let package_json = r#"{
        "dependencies": {
            "express": "4.18.0"
        }
    }"#;
    fs::write(dir.path().join("package.json"), package_json).unwrap();

    let result = detect(dir.path()).unwrap();
    assert_eq!(result.framework, "Express");
}

#[test]
fn test_detect_remix() {
    let dir = tempdir().unwrap();
    let package_json = r#"{
        "dependencies": {
            "@remix-run/dev": "1.0.0"
        }
    }"#;
    fs::write(dir.path().join("package.json"), package_json).unwrap();

    let result = detect(dir.path()).unwrap();
    assert_eq!(result.framework, "Remix");
}

#[test]
fn test_detect_nuxt() {
    let dir = tempdir().unwrap();
    let package_json = r#"{
        "dependencies": {
            "nuxt": "3.0.0"
        }
    }"#;
    fs::write(dir.path().join("package.json"), package_json).unwrap();

    let result = detect(dir.path()).unwrap();
    assert_eq!(result.framework, "Nuxt");
}

#[test]
fn test_detect_nestjs() {
    let dir = tempdir().unwrap();
    let package_json = r#"{
        "dependencies": {
            "@nestjs/core": "9.0.0"
        }
    }"#;
    fs::write(dir.path().join("package.json"), package_json).unwrap();

    let result = detect(dir.path()).unwrap();
    assert_eq!(result.framework, "NestJS");
    assert_eq!(result.start_command, "npm run start:dev");
}

#[test]
fn test_detect_fastify() {
    let dir = tempdir().unwrap();
    let package_json = r#"{
        "dependencies": {
            "fastify": "4.0.0"
        }
    }"#;
    fs::write(dir.path().join("package.json"), package_json).unwrap();

    let result = detect(dir.path()).unwrap();
    assert_eq!(result.framework, "Fastify");
}

#[test]
fn test_detect_rails() {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("Gemfile"), "gem 'rails'").unwrap();

    let result = detect(dir.path()).unwrap();
    assert_eq!(result.framework, "Rails");
    assert_eq!(result.start_command, "rails server");
    assert_eq!(result.port_strategy, PortStrategy::CliFlag("-p".into()));
}

#[test]
fn test_detect_django() {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("manage.py"), "#!/usr/bin/env python").unwrap();

    let result = detect(dir.path()).unwrap();
    assert_eq!(result.framework, "Django");
    assert_eq!(result.start_command, "python manage.py runserver");
    assert_eq!(result.port_strategy, PortStrategy::CliFlag("0.0.0.0:".into()));
}

#[test]
fn test_detect_go() {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("go.mod"), "module example.com/app\n\ngo 1.21").unwrap();

    let result = detect(dir.path()).unwrap();
    assert_eq!(result.framework, "Go");
    assert_eq!(result.start_command, "go run .");
    assert_eq!(result.port_strategy, PortStrategy::EnvVar("PORT".into()));
}

#[test]
fn test_detect_generic_nodejs_with_dev_script() {
    let dir = tempdir().unwrap();
    let package_json = r#"{
        "scripts": {
            "dev": "node index.js"
        }
    }"#;
    fs::write(dir.path().join("package.json"), package_json).unwrap();

    let result = detect(dir.path()).unwrap();
    assert_eq!(result.framework, "Node.js");
    assert_eq!(result.start_command, "npm run dev");
}

#[test]
fn test_detect_generic_nodejs_with_start_script() {
    let dir = tempdir().unwrap();
    let package_json = r#"{
        "scripts": {
            "start": "node index.js"
        }
    }"#;
    fs::write(dir.path().join("package.json"), package_json).unwrap();

    let result = detect(dir.path()).unwrap();
    assert_eq!(result.framework, "Node.js");
    assert_eq!(result.start_command, "npm start");
}

#[test]
fn test_detect_unknown_project() {
    let dir = tempdir().unwrap();
    let result = detect(dir.path()).unwrap();
    assert_eq!(result.framework, "Unknown");
}

#[test]
fn test_nextjs_priority_over_express() {
    let dir = tempdir().unwrap();
    let package_json = r#"{
        "dependencies": {
            "next": "13.0.0",
            "express": "4.18.0"
        }
    }"#;
    fs::write(dir.path().join("package.json"), package_json).unwrap();

    let result = detect(dir.path()).unwrap();
    assert_eq!(result.framework, "Next.js");
}
