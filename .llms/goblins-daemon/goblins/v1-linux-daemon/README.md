# Epic v1-linux-daemon — Daemon Mode Linux

Status: rascunho
Prioridade: lançamento-bloqueante
Depende de: nenhum (foundation epic)
Habilita: v2-windows-daemon (mesma superfície de CLI, adaptada para Windows)
Skills relacionadas: nenhuma

## Arquitetura

O daemon supervisor (`codex-app-server-daemon`) já existe no 0.142.3 e funciona. Este epic
**adapta** três pontos para o modelo de instalação do Goblins (npm) e UX de auto-start.
Não reimplementation — mudanças cirúrgicas em código existente.

### Mudanças por arquivo

1. **`codex-rs/app-server-daemon/src/lib.rs`** (modificar) — `Daemon::from_environment()`
   resolve binary via `current_exe()` quando managed install não existe. Remover
   `ensure_managed_codex_bin()` de `start()`/`restart()`.

2. **`codex-rs/tui/src/lib.rs`** (modificar) — `run_main()`: quando
   `maybe_probe_default_daemon_socket()` retorna `None` e `daemon.auto_start = true`,
   chamar `codex_app_server_daemon::run(Start)` antes de re-probe do socket.

3. **`codex-rs/cli/src/main.rs`** (modificar) — adicionar subcomando `daemon` como alias
   para `app-server daemon`. Flags `--no-daemon`.

4. **`codex-rs/core/src/config/mod.rs`** (modificar) — struct `DaemonConfig` com
   `auto_start`, `idle_shutdown_minutes`.

5. **`codex-rs/core/config.schema.json`** (regenerar) — `just write-config-schema`.

### Data flow

```
goblin (TUI)
  │
  ├─ maybe_probe_default_daemon_socket() → Some(socket) → LocalDaemon (conecta)
  │
  └─ maybe_probe_default_daemon_socket() → None
       │
       ├─ daemon.auto_start = false → Embedded (fallback, como hoje)
       │
       └─ daemon.auto_start = true
            │
            ├─ codex_app_server_daemon::run(Start)
            │   ├─ from_environment() → resolve binary (current_exe fallback)
            │   ├─ spawn detached (setsid) → processo daemon em background
            │   └─ wait_until_ready() → poll socket (50ms, 10s timeout)
            │
            ├─ Start OK → re-probe socket → LocalDaemon (conecta)
            └─ Start falha → Embedded (com warning no TUI)
```

## Escopo

### ADICIONAR
- Fallback de binary resolution: `current_exe()` quando managed install não existe
- Auto-start do daemon no TUI: `run_main()` chama `daemon::run(Start)` se config permite
- Subcomando `goblin daemon` (alias para `app-server daemon`)
- Flag `--no-daemon` (força Embedded, ignora daemon existente)
- Seção `[daemon]` em config.toml: `auto_start` (default true), `idle_shutdown_minutes` (default 0)
- Testes: binary resolution fallback, auto-start flow, CLI alias

### REFACTORIZAR
- `Daemon::from_environment()` — resolver binary em tempo de construção, não em `start()`
- `ensure_managed_codex_bin()` — remover ou adaptar para validar binary resolvido
(Refactor = preserve behavior quando managed install existe; adicionar fallback quando não.)

### REMOVER
- Nada. Managed install path continua funcionando como prioridade. Fallback é aditivo.

### MANTÉM
- `codex-app-server-daemon` backend/pid.rs — spawn com setsid, pidfile, flock (sem mudanças)
- `codex-app-server-daemon` client.rs — probe e health check (sem mudanças)
- `AppServerTarget::LocalDaemon` — já funciona (sem mudanças)
- `maybe_probe_default_daemon_socket()` — já funciona (sem mudanças)
- Protocolo v2 — sem mudanças
- `codex-core` — sem mudanças
- `goblin.js` — sem mudanças

## Commands

### `goblin daemon start` (alias para `goblin app-server daemon start`)
- Resolve binary (managed install ou current_exe)
- Spawn detached com setsid
- Escreve pidfile, segura lock
- Faz bind no socket
- Redireciona stderr para log
- `wait_until_ready()` — poll socket até 10s
- Se já rodando: "Daemon already running (PID N)", exit(0)

