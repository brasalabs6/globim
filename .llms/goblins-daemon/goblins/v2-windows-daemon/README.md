# Epic v2-windows-daemon — Daemon Mode Windows

Status: planejado
Prioridade: pós-lançamento
Depende de: v1-linux-daemon (mesma superfície de CLI, adaptada para Windows)
Habilita: nenhum

## Arquitetura

O daemon supervisor existente (`codex-app-server-daemon`) é Unix-only.
`ensure_supported_platform()` retorna erro em não-Unix:

```rust
#[cfg(not(unix))]
fn ensure_supported_platform() -> Result<()> {
    Err(anyhow!("codex app-server daemon lifecycle is only supported on Unix platforms"))
}
```

O `backend/pid.rs` usa `libc::setsid()` via `pre_exec`, que não existe em Windows.

### Mudanças necessárias para Windows

1. **Process spawn** — substituir `setsid()` + `pre_exec` por `CreateProcess` com
   `DETACHED_PROCESS` + `CREATE_NO_WINDOW` flags. O `std::process::Command` no Windows
   suporta `creation_flags` via `std::os::windows::process::CommandExt`.

2. **IPC transport** — substituir Unix domain socket por named pipe
   (`\\.\pipe\goblins-daemon`). O `AppServerTransport` precisaria de uma variante
   `NamedPipe { pipe_name: String }`.

3. **Pidfile** — Windows não tem `kill(pid, 0)` para verificar processo vivo. Usar
   `OpenProcess` + `GetExitCodeProcess`, ou WMI.

4. **Lock file** — `flock` não existe em Windows. Usar `LockFileEx` via
   `std::os::windows::io`.

5. **Signals** — Windows não tem SIGTERM/SIGINT para processos arbitrários. Usar
   `GenerateConsoleCtrlEvent` ou um named event para graceful shutdown.

## Escopo

### ADICIONAR
- `#[cfg(windows)]` impl de spawn detached com `CREATE_NO_WINDOW`
- Variante `AppServerTransport::NamedPipe`
- Windows pidfile com `OpenProcess`
- Windows lock file com `LockFileEx`
- Named event para graceful shutdown signal

### MANTÉM
- CLI surface (`goblin daemon start|stop|restart|status`) — mesma UX
- Config `[daemon]` — mesma estrutura
- Auto-start no TUI — mesma lógica, adaptada para named pipe probe
- Protocolo v2 — sem mudanças

## Riscos e incertezas

- **[HIGH][Confirmed] Windows process lifecycle é materialmente diferente** — sem setsid,
  sem signals POSIX, sem flock. Cada primitiva precisa de equivalente Windows nativo.
- **[MEDIUM][Likely] Named pipe vs Unix socket semantics** — named pipe tem comportamento
  diferente para multi-cliente (cada conexão precisa de uma instância de pipe).
- **[LOW][Possible] Sem demanda imediata** — usuário está em Linux. Windows é defer.

## Por que defer

O usuário está em Linux. Windows não tem vantagem imediata. O esforço de adaptar 5
primitivas Unix para Windows é significativo (spawn, IPC, pidfile, lock, signals) e não
bloqueia o caso de uso principal. Implementar quando houver demanda real de usuários
Windows.
