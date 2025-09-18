# Lockr
Zero Knowledge Passwort Manager mit Tauri und Rust.

## Ziel
Passwoerter sicher speichern mit starker Kryptografie und optionaler Synchronisation ueber eigenes Backend.

## Features
* Tresor lokal mit Argon2 KDF
* Auto Lock und Clipboard Hygiene
* Suche und Tags
* Passwort Generator
* Optional Sync ueber Ende zu Ende

## Tech Stack
* Rust
* Tauri
* SQLCipher oder sqlite normal mit libsodium Secretbox
* Rust Crypto

## Kritische Crates
* argon2
* rand
* chacha20poly1305 oder libsodium
* secrecy
* sqlx
* tauri
* zeroize

## Env Variablen
* Keine zwingen erforderlich lokal

## Ordnerstruktur
* src app
* crates core crypto storage

## Tests
* Krypto Property Tests
* Fuzzing fuer Parser

## CI
* Rust stable clippy fmt test
* Supply Chain Audit mit cargo audit

## Security
* Memory Zeroization
* Lock Screen
* Keine Telemetrie
* Threat Model in docs

## Roadmap
* Mobile Client
* Hardware Key Support

## Lizenz
GPLv3
