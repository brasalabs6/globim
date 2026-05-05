# Model Providers And Auth

Auth and model-provider code spans login flows, token storage, external bearer
auth, ChatGPT backend clients, model catalogs, OpenAI-compatible APIs, Bedrock,
Ollama, LM Studio, and keyring persistence.

## Login And Token Storage

`codex-login` owns device-code/PKCE flows, auth manager behavior, external
bearer providers, token data, revocation, and storage. Token storage can use
filesystem or keyring-backed modes depending on config and platform support.

Source references:

- `codex-rs/login/src/lib.rs:1-50`
- `codex-rs/login/src/auth/manager.rs:48-55`
- `codex-rs/login/src/auth/manager.rs:723-777`
- `codex-rs/login/src/auth/manager.rs:1049-1067`
- `codex-rs/login/src/auth/manager.rs:1236-1416`
- `codex-rs/login/src/auth/manager.rs:1-880`
- `codex-rs/login/src/auth/storage.rs:31-48`
- `codex-rs/login/src/auth/storage.rs:125-153`
- `codex-rs/login/src/auth/storage.rs:220-357`
- `codex-rs/login/src/auth/storage.rs:1-361`
- `codex-rs/login/src/auth/external_bearer.rs:30-77`
- `codex-rs/login/src/auth/external_bearer.rs:105-174`
- `codex-rs/login/src/device_code_auth.rs:1-228`
- `codex-rs/login/src/pkce.rs:1-27`
- `codex-rs/login/src/token_data.rs:1-180`
- `codex-rs/keyring-store/src/lib.rs:1-226`
- `codex-rs/protocol/src/config_types.rs:369-402`

## Agent Identity

Agent Identity is a credential-bearing auth mode distinct from API-key and
ChatGPT token login. The stored JWT carries agent runtime identity, account/user
fields, plan metadata, and private key material. Runtime task authorization uses
that key material to sign task-scoped `AgentAssertion` headers after validating
runtime/task binding.

Source references:

- `codex-rs/agent-identity/src/lib.rs:64-78`
- `codex-rs/agent-identity/src/lib.rs:106-125`
- `codex-rs/agent-identity/src/lib.rs:128-170`
- `codex-rs/agent-identity/src/lib.rs:191-221`
- `codex-rs/login/src/auth/manager.rs:543-562`

## Model Providers

`codex-model-provider` abstracts auth and provider behavior. It includes
OpenAI-compatible provider wiring, bearer auth, the `/models` endpoint, and
Amazon Bedrock support. Model provider info maps provider capabilities and model
catalog metadata used by runtime selection and verification.

Source references:

- `codex-rs/model-provider-info/src/lib.rs:77-132`
- `codex-rs/model-provider-info/src/lib.rs:144-203`
- `codex-rs/model-provider-info/src/lib.rs:205-284`
- `codex-rs/model-provider-info/src/lib.rs:314-375`
- `codex-rs/model-provider-info/src/lib.rs:401-507`
- `codex-rs/model-provider/src/lib.rs:1-18`
- `codex-rs/model-provider/src/provider.rs:1-394`
- `codex-rs/model-provider/src/auth.rs:17-49`
- `codex-rs/model-provider/src/auth.rs:65-120`
- `codex-rs/model-provider/src/bearer_auth_provider.rs:1-102`
- `codex-rs/model-provider/src/models_endpoint.rs:1-127`
- `codex-rs/model-provider/src/amazon_bedrock/mod.rs:29-100`
- `codex-rs/model-provider/src/amazon_bedrock/auth.rs:22-138`
- `codex-rs/model-provider-info/src/lib.rs:1-511`

## Backend Clients

`codex-api` and `codex-client` own request transport, SSE/WebSocket handling,
telemetry headers, ChatGPT cookies/hosts, backend endpoint routing, rate limits,
and response endpoint wrappers. `codex-chatgpt` contains ChatGPT-specific
client helpers and workspace settings.

Source references:

- `codex-rs/codex-api/src/auth.rs:1-81`
- `codex-rs/codex-api/src/provider.rs:1-169`
- `codex-rs/codex-api/src/endpoint/models.rs:1-93`
- `codex-rs/codex-api/src/rate_limits.rs:1-202`
- `codex-rs/codex-client/src/default_client.rs:1-218`
- `codex-rs/codex-client/src/request.rs:1-215`
- `codex-rs/chatgpt/src/chatgpt_client.rs:1-68`
- `codex-rs/chatgpt/src/workspace_settings.rs:1-152`

## Local Providers

Ollama and LM Studio are local-provider paths with their own client, parser,
pull, and URL handling. Keep them provider-specific rather than mixing local
provider behavior into core auth.

Source references:

- `codex-rs/ollama/src/lib.rs:1-97`
- `codex-rs/ollama/src/client.rs:1-389`
- `codex-rs/ollama/src/parser.rs:1-75`
- `codex-rs/lmstudio/src/lib.rs:1-46`
- `codex-rs/lmstudio/src/client.rs:1-224`

## Invariants

- Do not log or document token values.
- Treat Agent Identity JWTs as sensitive because they can contain private key
  material, not only bearer identity metadata.
- Keep token refresh and storage policy behind auth manager/provider traits.
- Do not assume ChatGPT auth and OpenAI-compatible API-key auth share storage
  semantics.
- Provider auth resolution prefers provider bearer token, then first-party
  `CodexAuth`, then unauthenticated provider use.
- Custom providers extend built-ins; reserved built-ins are not overridden
  except for Bedrock `aws.profile` and `aws.region`.
- OpenAI first-party providers require first-party auth and can support
  WebSockets; OSS/local providers skip login/auth.
- Agent Identity auth must verify JWT issuer/audience/JWKS when possible and
  must keep task assertion signatures scoped to the registered task id.
- Preserve model catalog prompt overrides from Goblins when remote models are
  refreshed.
- Keep provider capability checks close to model/provider metadata.

## When Changing This

- Add auth manager/storage tests for login, refresh, revoke, and external bearer
  behavior.
- Add provider tests for model catalog, rate-limit, and capability changes.
- Update app-server account/model APIs if clients can observe the change.
- Recheck [Personality And Context](../prompts/personality-and-context.md) when
  model metadata changes affect prompt fields.
- `docs/authentication.md:1-3`
- `docs/config.md:79-99`

Read next:

- [Secrets And Proxy Boundaries](../safety/secrets-and-proxy-boundaries.md)
- [Sandbox, Permissions, And Guardian](../safety/sandbox-permissions-guardian.md)
