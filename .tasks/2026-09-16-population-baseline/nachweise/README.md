# Nachweise Paket M

Die Mess-JSONs (8,8 MB) liegen nicht im Repo, sondern unter
`/home/nathanael/.local/share/deadlock-brain/nachweise/2026-09-16-population-baseline/` mit Prüfsummen in `SHA256SUMS`:

```
8efc777779bf9fc9d25bcfb5abc179ca45f5e32d2d1c5884a6c34213cb3e85f6  M-DIAG-fc71b5d.json
ff0926664529595d84646d60480011d919fb19e456b147ce72fae6c3c7192804  M-SECHS-nopop.json
c5292d54fc6b143d4eaf789a5867c5f16eedf81f58e245766900743339642c7d  M-SECHS-pop.json
94683e03ef47e8faa952a76e119584a390a430cd47578bec6989111178123095  M-WARDEN-fc71b5d.json
```

Erzeugt mit `examples/build_evaluation.rs` gegen `FROZEN-V2.json` (Welle 2) und die Population aus `population_dev`; Bedeutung je Datei in `M-MESSUNG.md`.
