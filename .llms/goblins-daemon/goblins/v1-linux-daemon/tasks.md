# Tasks — v1-linux-daemon

## Layer 1 — Binary resolution
- [ ] Modificar `Daemon::from_environment()` em `codex-rs/app-server-daemon/src/lib.rs`: resolver `managed_codex_bin` via `current_exe()` quando managed install não existe
- [ ] Remover chamada `ensure_managed_codex_bin()` de `start()` e `restart()` (binary já resolvido em `from_environment()`)
- [ ] Adicionar teste unitário: `from_environment()` usa managed install quando existe
- [ ] Adicionar teste unitário: `from_environment()` usa `current_exe()` quando managed install não existe

## Layer 2 — Config
- [ ] Criar struct `DaemonConfig` em `codex-rs/core/src/config/mod.rs` com campos `auto_start: bool` (default true), `idle_shutdown_minutes: u64` (default 0)
- [ ] Adicionar seção `[daemon]` ao parsing de config.toml
- [ ] Rodar `just write-config-schema` para atualizar `codex-rs/core/config.schema.json`
- [ ] Adicionar teste: config default `auto_start = true`, `idle_shutdown_minutes = 0`

## Layer 3 — Auto-start no TUI
- [ ] Em `codex-rs/tui/src/lib.rs` `run_main()`: quando `maybe_probe_default_daemon_socket()` retorna `None` e `daemon.auto_start = true`, chamar `codex_app_server_daemon::run(LifecycleCommand::Start)`
- [ ] Após `Start` OK, re-probe do socket (`maybe_probe_default_daemon_socket()`) e usar `LocalDaemon`
- [ ] Se `Start` falha ou timeout, logar warning e cair para `Embedded`
- [ ] Respeitar flag `--no-daemon`: pular auto-start e forçar `Embedded`
- [ ] Adicionar teste: auto-start chama `daemon::run(Start)` quando socket vazio + auto_start true
- [ ] Adicionar teste: `--no-daemon` pula auto-start

## Layer 4 — CLI alias
- [ ] Adicionar subcomando `daemon` em `codex-rs/cli/src/main.rs` como alias para `app-server daemon`
- [ ] Mapear `goblin daemon start|stop|restart|status` para `AppServerDaemonSubcommand` existente
- [ ] Adicionar flag `--no-daemon` no `Cli` struct do TUI
- [ ] Adicionar teste: `goblin daemon start` parseia como `app-server daemon start`

## Layer 5 — Idle shutdown (opcional, se idle_shutdown_minutes > 0)
- [ ] No daemon, adicionar timer que conta minutos sem clientes conectados
- [ ] Se `idle_shutdown_minutes > 0` e 0 clientes por N minutos, graceful shutdown
- [ ] Adicionar teste: idle shutdown trigga após N minutos sem clientes

## Layer 6 — Tests de integração
- [ ] Test: `daemon start` → socket existe → `daemon status` retorna Running → `daemon stop` → socket não existe
- [ ] Test: TUI auto-start spawna daemon, conecta, segunda invocação TUI reusa daemon
- [ ] Test: `--no-daemon` força Embedded mesmo com daemon rodando
- [ ] Test: binary resolution fallback (mock managed install ausente)

## Validação
- [ ] `cargo test -p codex-app-server-daemon` verde
- [ ] `cargo test -p codex-cli` verde
- [ ] `cargo test -p codex-tui` verde
- [ ] `cargo build --release --bin codex` verde
- [ ] Smoke: `goblin daemon start && goblin daemon status && goblin daemon stop`

## Specs e docs
- [ ] Atualizar `goblins/SPECS.md` com mudanças implementadas
- [ ] Atualizar `goblins/README.md` estado atual
- [ ] Atualizar `GOBLINS.md` com seção sobre daemon mode
- [ ] Atualizar root `README.md` status table

## Tarefas operacionais (humanas)
- [ ] (HUMAN) Decidir default de `auto_start` (true vs false) — type: product-decision — blocking: Layer 3
- [ ] (HUMAN) Smoke test manual em terminal real: abrir 3 janelas, verificar compartilhamento — type: manual-verify — blocking: none
