# Lockr

Zero Knowledge Passwort Manager mit Tauri und Rust. Diese Schablone ist startklar fuer lokale Entwicklung. UI ist minimal. Kryptografische Kernlogik liegt in `crates/core`.

## Ziele
Sichere Speicherung. Einfache Nutzung. Keine Telemetrie. Optionales Self Hosting fuer Sync spaeter.

## Funktionen in dieser Schablone
- Tresor Datei. Lokal. Verschluesselt mit Argon2 KDF und AEAD
- Erstellen Oeffnen Speichern
- Einfache Eintraege mit Title Username Password Url Notes
- CLI Testwerkzeug fuer Import und Dump
- Unit Tests fuer Krypto Parameter und Serialisierung
- CI Workflow fuer Build Test Lint Audit

## Tech Stack
- Rust 1.79 oder neuer
- Tauri als Desktop Rahmen fuer UI spaeter
- SQLCipher optional. Hier Standard sqlite mit verschluesseltem Blob
- Crates: argon2 chacha20poly1305 zeroize secrecy serde anyhow thiserror rusqlite

## Schnellstart
```bash
cargo build
cargo test
cargo run -p lockr-cli -- --help
```
UI kommt spaeter. Diese Schablone legt Fokus auf Kernlogik.

## Sicherheitsmodell
- Argon2id mit moderaten Parametern fuer Desktop
- AEAD mit ChaCha20 Poly1305 fuer Datenverschluesselung
- Memory Zeroization fuer geheime Werte
- Keine Netzwerk Funktion in dieser Stufe
- Threat Model siehe SECURITY.md

## Projektstruktur
```
Cargo.toml                Workspace
crates/core               Kryptokern
crates/cli                CLI Tool
apps/desktop              Tauri Platzhalter
.github/workflows         CI
```

## Roadmap kurz
1. Vault Kern fertigstellen
2. CLI erweitern
3. UI in Tauri aufsetzen
4. Sync Protokoll entwerfen
5. Security Review

## Lizenz
GPLv3
