use std::collections::HashMap;
use std::path::PathBuf;

const PORT_RANGE_START: u16 = 4000;
const PORT_RANGE_END: u16 = 5000;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
struct Service {
    domain: String,
    port: u16,
    pid: u32,
    directory: PathBuf,
}

#[derive(Default)]
struct Registry {
    services: HashMap<String, Service>,
    next_port: u16,
}

impl Registry {
    fn new() -> Self {
        Self {
            services: HashMap::new(),
            next_port: PORT_RANGE_START,
        }
    }

    fn get_port(&mut self) -> u16 {
        let port = self.next_port;
        self.next_port += 1;
        if self.next_port > PORT_RANGE_END {
            self.next_port = PORT_RANGE_START;
        }
        port
    }

    fn register(&mut self, service: Service) {
        self.services.insert(service.domain.clone(), service);
    }

    fn unregister(&mut self, domain: &str) -> Option<Service> {
        self.services.remove(domain)
    }

    fn get(&self, domain: &str) -> Option<&Service> {
        self.services.get(domain)
    }

    fn list(&self) -> Vec<Service> {
        self.services.values().cloned().collect()
    }
}

#[test]
fn test_registry_new() {
    let registry = Registry::new();
    assert!(registry.services.is_empty());
    assert_eq!(registry.next_port, PORT_RANGE_START);
}

#[test]
fn test_get_port_increments() {
    let mut registry = Registry::new();
    assert_eq!(registry.get_port(), 4000);
    assert_eq!(registry.get_port(), 4001);
    assert_eq!(registry.get_port(), 4002);
}

#[test]
fn test_get_port_wraps_around() {
    let mut registry = Registry::new();
    registry.next_port = PORT_RANGE_END;

    let port = registry.get_port();
    assert_eq!(port, PORT_RANGE_END);
    assert_eq!(registry.next_port, PORT_RANGE_START);
}

#[test]
fn test_register_service() {
    let mut registry = Registry::new();
    let service = Service {
        domain: "api.localhost".to_string(),
        port: 4000,
        pid: 12345,
        directory: PathBuf::from("/home/user/api"),
    };

    registry.register(service.clone());
    assert_eq!(registry.services.len(), 1);
    assert!(registry.services.contains_key("api.localhost"));
}

#[test]
fn test_register_multiple_services() {
    let mut registry = Registry::new();

    registry.register(Service {
        domain: "api.localhost".to_string(),
        port: 4000,
        pid: 1000,
        directory: PathBuf::from("/app/api"),
    });

    registry.register(Service {
        domain: "web.localhost".to_string(),
        port: 4001,
        pid: 1001,
        directory: PathBuf::from("/app/web"),
    });

    assert_eq!(registry.services.len(), 2);
}

#[test]
fn test_register_overwrites_existing() {
    let mut registry = Registry::new();

    registry.register(Service {
        domain: "api.localhost".to_string(),
        port: 4000,
        pid: 1000,
        directory: PathBuf::from("/app/api"),
    });

    registry.register(Service {
        domain: "api.localhost".to_string(),
        port: 4001,
        pid: 1001,
        directory: PathBuf::from("/app/api-new"),
    });

    assert_eq!(registry.services.len(), 1);
    assert_eq!(registry.get("api.localhost").unwrap().port, 4001);
}

#[test]
fn test_unregister_service() {
    let mut registry = Registry::new();
    let service = Service {
        domain: "api.localhost".to_string(),
        port: 4000,
        pid: 12345,
        directory: PathBuf::from("/home/user/api"),
    };

    registry.register(service.clone());
    let removed = registry.unregister("api.localhost");

    assert!(removed.is_some());
    assert_eq!(removed.unwrap().domain, "api.localhost");
    assert!(registry.services.is_empty());
}

#[test]
fn test_unregister_nonexistent() {
    let mut registry = Registry::new();
    let removed = registry.unregister("nonexistent.localhost");
    assert!(removed.is_none());
}

#[test]
fn test_get_service() {
    let mut registry = Registry::new();
    let service = Service {
        domain: "api.localhost".to_string(),
        port: 4000,
        pid: 12345,
        directory: PathBuf::from("/home/user/api"),
    };

    registry.register(service);

    let result = registry.get("api.localhost");
    assert!(result.is_some());
    assert_eq!(result.unwrap().port, 4000);
}

#[test]
fn test_get_nonexistent_service() {
    let registry = Registry::new();
    let result = registry.get("nonexistent.localhost");
    assert!(result.is_none());
}

#[test]
fn test_list_empty() {
    let registry = Registry::new();
    let list = registry.list();
    assert!(list.is_empty());
}

#[test]
fn test_list_services() {
    let mut registry = Registry::new();

    registry.register(Service {
        domain: "api.localhost".to_string(),
        port: 4000,
        pid: 1000,
        directory: PathBuf::from("/app/api"),
    });

    registry.register(Service {
        domain: "web.localhost".to_string(),
        port: 4001,
        pid: 1001,
        directory: PathBuf::from("/app/web"),
    });

    let list = registry.list();
    assert_eq!(list.len(), 2);
}

#[test]
fn test_service_serialization() {
    let service = Service {
        domain: "api.localhost".to_string(),
        port: 4000,
        pid: 12345,
        directory: PathBuf::from("/home/user/api"),
    };

    let json = serde_json::to_string(&service).unwrap();
    assert!(json.contains("api.localhost"));
    assert!(json.contains("4000"));
    assert!(json.contains("12345"));
}

#[test]
fn test_service_deserialization() {
    let json = r#"{
        "domain": "api.localhost",
        "port": 4000,
        "pid": 12345,
        "directory": "/home/user/api"
    }"#;

    let service: Service = serde_json::from_str(json).unwrap();
    assert_eq!(service.domain, "api.localhost");
    assert_eq!(service.port, 4000);
    assert_eq!(service.pid, 12345);
}

#[test]
fn test_registry_serialization() {
    let mut services: HashMap<String, Service> = HashMap::new();
    services.insert(
        "api.localhost".to_string(),
        Service {
            domain: "api.localhost".to_string(),
            port: 4000,
            pid: 12345,
            directory: PathBuf::from("/home/user/api"),
        },
    );

    let json = serde_json::to_string_pretty(&services).unwrap();
    let deserialized: HashMap<String, Service> = serde_json::from_str(&json).unwrap();

    assert_eq!(services, deserialized);
}

#[test]
fn test_max_port_calculation() {
    let mut registry = Registry::new();

    registry.register(Service {
        domain: "a.localhost".to_string(),
        port: 4000,
        pid: 1,
        directory: PathBuf::from("/a"),
    });

    registry.register(Service {
        domain: "b.localhost".to_string(),
        port: 4005,
        pid: 2,
        directory: PathBuf::from("/b"),
    });

    registry.register(Service {
        domain: "c.localhost".to_string(),
        port: 4002,
        pid: 3,
        directory: PathBuf::from("/c"),
    });

    let max_port = registry.services.values().map(|s| s.port).max().unwrap();
    assert_eq!(max_port, 4005);
}

#[test]
fn test_port_range_constants() {
    assert_eq!(PORT_RANGE_START, 4000);
    assert_eq!(PORT_RANGE_END, 5000);
    assert!(PORT_RANGE_END > PORT_RANGE_START);
    assert_eq!(PORT_RANGE_END - PORT_RANGE_START, 1000);
}