### `goblin daemon stop`
- Lê pidfile, envia graceful termination
- Aguarda grace period, depois força se necessário
- Limpa stale files

### `goblin daemon status`
- Probe socket + pid check
- Imprime JSON: status, pid, socket_path, version

### `goblin daemon restart`
- Stop + Start

### `goblin --no-daemon`
- Força `AppServerTarget::Embedded`
- Ignora socket existente e auto_start

## Business rules

- **Managed install tem prioridade** — se `$CODEX_HOME/packages/standalone/current/codex`
  existe, usar. Fallback para `current_exe()` só quando não existe.
- **Auto-start é opt-in via config** — `daemon.auto_start = true` (default). User pode
  desabilitar com `auto_start = false`.
- **`--no-daemon` tem precedência sobre config** — flag explícita sempre vence.
- **Fallback embedded é silencioso com warning** — se daemon falha, TUI mostra warning e
  usa embedded. Não é erro fatal.
- **Daemon é singleton** — flock no lock file previne double-spawn.
- **Binary resolvido em `from_environment()`** — não em `start()`, para que `stop()` e
  `status()` não precisem do binary (só do pidfile e socket).

## Features

- Usuário: `goblin` abre TUI. Se daemon não está rodando, sobe automaticamente. Se já está,
  conecta. Múltiplas janelas compartilham o mesmo backend.
- Usuário: `goblin daemon start` sobe o daemon explicitamente sem abrir TUI.
- Usuário: `goblin daemon stop` para o daemon.
- Usuário: `goblin --no-daemon` força modo isolado (embedded, como antes).
- Usuário: `daemon.auto_start = false` em config.toml desabilita auto-spawn.
- Interno: daemon resolve binary via current_exe quando npm-installed.

## Contratos

- [`contracts/binary-resolution.md`](./contracts/binary-resolution.md) — Como o daemon encontra o binary (managed install vs current_exe)

## Plano de execução

1. **Binary resolution** — modificar `from_environment()`, remover `ensure_managed_codex_bin()`
2. **Config** — `DaemonConfig` struct, seção `[daemon]`, schema
3. **Auto-start no TUI** — `run_main()` chama `daemon::run(Start)` quando socket vazio + auto_start
4. **CLI alias** — `goblin daemon` subcomando, flag `--no-daemon`
5. **Tests** — binary resolution, auto-start flow, CLI alias
6. **Docs** — SPECS.md, README.md, GOBLINS.md

## Tasks

- [tasks.md](./tasks.md) — Checklist granular por layer.

## Riscos e incertezas

- **[MEDIUM][Likely] `current_exe()` em symlink** — npm global install pode usar symlink.
  `current_exe()` resolve o real path, que pode apontar para um path temporário se o
  package manager usa staging dirs. — mitigação: testar com npm global install real;
  se problema, resolver via `goblin.js` passando o path via env var.

- **[LOW][Confirmed] Auto-start adiciona latência na primeira invocação** — `wait_until_ready()`
  tem 10s timeout. Primeiro `goblin` pode levar alguns segundos. — mitigação: timeout
  curto (10s), fallback embedded se excede. Subsequentes invocações são instantâneas
  (socket já existe).

- **[LOW][Possible] Daemon spawnado por TUI herda environment** — o processo detached
  herda env vars do TUI. Se o TUI foi invocado com env vars específicas (ex: API key),
  o daemon as recebe. — mitigação: comportamento correto (daemon precisa das mesmas
  creds). Não é bug.

- **UNVERIFIED**: O `codex_message_processor` isola estado entre múltiplos clientes
  conectados ao mesmo socket? O acceptor aceita em loop, mas preciso confirmar que
  threads de clientes diferentes não compartilham estado mutável indevidamente.
  [provenance: code — aceita em loop, isolamento não verificado]

- **Human decision required**: Default de `auto_start` — `true` (sobe automaticamente)
  ou `false` (user roda `goblin daemon start` explicitamente)? Recomendação: `true`
  (UX seamless). [provenance: conversation]
