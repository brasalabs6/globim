# Goblins Daemon Mode — Ecosystem

O Codex CLI 0.142.3 (base do Goblins) **já possui** um daemon supervisor completo:
`codex-app-server-daemon`. Ele faz pidfile-backed daemonização com `setsid()`, lock file,
lifecycle commands (`start`/`stop`/`restart`/`status`), e o TUI já auto-conecta ao socket
do daemon se ele existir (`AppServerTarget::LocalDaemon` + `maybe_probe_default_daemon_socket`).

O que **falta** para o caso de uso do Goblins (multi-janela compartilhando backend):

1. **Daemon exige managed install** — `ensure_managed_codex_bin()` procura o binary em
   `$CODEX_HOME/packages/standalone/current/codex` (path do `install.sh`). Goblins é
   instalado via `npm install -g @brasalabs/goblins`, não via `install.sh`. O daemon não
   consegue encontrar o binary para spawnar.

2. **TUI não auto-spawna o daemon** — `maybe_probe_default_daemon_socket()` só *conecta*
   se o socket existir. Se não existe, cai para `Embedded`. Não há auto-start.

3. **UX de CLI** — o comando é `goblin app-server daemon start` (verboso). Falta um
   atalho `goblin daemon start` e integração com o fluxo "abre terminal, digita goblin".

Este plano fecha esses três gaps. Não reimplementation — adaptação do daemon existente
para o modelo de instalação do Goblins (npm) e UX de auto-start.

## Repositório

| Repo | Papel | Stack | v1 (Linux) |
|---|---|---|---|
| `brasalabs6/goblins` | Fork do Codex CLI 0.142.3 com daemon mode adaptado | Rust + Node.js | ✅ implementar |

Sistema single-project. Contratos ficam em `goblins/v1-linux-daemon/contracts/`.

## Roadmap

### v1 — Linux Daemon (prioridade, implementar agora)
- Resolver o binary path: daemon usa o binary do Goblins (npm install), não managed install
- Auto-start do daemon na primeira invocação do TUI (se config `daemon.auto_start = true`)
- Atalho de CLI: `goblin daemon start|stop|status|restart`
- Config `[daemon]` em config.toml: `auto_start`, `idle_shutdown_minutes`
- Fallback gracioso: se daemon falha, TUI cai para embedded (como hoje)

### v2 — Windows Daemon (defer)
- O daemon existente é Unix-only (`ensure_supported_platform()` retorna erro em não-Unix)
- Windows precisaria de named pipe + CreateProcess detached
- Sem vantagem imediata — defer

## Estrutura de Documentação

```
.llms/goblins-daemon/
├── README.md                          # Este arquivo
├── goblins/
│   ├── README.md                      # O que é, papel, estado atual
│   ├── SPECS.md                       # Specs técnicas
│   ├── VISION.md                      # Visão de longo prazo
│   ├── v1-linux-daemon/
│   │   ├── README.md                  # Spec do epic (Escopo Pattern A)
│   │   ├── contracts/
│   │   │   └── binary-resolution.md   # Como o daemon encontra o binary do Goblins
│   │   └── tasks.md                   # Checklist granular por layer
│   └── v2-windows-daemon/
│       └── README.md                  # Spec do epic (planejado, defer)
```

## Status da documentação

| Componente | Docs base | Epics | Status |
|---|---|---|---|
| `goblins/` | README + SPECS + VISION | v1-linux-daemon, v2-windows-daemon | ✅ completo |

**Próximo passo:** Implementar v1-linux-daemon seguindo `tasks.md`. Branch
`feat/daemon-mode-linux`, PR para `goblins`.

## Princípios

1. **Não reimplementation** — o daemon supervisor (`codex-app-server-daemon`) já existe e
   funciona. Adaptar, não reescrever. [provenance: code]
2. **Zero breaking change no TUI embedded** — daemon mode é opt-in. Se o daemon falha,
   TUI cai para `Embedded` exatamente como hoje. [provenance: user-input]
3. **Não modificar o protocolo v2** — o app-server multi-cliente já existe. [provenance: code]
4. **Não modificar o codex-core** — toda lógica fica em `codex-rs/app-server-daemon/`,
   `codex-rs/cli/`, e `codex-rs/tui/`. [provenance: code]
5. **Linux first, Windows defer** — o daemon existente já é Unix-only. Windows é epic
   separado, sem vantagem imediata. [provenance: user-input]

## Decisões de design

### DD-1: Resolver binary path via current_exe, não managed install
- **Decisão:** Modificar `Daemon::from_environment()` para usar `std::env::current_exe()`
  (o binary `codex` que está rodando) em vez de `managed_codex_bin(codex_home)` quando o
  managed install não existe.
- **Contexto:** Goblins é instalado via npm. O binary está em
  `node_modules/@brasalabs/goblins-linux-x64/vendor/.../codex`, não em
  `$CODEX_HOME/packages/standalone/current/codex`.
- **Alternativas consideradas:**
  - Symlink do binary npm para o path managed — rejeitado: frágil, quebra em update do npm.
  - Config `daemon.binary_path` — rejeitado: exige config manual, quebra UX "funciona out of the box".
  - Env var `GOBLINS_DAEMON_BIN` — rejeitado: exige wrapper `goblin.js` setar env, complexidade desnecessária.
- **Status:** aceito [provenance: code + inferred]

### DD-2: Auto-start no TUI via config, default true
- **Decisão:** Adicionar `daemon.auto_start = true` (default) em config.toml. Quando o TUI
  não encontra daemon rodando e `auto_start = true`, chama `codex_app_server_daemon::run(Start)`
  antes de tentar conectar.
- **Contexto:** UX precisa ser "abre terminal, digita goblin, funciona" sem passo extra.
- **Status:** aceito [provenance: conversation]

### DD-3: Atalho `goblin daemon` vs `goblin app-server daemon`
- **Decisão:** Adicionar `goblin daemon` como atalho para `goblin app-server daemon`.
  Ambos funcionam. `goblin daemon` é o canonical para Goblins.
- **Contexto:** `goblin app-server daemon start` é verboso. `goblin daemon start` é natural.
- **Status:** aceito [provenance: inferred]
