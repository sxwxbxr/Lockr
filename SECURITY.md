# SECURITY

## Melden von Schwachstellen
Bitte sende private Hinweise an DEINE KONTAKTMAIL. Wir bestaetigen innerhalb von 7 Tagen.

## Unterstuetzte Versionen
Nur der Hauptbranch in diesem Stadium.

## Threat Model kurz
- Angreifer hat keinen Zugriff auf Master Passwort
- Angreifer kann die Vault Datei lesen
- Ziel ist Vertraulichkeit der Daten bei Datei Leck
- Memory Angriffe werden durch Zeroization reduziert
- Kein Schutz gegen kompromittiertes Betriebssystem

## Krypto Parameter
- Argon2id
  - m 64 MiB
  - t 3
  - p 1
- AEAD
  - ChaCha20 Poly1305
  - Nonce 12 Byte zufaellig pro Datei Segment
