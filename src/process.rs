use anyhow::{Context, Result};
use std::process::{Child, Command, Stdio};

use crate::detect::PortStrategy;

/// Spawn an app process with port injection
pub fn spawn_app(
    command: &str,
    port: u16,
    port_strategy: &PortStrategy,
    port_env_override: Option<&str>,
    port_arg_override: Option<&str>,
) -> Result<Child> {
    let mut parts: Vec<&str> = command.split_whitespace().collect();
    if parts.is_empty() {
        anyhow::bail!("Empty command");
    }

    let program = parts.remove(0);
    let mut cmd = Command::new(program);
    cmd.args(&parts);
    cmd.stdin(Stdio::null());
    cmd.stdout(Stdio::inherit());
    cmd.stderr(Stdio::inherit());

    // Apply port injection based on strategy
    match (port_env_override, port_arg_override) {
        // User override: env var
        (Some(env_var), _) => {
            cmd.env(env_var, port.to_string());
        }
        // User override: CLI arg
        (_, Some(arg)) => {
            cmd.arg(arg);
            cmd.arg(port.to_string());
        }
        // Auto-detected strategy
        _ => match port_strategy {
            PortStrategy::EnvVar(var) => {
                cmd.env(var, port.to_string());
            }
            PortStrategy::CliFlag(flag) => {
                // Special case for Django: "0.0.0.0:" needs port appended directly
                if flag.ends_with(':') {
                    cmd.arg(format!("{}{}", flag, port));
                } else {
                    cmd.arg(flag);
                    cmd.arg(port.to_string());
                }
            }
        },
    }

    let child = cmd.spawn().context("Failed to spawn process")?;
    Ok(child)
}
