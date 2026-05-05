# Secrets And Proxy Boundaries

This reference covers security-sensitive support surfaces that are easy to miss
because they are not the main model runtime: local secret storage/redaction and
the standalone Responses API proxy.

## Local Secrets

`codex-secrets` owns secret names, scopes, local encrypted storage, keyring
account derivation, and best-effort redaction. Secret names are restricted to
`A-Z`, `0-9`, and `_`; scopes can be global or environment-specific. The local
backend encrypts a JSON secrets file with an `age` passphrase stored in the OS
keyring.

Source references:

- `codex-rs/secrets/src/lib.rs:24-93`
- `codex-rs/secrets/src/lib.rs:101-140`
- `codex-rs/secrets/src/lib.rs:142-180`
- `codex-rs/secrets/src/local.rs:68-180`
- `codex-rs/secrets/src/sanitizer.rs:13-22`

## Responses API Proxy

`codex-responses-api-proxy` is a standalone strict proxy for Responses API
traffic. It reads an API key from stdin, injects `Authorization: Bearer <key>`,
accepts only `POST /v1/responses`, rejects other requests, can write server info
and request/response dumps, and can expose an HTTP shutdown endpoint when
explicitly enabled.

Source references:

- `codex-rs/responses-api-proxy/README.md:31-67`
- `codex-rs/responses-api-proxy/README.md:71-80`
- `codex-rs/responses-api-proxy/README.md:91-110`

## Invariants

- Do not store secrets in plaintext config.
- Treat redaction as best effort; do not rely on it as the only protection for
  sensitive values.
- Secret names and scopes are part of the public storage contract.
- Responses API proxy callers must pipe the key through stdin and should not
  leave the key in the unprivileged user's environment.
- Request/response dump mode can contain sensitive payloads even when selected
  headers are redacted.
- `--http-shutdown` is an explicit cross-user control surface and should stay
  opt-in.

Read next:

- [Model Providers And Auth](../auth/model-providers-and-auth.md)
- [Sandbox, Permissions, And Guardian](sandbox-permissions-guardian.md)
- [Analytics, Feedback, And OTEL](../observability/analytics-feedback-otel.md)
