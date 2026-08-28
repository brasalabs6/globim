# Binary Resolution — Como o daemon encontra o binary do Goblins

**Fonte de verdade.** Este contrato define como o daemon resolve o path do binary `codex`
para spawnar o app-server em background.

## Estado atual (upstream 0.142.3)

`Daemon::from_environment()` em `codex-rs/app-server-daemon/src/lib.rs` define:

```rust
managed_codex_bin: managed_codex_bin(codex_home.as_path()),
```

`managed_codex_bin()` em `managed_install.rs` retorna:

```rust
codex_home.join("packages").join("standalone").join("current").join(managed_codex_file_name())
```

`ensure_managed_codex_bin()` verifica se o arquivo existe. Se não, erro:

```
managed standalone Codex install not found at {path}
This command requires the standalone install managed by the Codex installer
```

Este path é populado pelo `install.sh` do OpenAI. Goblins é instalado via
`npm install -g @brasalabs/goblins` — o binary está em
`node_modules/@brasalabs/goblins-linux-x64/vendor/x86_64-unknown-linux-musl/codex/codex`.

## Contrato novo: fallback para current_exe

### Resolução em ordem

1. **Managed install** — se `$CODEX_HOME/packages/standalone/current/codex` existe, usar.
   (Compatibilidade com install.sh — não quebra quem usa ambos.)
2. **Current exe** — se managed install não existe, usar `std::env::current_exe()`.
   (Goblins npm-installed — o binary que está rodando é o binary do daemon.)

### Implementação

Modificar `Daemon::from_environment()`:

```rust
fn from_environment() -> Result<Self> {
    let codex_home = find_codex_home()?;
    let socket_path = app_server_control_socket_path(codex_home.as_path())?;
    let state_dir = codex_home.as_path().join(STATE_DIR_NAME);

    let managed_codex_bin = managed_codex_bin(codex_home.as_path());
    let resolved_bin = if managed_codex_bin.is_file() {
        managed_codex_bin
    } else {
        std::env::current_exe()
            .context("failed to resolve current executable path for daemon backend")?
    };

    Ok(Self {
        socket_path,
        managed_codex_bin: resolved_bin,
        // ...
    })
}
```

### Modificar `ensure_managed_codex_bin()`

A função atual retorna erro se o managed path não existe. Com o fallback, ela não deve
mais ser chamada — ou deve ser adaptada para validar o `resolved_bin` em vez do path fixo.

Opção recomendada: remover a chamada `ensure_managed_codex_bin()` de `start()` e
`restart()`, pois `from_environment()` já resolveu o binary. Se o binary não existe,
o `spawn()` em `start_managed_backend()` falha com erro claro.

### Por que current_exe é seguro

- O binary `codex` é o mesmo que o user invocou como `goblin`
- `goblin.js` faz `spawn(binaryPath, ...)` — o binary path é resolvido pelo npm wrapper
- O daemon spawna `codex app-server --listen unix://...` — mesmo binary, mesmo comportamento
- Em npm global install, o binary é symlink ou cópia — `current_exe()` resolve o real path

### Edge case: binary movido/deletado após daemon start

Se o user faz `npm update -g @brasalabs/goblins` enquanto o daemon roda, o binary antigo
continua em memória (processo já spawnado). O próximo `daemon restart` usa o novo binary
(resolvido por `current_exe()` no novo processo `goblin daemon restart`). Comportamento
correto — não há stale binary.
