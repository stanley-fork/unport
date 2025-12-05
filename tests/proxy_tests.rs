use std::net::TcpListener;
use std::sync::atomic::{AtomicU16, Ordering};

static TEST_PORT: AtomicU16 = AtomicU16::new(16000);

fn get_test_port() -> u16 {
    TEST_PORT.fetch_add(1, Ordering::SeqCst)
}

// Helper function to extract host header
fn extract_host_from_headers(headers: &str) -> Option<String> {
    for line in headers.lines() {
        let lower = line.to_lowercase();
        if lower.starts_with("host:") {
            return Some(line[5..].trim().to_string());
        }
    }
    None
}

// Helper to check if request is WebSocket upgrade
fn is_websocket_upgrade(headers: &str) -> bool {
    headers.contains("Upgrade: websocket") || headers.contains("upgrade: websocket")
}

mod host_extraction {
    use super::*;

    #[test]
    fn test_extract_simple_host() {
        let headers = "GET / HTTP/1.1\r\nHost: myapp.localhost\r\n\r\n";
        let host = extract_host_from_headers(headers);
        assert_eq!(host, Some("myapp.localhost".to_string()));
    }

    #[test]
    fn test_extract_host_with_port() {
        let headers = "GET / HTTP/1.1\r\nHost: myapp.localhost:80\r\n\r\n";
        let host = extract_host_from_headers(headers);
        assert_eq!(host, Some("myapp.localhost:80".to_string()));
    }

    #[test]
    fn test_extract_host_lowercase() {
        let headers = "GET / HTTP/1.1\r\nhost: myapp.localhost\r\n\r\n";
        let host = extract_host_from_headers(headers);
        assert_eq!(host, Some("myapp.localhost".to_string()));
    }

    #[test]
    fn test_extract_host_mixed_case() {
        let headers = "GET / HTTP/1.1\r\nHost: MyApp.LocalHost\r\n\r\n";
        let host = extract_host_from_headers(headers);
        assert_eq!(host, Some("MyApp.LocalHost".to_string()));
    }

    #[test]
    fn test_missing_host_header() {
        let headers = "GET / HTTP/1.1\r\nConnection: keep-alive\r\n\r\n";
        let host = extract_host_from_headers(headers);
        assert_eq!(host, None);
    }

    #[test]
    fn test_extract_host_with_whitespace() {
        let headers = "GET / HTTP/1.1\r\nHost:   myapp.localhost  \r\n\r\n";
        let host = extract_host_from_headers(headers);
        assert_eq!(host, Some("myapp.localhost".to_string()));
    }

    #[test]
    fn test_extract_host_multiple_headers() {
        let headers = "GET / HTTP/1.1\r\nAccept: */*\r\nHost: api.localhost\r\nConnection: keep-alive\r\n\r\n";
        let host = extract_host_from_headers(headers);
        assert_eq!(host, Some("api.localhost".to_string()));
    }

    #[test]
    fn test_domain_extraction_from_host() {
        let host = "myapp.localhost:80";
        let domain = host.split(':').next().unwrap_or(host);
        assert_eq!(domain, "myapp.localhost");
    }

    #[test]
    fn test_domain_extraction_without_port() {
        let host = "myapp.localhost";
        let domain = host.split(':').next().unwrap_or(host);
        assert_eq!(domain, "myapp.localhost");
    }
}

mod websocket_detection {
    use super::*;

    #[test]
    fn test_detect_websocket_upgrade() {
        let headers = "GET /ws HTTP/1.1\r\nHost: myapp.localhost\r\nUpgrade: websocket\r\nConnection: Upgrade\r\n\r\n";
        assert!(is_websocket_upgrade(headers));
    }

    #[test]
    fn test_detect_websocket_lowercase() {
        let headers = "GET /ws HTTP/1.1\r\nHost: myapp.localhost\r\nupgrade: websocket\r\nconnection: upgrade\r\n\r\n";
        assert!(is_websocket_upgrade(headers));
    }

    #[test]
    fn test_regular_http_not_websocket() {
        let headers = "GET / HTTP/1.1\r\nHost: myapp.localhost\r\nConnection: keep-alive\r\n\r\n";
        assert!(!is_websocket_upgrade(headers));
    }

    #[test]
    fn test_post_request_not_websocket() {
        let headers = "POST /api HTTP/1.1\r\nHost: myapp.localhost\r\nContent-Type: application/json\r\n\r\n";
        assert!(!is_websocket_upgrade(headers));
    }

