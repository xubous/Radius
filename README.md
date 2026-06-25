# RadiusRS — Servidor RADIUS em Rust

> Um servidor RADIUS completo, do zero em Rust, com API REST, interface web (React), terminal (TUI), IA e monitoramento.

---

## O que é RADIUS?

RADIUS é um protocolo de rede que **autentica** (verifica login/senha), **autoriza** (define permissões) e **contabiliza** (registra uso) conexões de rede.

**Exemplo real:** Quando você conecta no Wi-Fi da empresa e digita seu login, um servidor RADIUS valida suas credenciais.

**Como funciona:** um roteador/switch (chamado **NAS**) recebe a tentativa de conexão e envia um pacote UDP para o servidor RADIUS (porta **1812**). O servidor verifica se o usuário existe e responde: **Access-Accept** (pode entrar) ou **Access-Reject** (senha errada).

---

## O que cada parte do projeto faz?

| Componente | O que é | Para que serve |
|---|---|---|
| **RADIUS Core** | Implementação do protocolo na mão (bytes) | Entender como redes funcionam de verdade |
| **Hashing/Criptografia** | MD5, SHA-256, Argon2id, AES | Proteger senhas — obrigatório pelo protocolo e por segurança |
| **API REST** | Servidor HTTP com Axum | Permitir que o frontend e ferramentas externas consultem dados |
| **TUI** | Interface no terminal (ratatui) | Monitorar o servidor sem precisar de navegador |
| **Frontend React** | Dashboard web com gráficos | Visualizar métricas, usuários, sessões |
| **IA** | Análise de logs com ML | Detectar ataques e anomalias automaticamente |
| **Monitoramento** | Prometheus + tracing | Coletar métricas e logs estruturados |

> É um projeto ambicioso para uma pessoa, mas **cada fase é independente**. Você pode parar em qualquer etapa e já ter algo funcional.

---

## Conceitos que você precisa entender

### Pacote RADIUS

Tudo que o servidor recebe e envia é um pacote binário com essa estrutura:

```
| Código (1 byte) | ID (1 byte) | Tamanho (2 bytes) | Authenticator (16 bytes) | Atributos... |
```

- **Código:** tipo do pacote. `1` = Access-Request, `2` = Access-Accept, `3` = Access-Reject
- **ID:** número que emparelha a requisição com a resposta
- **Tamanho:** total de bytes do pacote
- **Authenticator:** 16 bytes calculados com MD5 (garante que o pacote não foi adulterado)
- **Atributos:** os dados (nome do usuário, senha, IP, etc.)

### Atributos (TLV — Type-Length-Value)

Cada atributo segue esse formato:

```
| Tipo (1 byte) | Tamanho (1 byte) | Valor (N bytes) |
```

Tipos importantes pra você:
- `1` User-Name — o login
- `2` User-Password — a senha **sempre criptografada**
- `4` NAS-IP-Address — IP do roteador
- `6` Service-Type — tipo de serviço solicitado

### Criptografia da senha (RFC 2865)

A senha **nunca** vai em texto puro no pacote. O RADIUS usa MD5 + um shared secret (senha combinada entre servidor e roteador) pra ofuscar:

```
bloco = MD5(shared_secret + authenticator)
senha_ofuscada = senha XOR bloco
```

Se a senha for maior que 16 bytes, calcula mais blocos.

### Hash de senhas armazenadas

Quando o usuário se cadastra, a senha dele **não** é salva em texto puro. Você guarda um hash:

- **Argon2id** — o mais seguro hoje, lento de propósito pra dificultar ataques
- **SHA-256** — mais rápido, menos seguro (use como fallback)

### Como o fluxo funciona

```
Cliente --> NAS (roteador) --> pacote UDP :1812 --> seu servidor Rust
                                                      |
                                               verifica shared secret
                                               decripta a senha
                                               confere o hash
                                                      |
                         <-- Access-Accept/Reject ----+
```

### O que faz a IA aqui?

A IA analisa os logs de autenticação pra encontrar padrões suspeitos:

- Muitas tentativas falhas seguidas (alguém tentando força bruta)
- IP estranho tentando acessar
- Horário atípico

Você pode implementar com regras simples primeiro (ex: "se falhou 5 vezes em 1 minuto, marca como anomalia") e depois evoluir pra um modelo de ML de verdade.

---

## Roadmap de implementação

Dividi em **fases**. Cada fase entrega algo funcional. Você pode parar em qualquer uma.

### Fase 1 — Núcleo RADIUS (viável em ~1 semana)

Objetivo: servidor que recebe pacote UDP, valida senha e responde.

```
src/
├── main.rs          # loop UDP com tokio
├── packet.rs        # structs Packet, Attribute, Code
├── password.rs      # criptografia de senha (MD5 XOR)
├── auth.rs          # validação PAP
└── users.rs         # carrega usuários de um arquivo JSON/TOML
```

**Dependências:** tokio, bytes, md-5, serde, serde_json, tracing, thiserror

