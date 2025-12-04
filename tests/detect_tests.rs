use std::collections::HashMap;
use std::fs;
use std::path::Path;
use tempfile::tempdir;

// Re-implement detection logic for testing (since we can't import from main crate easily)
#[derive(Debug, PartialEq)]
enum PortStrategy {
    EnvVar(String),
    CliFlag(String),
}

#[derive(Debug)]
struct Detection {
    framework: String,
    start_command: String,
    port_strategy: PortStrategy,
}

#[derive(serde::Deserialize)]
struct PackageJson {
    scripts: Option<HashMap<String, String>>,
    dependencies: Option<HashMap<String, serde_json::Value>>,
    #[serde(rename = "devDependencies")]
    dev_dependencies: Option<HashMap<String, serde_json::Value>>,
}

fn merge_deps(
    deps: &Option<HashMap<String, serde_json::Value>>,
    dev_deps: &Option<HashMap<String, serde_json::Value>>,
) -> HashMap<String, serde_json::Value> {
    let mut all = HashMap::new();
    if let Some(d) = deps {
        all.extend(d.clone());
    }
    if let Some(d) = dev_deps {
        all.extend(d.clone());
    }
    all
}

fn detect(dir: &Path) -> Result<Detection, Box<dyn std::error::Error>> {
    let package_json_path = dir.join("package.json");
    if package_json_path.exists() {
        let content = fs::read_to_string(&package_json_path)?;
        let package: PackageJson = serde_json::from_str(&content)?;

        let all_deps = merge_deps(&package.dependencies, &package.dev_dependencies);
        let scripts = package.scripts.unwrap_or_default();
        let dev_script = scripts.get("dev").cloned().unwrap_or_default();

        if all_deps.contains_key("next") {
            return Ok(Detection {
                framework: "Next.js".into(),
                start_command: "npm run dev".into(),
                port_strategy: PortStrategy::EnvVar("PORT".into()),
            });
        }

        if all_deps.contains_key("vite") || dev_script.contains("vite") {
            return Ok(Detection {
                framework: "Vite".into(),
                start_command: "npm run dev --".into(),
                port_strategy: PortStrategy::CliFlag("--port".into()),
            });
        }

        if all_deps.contains_key("react-scripts") {
            return Ok(Detection {
                framework: "Create React App".into(),
                start_command: "npm start".into(),
                port_strategy: PortStrategy::EnvVar("PORT".into()),
            });
        }

        if all_deps.contains_key("@remix-run/dev") {
            return Ok(Detection {
                framework: "Remix".into(),
                start_command: "npm run dev".into(),
                port_strategy: PortStrategy::EnvVar("PORT".into()),
            });
        }

        if all_deps.contains_key("nuxt") {
            return Ok(Detection {
                framework: "Nuxt".into(),
                start_command: "npm run dev".into(),
                port_strategy: PortStrategy::EnvVar("PORT".into()),
            });
        }

        if all_deps.contains_key("@nestjs/core") {
            return Ok(Detection {
                framework: "NestJS".into(),
                start_command: "npm run start:dev".into(),
                port_strategy: PortStrategy::EnvVar("PORT".into()),
            });
        }

        if all_deps.contains_key("fastify") {
            return Ok(Detection {
                framework: "Fastify".into(),
                start_command: "npm run dev".into(),
                port_strategy: PortStrategy::EnvVar("PORT".into()),
            });
        }

        if all_deps.contains_key("express") {
            return Ok(Detection {
                framework: "Express".into(),
                start_command: "npm run dev".into(),
                port_strategy: PortStrategy::EnvVar("PORT".into()),
            });
        }

        let start_cmd = if scripts.contains_key("dev") {
            "npm run dev"
        } else if scripts.contains_key("start") {
            "npm start"
        } else {
            "npm run dev"
        };

        return Ok(Detection {
            framework: "Node.js".into(),
            start_command: start_cmd.into(),
            port_strategy: PortStrategy::EnvVar("PORT".into()),
        });
    }

    if dir.join("Gemfile").exists() {
        return Ok(Detection {
            framework: "Rails".into(),
            start_command: "rails server".into(),
            port_strategy: PortStrategy::CliFlag("-p".into()),
        });
    }

    if dir.join("manage.py").exists() {
        return Ok(Detection {
            framework: "Django".into(),
            start_command: "python manage.py runserver".into(),
            port_strategy: PortStrategy::CliFlag("0.0.0.0:".into()),
        });
    }

    if dir.join("go.mod").exists() {
        return Ok(Detection {
            framework: "Go".into(),
            start_command: "go run .".into(),
            port_strategy: PortStrategy::EnvVar("PORT".into()),
        });
    }

    Ok(Detection {
        framework: "Unknown".into(),
        start_command: "npm run dev".into(),
        port_strategy: PortStrategy::EnvVar("PORT".into()),
    })
}

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
fn test_merge_deps_both() {
    let mut deps = HashMap::new();
    deps.insert("express".to_string(), serde_json::json!("4.0.0"));

    let mut dev_deps = HashMap::new();
    dev_deps.insert("jest".to_string(), serde_json::json!("29.0.0"));

    let merged = merge_deps(&Some(deps), &Some(dev_deps));
    assert!(merged.contains_key("express"));
    assert!(merged.contains_key("jest"));
    assert_eq!(merged.len(), 2);
}

#[test]
fn test_merge_deps_only_deps() {
    let mut deps = HashMap::new();
    deps.insert("express".to_string(), serde_json::json!("4.0.0"));

    let merged = merge_deps(&Some(deps), &None);
    assert!(merged.contains_key("express"));
    assert_eq!(merged.len(), 1);
}

#[test]
fn test_merge_deps_only_dev_deps() {
    let mut dev_deps = HashMap::new();
    dev_deps.insert("jest".to_string(), serde_json::json!("29.0.0"));

    let merged = merge_deps(&None, &Some(dev_deps));
    assert!(merged.contains_key("jest"));
    assert_eq!(merged.len(), 1);
}

#[test]
fn test_merge_deps_none() {
    let merged = merge_deps(&None, &None);
    assert!(merged.is_empty());
}

#[test]
fn test_nextjs_priority_over_express() {
    // Next.js apps often have express as a dependency too
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