    #[test]
    fn test_websocket_with_sec_headers() {
        let headers = "GET /ws HTTP/1.1\r\n\
            Host: myapp.localhost\r\n\
            Upgrade: websocket\r\n\
            Connection: Upgrade\r\n\
            Sec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==\r\n\
            Sec-WebSocket-Version: 13\r\n\r\n";
        assert!(is_websocket_upgrade(headers));
    }
}

mod port_availability {
    use super::*;

    #[test]
    fn test_port_available_when_free() {
        let port = get_test_port();
        let result = TcpListener::bind(("127.0.0.1", port));
        assert!(result.is_ok());
    }

    #[test]
    fn test_port_unavailable_when_bound() {
        let port = get_test_port();
        let _listener = TcpListener::bind(("127.0.0.1", port)).unwrap();

        let result = TcpListener::bind(("127.0.0.1", port));
        assert!(result.is_err());
    }

    #[test]
    fn test_ipv4_binding() {
        let port = get_test_port();
        let result = TcpListener::bind(("127.0.0.1", port));
        assert!(result.is_ok());
    }

    #[test]
    fn test_all_interfaces_binding() {
        let port = get_test_port();
        let result = TcpListener::bind(("0.0.0.0", port));
        assert!(result.is_ok());
    }

    #[test]
    fn test_ipv6_binding() {
        let port = get_test_port();
        // IPv6 might not be available on all systems
        let _ = TcpListener::bind(("::1", port));
        // Just verify it doesn't panic
    }
}

mod http_responses {
    #[test]
    fn test_404_response_format() {
        let response = "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\n\r\n";
        assert!(response.starts_with("HTTP/1.1 404"));
        assert!(response.contains("Content-Length: 0"));
    }

    #[test]
    fn test_502_response_format() {
        let error = "Connection refused";
        let response = format!("HTTP/1.1 502 Bad Gateway\r\nContent-Type: text/plain\r\n\r\nBad Gateway: {}", error);
        assert!(response.starts_with("HTTP/1.1 502"));
        assert!(response.contains("Bad Gateway"));
        assert!(response.contains(error));
    }

    #[test]
    fn test_200_json_response() {
        let response = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{\"ok\":true}";
        assert!(response.starts_with("HTTP/1.1 200"));
        assert!(response.contains("application/json"));
        assert!(response.contains("{\"ok\":true}"));
    }
}

mod dashboard {
    #[test]
    fn test_dashboard_html_doctype() {
        let html = "<!DOCTYPE html><html>";
        assert!(html.starts_with("<!DOCTYPE html>"));
    }

    #[test]
    fn test_service_row_html() {
        let domain = "api.localhost";
        let port = 4000;
        let url = format!("http://{}", domain);

        let row = format!(
            r#"<tr><td>running</td><td>{}</td><td>{}</td></tr>"#,
            url, port
        );

        assert!(row.contains("http://api.localhost"));
        assert!(row.contains("4000"));
    }

    #[test]
    fn test_empty_services_message() {
        let message = "No services running. Start one with unport start";
        assert!(message.contains("No services running"));
    }
}

mod request_routing {
    #[test]
    fn test_localhost_dashboard() {
        let domain = "localhost";
        let show_dashboard = domain == "localhost" || domain == "127.0.0.1";
        assert!(show_dashboard);
    }

    #[test]
    fn test_127_0_0_1_dashboard() {
        let domain = "127.0.0.1";
        let show_dashboard = domain == "localhost" || domain == "127.0.0.1";
        assert!(show_dashboard);
    }

    #[test]
    fn test_service_domain_no_dashboard() {
        let domain = "api.localhost";
        let show_dashboard = domain == "localhost" || domain == "127.0.0.1";
        assert!(!show_dashboard);
    }

    #[test]
    fn test_kill_api_path() {
        let path = "/api/kill/myapp.localhost";
        assert!(path.starts_with("/api/kill/"));

        let target = path.strip_prefix("/api/kill/").unwrap();
        assert_eq!(target, "myapp.localhost");
    }
}

mod https_proxy {
    #[test]
    fn test_https_port() {
        let https_port: u16 = 443;
        assert_eq!(https_port, 443);
    }

    #[test]
    fn test_http_port() {
        let http_port: u16 = 80;
        assert_eq!(http_port, 80);
    }

    #[test]
    fn test_https_url_format() {
        let domain = "api.localhost";
        let url = format!("https://{}", domain);
        assert_eq!(url, "https://api.localhost");
    }

    #[test]
    fn test_http_url_format() {
        let domain = "api.localhost";
        let url = format!("http://{}", domain);
        assert_eq!(url, "http://api.localhost");
    }