**Teste:** `cargo run` + `radtest usuario senha localhost 1812 segredo 10`

### Fase 2 — Hashing e segurança (~dias)

Melhorar o armazenamento de senhas:
- Hash com Argon2id no cadastro
- SHA-256 como fallback
- AES-256-GCM pra criptografar shared secrets armazenados
- Atributo `Message-Authenticator` (HMAC-MD5) pra integridade extra

### Fase 3 — API REST (~1 semana)

Adicionar Axum pra expor endpoints HTTP:

```
GET  /api/status         — se o servidor está rodando
GET  /api/users          — listar usuários
POST /api/users          — criar usuário
GET  /api/logs           — histórico de autenticações
GET  /api/metrics        — total de Accept/Reject, sessões ativas
```

**Dependências novas:** axum, tower-http, serde_json, sqlx (Postgres) ou sqlite

### Fase 4 — Frontend React (~1-2 semanas)

Interface web com:
- Dashboard com métricas
- Tabela de usuários (CRUD)
- Lista de logs
- Gráficos simples

**Stack:** React + TypeScript + Tailwind + Vite

Pode começar com componentes simples (uma tabela, uns cards) e depois incrementar.

### Fase 5 — TUI (~1 semana)

Interface no terminal com ratatui:
- Dashboard ASCII (Accept/Reject, sessões ativas)
- Logs ao vivo
- Tabela de usuários

### Fase 6 — IA (~1-2 semanas)

Comece com regras heurísticas simples:
```
if tentativas_falhas > 5 em 60s → anomalia
if IP em blacklist → malicioso
if horário_atípico && primeira_vez → suspeito
```

Depois evolua pra um modelo Isolation Forest (biblioteca `linfa`) ou chame uma API OpenAI.

### Fase 7 — Monitoramento e Docker

- Prometheus pra métricas
- OpenTelemetry pra tracing
- docker-compose com app + postgres (+ redis opcional)

---

## O que você não precisa implementar agora (e talvez nunca)

- **EAP-TLS** (autenticação com certificado) — complexo, poucos cenários usam
- **CHAP** — método de autenticação legado, PAP já cobre o básico
- **Redis** — cache de sessões é otimização, não essencial
- **Grafana dashboards** — visual legal, mas você já tem React + TUI
- **Nagios plugin** — monitoramento enterprise, desnecessário
- **radius-bench** — benchmark, legal mas não essencial
- **EAP-MD5** — variante do EAP, pode pular

---

## Dependências completas (Cargo.toml)

```toml
[package]
name = "radius"
version = "0.1.0"
edition = "2021"

[dependencies]
# Async runtime
tokio = { version = "1", features = ["full"] }

# Protocolo RADIUS (manipulação de bytes)
bytes = "1"

# Hashing e criptografia
md-5 = "0.10"              # MD5 — obrigatório pelo RADIUS
sha2 = "0.10"              # SHA-256/512
argon2 = "0.5"             # Argon2id — hash de senha moderno
aes-gcm = "0.10"           # AES-256-GCM — criptografar secrets em repouso

# Web framework (API REST)
axum = "0.8"
tower-http = { version = "0.6", features = ["cors"] }

# Serialização
serde = { version = "1", features = ["derive"] }
serde_json = "1"

# Banco de dados
sqlx = { version = "0.8", features = ["runtime-tokio", "postgres", "chrono"] }

# Terminal UI
ratatui = "0.29"
crossterm = "0.28"

# IA
linfa = "0.7"              # ML em Rust (Isolation Forest)
reqwest = { version = "0.12", features = ["json"] }  # Chamar API externa

# Monitoramento
tracing = "0.1"
tracing-subscriber = "0.3"
metrics = "0.24"
metrics-exporter-prometheus = "0.16"

# Utilitários
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1", features = ["v4"] }
rand = "0.8"
thiserror = "2"
anyhow = "1"
serde_json = "1"
toml = "0.8"
```

---

## Por onde começar (guia prático)

1. **Leia a RFC 2865** — pelo menos as seções 1-5 (é mais curta do que parece)
2. **Implemente as structs** `Packet` e `Attribute` em Rust
3. **Faça o parse** de um pacote manual (bytes → struct)
4. **Implemente a criptografia** da senha (decrypt)
5. **Crie um arquivo de usuários** (JSON/TOML)
6. **Abra um socket UDP** com tokio e receba pacotes de verdade
7. **Teste com `radtest`** do freeradius-utils

Cada fase do roadmap te dá um marco concreto. Se travar em Rust, o [Rust Book](https://doc.rust-lang.org/book/) é seu melhor amigo.

---

## Referências

- [RFC 2865](https://datatracker.ietf.org/doc/html/rfc2865) — RADIUS Authentication
- [RFC 2866](https://datatracker.ietf.org/doc/html/rfc2866) — RADIUS Accounting
- [Rust Book](https://doc.rust-lang.org/book/)
- [Axum docs](https://docs.rs/axum/)
- [ratatui docs](https://docs.rs/ratatui/)
