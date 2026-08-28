# goblins

## O que é

Goblins é um fork do OpenAI Codex CLI 0.142.3. Mantém a arquitetura Rust upstream intacta,
mas troca o package npm (`@brasalabs/goblins`), comando (`goblin`), release surface, README
e personalidade do agent. O binary interno continua se chamando `codex`.

## Papel no ecossistema

O Goblins é o único repo deste sistema. O daemon mode adapta o daemon supervisor existente
do Codex CLI (`codex-app-server-daemon`) para o modelo de instalação via npm.

```
┌─────────────────────────────────────────────────────┐
│  Terminal 1 (TUI)  ──┐                              │
│  Terminal 2 (TUI)  ──┼──► Unix Socket ──► App-Server (daemon)
│  Terminal 3 (TUI)  ──┘     $CODEX_HOME/       (background)
│                            app-server-control/  persists across
│                            app-server-control   terminal close
│                            .sock                (setsid detached)
└─────────────────────────────────────────────────────┘
```

## Stack

- **Runtime:** Rust 1.95.0 (codex-rs workspace), Node.js (codex-cli npm wrapper)
- **Language:** Rust, TypeScript/ESM (goblin.js)
- **Framework:** tokio, ratatui (TUI), clap (CLI)
- **Transport:** Unix domain socket (Linux), WebSocket (remote), stdio (default)
- **Key deps:** codex-core, codex-app-server, codex-app-server-daemon, codex-app-server-client, codex-tui

## Estado atual

- **Package:** `@brasalabs/goblins@0.142.3`
- **Repo:** `/home/guilherme/brainstorm/goblins` (GitHub: `brasalabs6/goblins`)
- **Branch:** `goblins` (default), upstream `main` é mirror
- **Daemon supervisor existe:** `codex-rs/app-server-daemon/` — pidfile, setsid, lock, lifecycle
- **TUI auto-conecta:** `maybe_probe_default_daemon_socket()` procura socket e conecta
- **`AppServerTarget::LocalDaemon`** já existe e funciona
- **Comando existe:** `goblin app-server daemon start|stop|restart|status`

### O que NÃO funciona para Goblins
- `ensure_managed_codex_bin()` exige binary em `$CODEX_HOME/packages/standalone/current/codex`
  (path do `install.sh`). Goblins é npm-installed — binary está em `node_modules/.../vendor/`.
- TUI não auto-spawna o daemon — só conecta se socket já existir
- Comando é verboso: `goblin app-server daemon start` (não `goblin daemon start`)

## Issues conhecidos

- Daemon não encontra o binary do Goblins (managed install path mismatch)
- Sem auto-start: user precisa rodar `goblin app-server daemon start` manualmente
- Sem config `[daemon]` para controlar auto_start / idle_shutdown
- Comando verboso sem atalho `goblin daemon`

## Epics

- [v1-linux-daemon](./v1-linux-daemon/) — Adaptar daemon para npm install + auto-start + atalho CLI
- [v2-windows-daemon](./v2-windows-daemon/) — Windows daemon (defer, sem vantagem imediata)
