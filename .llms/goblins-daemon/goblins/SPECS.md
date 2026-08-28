# goblins — SPECS

## 1. Daemon Supervisor (existente — codex-app-server-daemon)

O crate `codex-app-server-daemon` já implementa:
- Pidfile-backed daemonização com `setsid()` via `pre_exec` (`backend/pid.rs`)
- Operation lock (`daemon.lock`) com flock para serializar lifecycle commands
- Lifecycle: `Start`, `Stop`, `Restart`, `Version` (`lib.rs::run()`)
- Socket probe + health check (`client.rs::probe()`)
- `wait_until_ready()` — poll socket a 50ms até 10s timeout
- State dir: `$CODEX_HOME/app-server-daemon/` (pid, lock, settings.json)
- Stderr log redirect para arquivo
- Unix-only (`ensure_supported_platform()` erro em não-Unix)

### Gap: binary resolution
`ensure_managed_codex_bin()` procura `$CODEX_HOME/packages/standalone/current/codex`.
Goblins é npm-installed. Solução: fallback para `std::env::current_exe()`.

## 2. CLI Surface

### Comandos existentes
- `goblin` (sem subcomando) → TUI, `AppServerTarget::Embedded` ou `LocalDaemon` (se socket existe)
- `goblin --remote ws://host:port` → TUI remoto
- `goblin app-server daemon start|stop|restart|status` → lifecycle do daemon

### Comandos novos (v1-linux-daemon)
- `goblin daemon start|stop|restart|status` → atalho para `app-server daemon`
- `goblin --no-daemon` → força Embedded, ignora daemon existente
- Auto-start: `goblin` (sem flags) spawna daemon se `daemon.auto_start = true`

## 3. AppServerTarget (TUI — existente)

```rust
pub(crate) enum AppServerTarget {
    Embedded,
    LocalDaemon { endpoint: RemoteAppServerEndpoint },
    Remote { endpoint: RemoteAppServerEndpoint },
}
```

`LocalDaemon` já funciona. `maybe_probe_default_daemon_socket()` já conecta.
Gap: não chama `daemon::run(Start)` quando socket não existe.

## 4. Configuração (nova)

```toml
[daemon]
auto_start = true           # auto-spawnar daemon na primeira invocação (default: true)
idle_shutdown_minutes = 0   # shutdown após N min sem clientes (0 = nunca, default: 0)
```

## 5. Validação

- `cargo test -p codex-app-server-daemon` — testes do daemon
- `cargo test -p codex-cli` — testes do CLI
- `cargo test -p codex-tui` — testes do TUI
- `cargo build --release --bin codex` — build de release
- Smoke: `goblin daemon start && goblin daemon status && goblin daemon stop`

## Infra

- **Runtime:** processo Rust detached (daemon), processo Rust (TUI cliente)
- **IPC:** Unix domain socket (SOCK_STREAM) em `$CODEX_HOME/app-server-control/`
- **Locking:** flock advisory lock em `daemon.lock`
- **Signals:** SIGTERM/SIGINT (graceful shutdown via daemon stop command)
- **Binary:** `std::env::current_exe()` resolve o binary `codex` em runtime
