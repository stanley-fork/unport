# HTTPS Support for Local Domains

## Overview

Add built-in HTTPS support for `*.localhost` domains using a self-signed CA, with TLS termination at the daemon.

## Architecture

The daemon runs two listeners:
- Port 80 → HTTP (existing)
- Port 443 → HTTPS (new, opt-in)

```
https://api.localhost/users
    ↓
unport daemon (port 443, TLS termination)
    ↓ decrypts, routes by Host header
http://127.0.0.1:4000  (backend app, plain HTTP)
```

Apps don't change - they still run HTTP. The daemon handles TLS termination.

## File Structure

```
~/.unport/
├── ca.key              # CA private key
├── ca.crt              # CA certificate (user trusts this once)
├── certs/
│   ├── localhost.key   # Wildcard cert private key
│   └── localhost.crt   # Wildcard cert for *.localhost
```

## Implementation

### New Dependencies

```toml
rcgen = "0.12"           # Certificate generation
tokio-rustls = "0.25"    # Async TLS
rustls-pemfile = "2"     # PEM parsing
```

### New Modules

**src/tls.rs:**
```rust
pub fn ensure_ca() -> Result<()>           // Create CA if missing
pub fn ensure_cert() -> Result<()>         // Create *.localhost cert if missing
pub fn load_tls_config() -> Result<ServerConfig>  // Load for HTTPS server
```

### CLI Changes

**Daemon flag:**
```bash
sudo unport daemon start -d --https    # Enable HTTPS on port 443
sudo unport daemon start -d            # HTTP only (default)
```

**Trust command:**
```bash
unport trust-ca          # Add CA to system trust store
unport trust-ca --remove # Remove CA from trust store
```

### Trust Store Handling

**macOS:**
```bash
security add-trusted-cert -d -r trustRoot -k /Library/Keychains/System.keychain ca.crt
```

**Linux:**
```bash
cp ca.crt /usr/local/share/ca-certificates/unport-ca.crt
update-ca-certificates
```

**Firefox (NSS):**
```bash
certutil -A -n "unport CA" -t "C,," -i ca.crt -d ~/.mozilla/firefox/*.default*/
```

## User Experience

**First run:**
```bash
$ sudo unport daemon start -d --https
Generated CA certificate at ~/.unport/ca.crt
Generated wildcard certificate for *.localhost

Daemon started in background.
  HTTP:  http://*.localhost  (port 80)
  HTTPS: https://*.localhost (port 443)

⚠️  To trust HTTPS in browsers, run: sudo unport trust-ca
```

**After trusting:**
```bash
$ sudo unport trust-ca
Added CA to system trust store.
Added CA to Firefox (NSS) trust store.

✓ https://*.localhost is now trusted
```

## Error Handling

| Scenario | Behavior |
|----------|----------|
| Port 443 in use | Error with suggestion to stop conflicting process |
| CA already trusted | "CA already trusted. Nothing to do." |
| HTTPS without trust | Daemon starts, browsers show warning |
| Firefox without certutil | Warn to install libnss3-tools |

## Testing

**Unit tests:**
- CA generation produces valid X.509
- Wildcard cert signed by CA
- Cert has correct SANs: `*.localhost`, `localhost`

**Integration tests:**
- HTTPS listener accepts connections
- TLS handshake succeeds
- Proxy routes HTTPS → HTTP backend
- WebSocket upgrade works over wss://

**Manual checklist:**
- [ ] Chrome trusts CA
- [ ] Firefox trusts CA
- [ ] Safari trusts CA
- [ ] curl works with CA
- [ ] Mixed HTTP/HTTPS works

## Out of Scope

- Cert rotation (localhost certs can be long-lived)
- Multiple CAs
- Custom domains outside .localhost
- Let's Encrypt integration
