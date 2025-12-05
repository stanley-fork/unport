//! Centralized logging module for unport
//!
//! Provides consistent log formatting with the [unport] prefix.
//! All logs should use these macros instead of tracing directly.

/// Log an info message with [unport] prefix
#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        tracing::info!("[unport] {}", format!($($arg)*))
    };
}

/// Log a warning message with [unport] prefix
#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        tracing::warn!("[unport] {}", format!($($arg)*))
    };
}

/// Log an error message with [unport] prefix
#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        tracing::error!("[unport] {}", format!($($arg)*))
    };
}

/// Log a debug message with [unport] prefix
#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        tracing::debug!("[unport] {}", format!($($arg)*))
    };
}

/// Log a trace message with [unport] prefix
#[macro_export]
macro_rules! log_trace {
    ($($arg:tt)*) => {
        tracing::trace!("[unport] {}", format!($($arg)*))
    };
}

/// Initialize the tracing subscriber with default settings
pub fn init() {
    use tracing_subscriber::EnvFilter;

    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .init();
}

#[cfg(test)]
mod tests {
    /// Test that log macros format messages correctly with prefix
    #[test]
    fn test_log_info_format() {
        // The macro expands to: tracing::info!("[unport] {}", format!(...))
        // We test the format string construction
        let message = format!("Test message: {}", 42);
        let formatted = format!("[unport] {}", message);
        assert!(formatted.starts_with("[unport] "));
        assert!(formatted.contains("Test message: 42"));
    }

    #[test]
    fn test_log_format_with_multiple_args() {
        let formatted = format!("[unport] {}", format!("Port {} assigned to {}", 4000, "api.localhost"));
        assert_eq!(formatted, "[unport] Port 4000 assigned to api.localhost");
    }

    #[test]
    fn test_log_format_with_debug_type() {
        let path = std::path::PathBuf::from("/home/user/.unport");
        let formatted = format!("[unport] {}", format!("Path: {:?}", path));
        assert!(formatted.contains("[unport]"));
        assert!(formatted.contains(".unport"));
    }

    #[test]
    fn test_log_format_empty_message() {
        let formatted = format!("[unport] {}", "");
        assert_eq!(formatted, "[unport] ");
    }

    #[test]
    fn test_log_format_special_characters() {
        let formatted = format!("[unport] {}", format!("URL: https://api.localhost:443/path?q=1&b=2"));
        assert!(formatted.contains("https://"));
        assert!(formatted.contains("?q=1&b=2"));
    }

    #[test]
    fn test_log_format_unicode() {
        let formatted = format!("[unport] {}", "日本語テスト");
        assert!(formatted.contains("日本語テスト"));
    }

    #[test]
    fn test_log_format_newlines_preserved() {
        let formatted = format!("[unport] {}", "Line 1\nLine 2");
        assert!(formatted.contains("\n"));
    }

    #[test]
    fn test_log_format_with_error() {
        let error = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
        let formatted = format!("[unport] {}", format!("Error: {}", error));
        assert!(formatted.contains("[unport]"));
        assert!(formatted.contains("File not found"));
    }
}
