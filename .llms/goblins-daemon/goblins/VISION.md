# goblins — VISION

## Identidade

Um Codex CLI fork onde múltiplas janelas compartilham um backend persistente — como o
OpenCode v2, mas nativo no Codex CLI.

## Norte

1. **v1 (atual):** Linux daemon mode. Primeira janela auto-spawna o daemon, demais conectam.
   Fechar qualquer janela não mata o backend. Threads e sessões persistem em memória.
2. **v2:** Windows daemon mode. Named pipe + CreateProcess detached. Mesma UX, adaptada
   para Windows semantics. Defer — sem vantagem imediata.
3. **v3+:** Frontend web/desktop como cliente do app-server. O protocolo v2 já é WebSocket-
   capable. Um frontend web seria só um cliente WS do daemon.

## Por que Goblins adapta o daemon existente (em vez de reescrever)

O Codex CLI 0.142.3 já tem `codex-app-server-daemon` — um supervisor completo com setsid,
pidfile, flock, lifecycle commands, e auto-conexão do TUI. Reescrever isso seria duplicar
código upstream e criar drift. O trabalho do Goblins é adaptar três pontos: binary resolution
(npm vs install.sh), auto-start no TUI, e UX de CLI. Tudo o mais já funciona.

## Por que não modificar o protocolo v2

O protocolo v2 já suporta `thread_list`, `thread_read`, múltiplos clientes simultâneos,
notifications broadcast. O app-server já é multi-cliente por design. O problema é
puramente de orquestração de processo, não de protocolo.

## Por que Linux first

O daemon existente já é Unix-only (`ensure_supported_platform()` retorna erro em não-Unix).
O usuário está em Linux. Windows não tem vantagem imediata. Epic separado, implementado
quando houver demanda real.
