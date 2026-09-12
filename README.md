# rusty-chat

En enkel sanntidschat bygget for å lære async Rust. Backend i
[axum](https://github.com/tokio-rs/axum) med WebSockets, frontend i vanilla JS.

Læringsprosjekt — skrevet fra bunnen av, inspirert av
[tinrab/rusty-chat](https://github.com/tinrab/rusty-chat), men med dagens API-er.

## Kjør

```bash
cargo run
```

Åpne <http://127.0.0.1:3000>. Test med flere faner for å se meldinger gå mellom
klienter.

## Stack

- **axum** — HTTP-ruting og WebSocket-oppgradering
- **tokio** — async runtime, `broadcast`-kanal for kringkasting
- **tower-http** — `ServeDir` for statiske filer
- **serde / serde_json** — meldingsprotokoll
- Frontend: vanilla HTML/CSS/JS, ingen byggesteg

## Hvordan det virker

Én `broadcast`-kanal deles av alle tilkoblinger via `Arc<AppState>`. Hver klient
får sin egen task som kjører `tokio::select!` på to kilder samtidig: meldinger
fra klientens socket, og meldinger fra kanalen.

Socketen splittes med `split()` slik at lesing og skriving kan skje uavhengig.
Kommer det en melding fra klienten, legges den ut på kanalen. Kommer det noe på
kanalen, skrives det ut til klienten.

Brukernavnet lagres som en lokal variabel i tilkoblingens egen task — det
tilhører én klient, ikke hele appen.

## Protokoll

Klient → server:

```json
{"type": "join", "username": "magnus"}
{"type": "chat", "text": "hei"}
```

Server → klient:

```json
{"type": "userJoined", "username": "magnus"}
{"type": "chat", "username": "magnus", "text": "hei"}
{"type": "userLeft", "username": "magnus"}
```

Definert som enums i `src/protocol.rs` med `#[serde(tag = "type")]`.

## Struktur

```
src/
├── main.rs       # oppsett, ruting, WebSocket-håndtering
└── protocol.rs   # ClientMessage / ServerMessage
static/
├── index.html
├── app.js
└── style.css
```

## Videre

- [ ] Flere rom (`HashMap<String, broadcast::Sender<_>>`)
- [ ] Meldingshistorikk med SQLite
- [ ] Liste over påloggede brukere
- [ ] Dockerfile og deploy