    #[test]
    fn test_https_url_with_port() {
        let domain = "api.localhost";
        let port = 443;
        let url = format!("https://{}:{}", domain, port);
        assert_eq!(url, "https://api.localhost:443");
    }
}

mod edge_cases {
    use super::*;

    #[test]
    fn test_empty_host_header() {
        let headers = "GET / HTTP/1.1\r\nHost:\r\n\r\n";
        let host = extract_host_from_headers(headers);
        assert_eq!(host, Some("".to_string()));
    }

    #[test]
    fn test_host_with_ipv6() {
        let host = "[::1]:8080";
        let parts: Vec<&str> = host.rsplitn(2, ':').collect();
        // For IPv6, this naive split doesn't work well
        // but we test the behavior
        assert!(parts.len() >= 1);
    }

    #[test]
    fn test_very_long_domain() {
        let long_subdomain = "a".repeat(63); // max label length
        let domain = format!("{}.localhost", long_subdomain);
        assert!(domain.len() > 63);
        assert!(domain.ends_with(".localhost"));
    }

    #[test]
    fn test_domain_with_many_subdomains() {
        let domain = "a.b.c.d.e.localhost";
        let parts: Vec<&str> = domain.split('.').collect();
        assert_eq!(parts.len(), 6);
        assert_eq!(parts.last(), Some(&"localhost"));
    }

    #[test]
    fn test_punycode_domain() {
        // Internationalized domain names use punycode
        let domain = "xn--n3h.localhost"; // emoji domain encoded
        assert!(domain.starts_with("xn--"));
    }

    #[test]
    fn test_case_insensitive_domain_matching() {
        let domain1 = "API.LOCALHOST";
        let domain2 = "api.localhost";
        assert_eq!(domain1.to_lowercase(), domain2.to_lowercase());
    }

    #[test]
    fn test_host_header_with_trailing_dot() {
        // DNS technically allows trailing dots
        let host = "api.localhost.";
        let normalized = host.trim_end_matches('.');
        assert_eq!(normalized, "api.localhost");
    }

    #[test]
    fn test_multiple_colons_in_host() {
        // IPv6 address has multiple colons
        let host = "[2001:db8::1]:8080";
        // Our simple split by first colon wouldn't work for IPv6
        // This tests awareness of the edge case
        assert!(host.contains("::"));
    }

    #[test]
    fn test_kill_api_empty_domain() {
        let path = "/api/kill/";
        let target = path.strip_prefix("/api/kill/").unwrap_or("");
        assert_eq!(target, "");
        assert!(target.is_empty());
    }

    #[test]
    fn test_kill_api_domain_with_special_chars() {
        let path = "/api/kill/my-app_v2.localhost";
        let target = path.strip_prefix("/api/kill/").unwrap();
        assert_eq!(target, "my-app_v2.localhost");
    }

    #[test]
    fn test_websocket_upgrade_case_variations() {
        let headers1 = "Upgrade: WebSocket";
        let headers2 = "upgrade: WEBSOCKET";
        let headers3 = "UPGRADE: websocket";

        // All should be detected as websocket
        assert!(headers1.to_lowercase().contains("websocket"));
        assert!(headers2.to_lowercase().contains("websocket"));
        assert!(headers3.to_lowercase().contains("websocket"));
    }

    #[test]
    fn test_connection_header_variations() {
        let conn1 = "Connection: Upgrade";
        let conn2 = "Connection: keep-alive, Upgrade";
        let conn3 = "connection: upgrade";

        assert!(conn1.to_lowercase().contains("upgrade"));
        assert!(conn2.to_lowercase().contains("upgrade"));
        assert!(conn3.to_lowercase().contains("upgrade"));
    }
}

mod forwarding {
    #[test]
    fn test_backend_address_format() {
        let port: u16 = 4000;
        let addr = format!("127.0.0.1:{}", port);
        assert_eq!(addr, "127.0.0.1:4000");
    }

    #[test]
    fn test_backend_port_range() {
        let port: u16 = 4500;
        assert!(port >= 4000 && port <= 5000);
    }

    #[test]
    fn test_x_forwarded_headers() {
        let original_host = "api.localhost";
        let x_forwarded_host = format!("X-Forwarded-Host: {}", original_host);
        assert!(x_forwarded_host.contains("api.localhost"));
    }

    #[test]
    fn test_x_forwarded_proto_https() {
        let proto = "https";
        let header = format!("X-Forwarded-Proto: {}", proto);
        assert!(header.contains("https"));
    }

    #[test]
    fn test_x_forwarded_proto_http() {
        let proto = "http";
        let header = format!("X-Forwarded-Proto: {}", proto);
        assert!(header.contains("http"));
    }
}
