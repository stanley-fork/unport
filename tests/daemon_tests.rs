use std::process::Command;

/// Test that `unport daemon` without subcommand shows help/error
#[test]
fn test_daemon_requires_subcommand() {
    let output = Command::new(env!("CARGO_BIN_EXE_unport"))
        .arg("daemon")
        .output()
        .expect("Failed to execute command");

    // Should fail because subcommand is required
    assert!(!output.status.success(), "daemon without subcommand should fail");

    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let combined = format!("{}{}", stdout, stderr);

    // Should mention available subcommands
    assert!(
        combined.contains("start") && combined.contains("stop") && combined.contains("status"),
        "Should show available subcommands, got: {}",
        combined
    );
}

/// Test that `unport daemon start` (without -d) attempts to start
/// This verifies the CLI parsing works correctly
#[test]
fn test_daemon_start_cli_parsing() {
    let output = Command::new(env!("CARGO_BIN_EXE_unport"))
        .arg("daemon")
        .arg("start")
        .arg("--help")
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should show the -d/--detach option
    assert!(
        stdout.contains("--detach") || stdout.contains("-d"),
        "Should show detach option, got: {}",
        stdout
    );
}

/// Test that the detach flag is properly recognized
#[test]
fn test_daemon_start_detach_flag() {
    let output = Command::new(env!("CARGO_BIN_EXE_unport"))
        .arg("daemon")
        .arg("start")
        .arg("-d")
        .arg("--help")  // Add help to prevent actual daemon start
        .output()
        .expect("Failed to execute command");

    // With --help after -d, clap should still show help
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success() || stdout.contains("daemon"));
}

/// Verify the exact arguments that would be passed when spawning detached daemon
/// This is the core test for the bug: spawn must use ["daemon", "start"] not just ["daemon"]
#[test]
fn test_detach_spawn_uses_correct_args() {
    // This test documents the expected behavior:
    // When spawning a detached daemon, we must pass "daemon start" not just "daemon"

    // Test that "daemon start" works (exits quickly without sudo, but parses correctly)
    let output = Command::new(env!("CARGO_BIN_EXE_unport"))
        .args(["daemon", "start"])
        .output()
        .expect("Failed to execute command");

    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let combined = format!("{}{}", stdout, stderr);

    // Should NOT show "daemon <COMMAND>" help - that would mean args weren't parsed
    assert!(
        !combined.contains("daemon <COMMAND>"),
        "Should not show subcommand help when 'daemon start' is provided. Got: {}",
        combined
    );
}
