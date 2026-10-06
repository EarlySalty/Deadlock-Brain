status: erledigt
Datum: 2026-10-03
Prüfstand: 06:34:10 UTC

# B2: tatsächlich ausgeführte Prüfkette

Nativer Prüfworker `a62188d3c4ba27ce0`, GPT 6.1 Sol high. B2 hält dessen Rückbericht fest. Bestehender `check-round2.sh` erstmals vollständig ausgeführt, Exit 0. 06:16:21 bis 06:34:10 UTC, Bash-time 17m48,689s einschließlich Lockwartezeit. Der frühere GNU-time-Aufruf endete vor Wrapperstart mit Exit 127 und war kein Testlauf.

Vier Rust-Compilerläufe erfolgreich. 27 Extraktortests, fünf Validatortests und ein zusätzlicher begrenzter VPK-Wiederholungstest erfolgreich. 32 unterschiedliche Tests, 33 erfolgreiche Testausführungen; null Fehlschläge, null ignoriert. Flags `--include-ignored --test-threads=2`. Der 51-Byte-VPK-Test wurde zusätzlich unter 128 MiB Adressraumlimit ausgeführt.

TESTNACHWEIS[TW-1]: 33 passed, 0 ignored | Baseline: unbekannt rot

„Unbekannt“ bezeichnet den nicht gemessenen früheren Teststand, keinen belegten Altfehler.

| Quelle | Dateien im Inventar | Dokumente | Fakten | JSONL-Bytes | Größte Zeile ohne LF |
| --- | ---: | ---: | ---: | ---: | ---: |
| GameTracking 4c6431ccdb816d2911bbbaa4335169cc34140386, Version 6745 | 237 | 237 | 331.287 | 265.376.583 | 117.157.259 |
| deadlock-data 0d46cdecfccf77adec16aac01af6d30173e0ebb8, Version 6731 | 431 | 423 | 345.076 | 241.035.873 | 11.107.243 |

Beide vollständigen Extraktions- und Validatorläufe erfolgreich. Originaltext-/Inhaltshashes und 55.504 beziehungsweise 13.367 Zahlenlexeme geprüft. Acht deadlock-data-Dateien nur inventarisiert: sechs PNGs, LICENSE und README.md. Git-Revisionen und Clientversionen sind keine Steam-Build-/Depot-/Manifestkennungen.

Elf Handoff-Hashes unverändert. Neun Quellhashes vor und nach dem Lauf identisch, sechs Artefakthashes nachgeprüft. Beide HOSTPROBE-Locks gehalten, unmittelbar vor jedem Compiler frische NonZombie-Probe, kein fremder Prozess gestoppt. Freigabe beider eigenen Locks mit `LOCK_RELEASE both released` belegt; eigene Task `bfotw4iyn` und PIDs 3213419/3213424/3213425 beendet. Globale Sperren danach wieder belegt, keine globale Sperrfreiheit behauptet.

Nachweisroot außerhalb Git:
`/home/nathanael/.local/share/deadlock-brain/wiki-spielwissen/2026-10-03/b/round2-proof/`.
Darin `check-b2-run.log`, beide JSONL-/Inventar-/Validatorausgaben, `source-before.sha256`, `source-after.sha256`, `artifacts.sha256`.

Dieser Stand wurde nicht geändert, committed, integriert oder deployt. RSS-Messung, fmt/Clippy und finale unabhängige Abnahme noch offen. Die bestehende Git-Exportprüfung ist keine Abnahme künftiger Steam-/VPK-Rohdaten. C2s aktuell gelesenes Importpartitionlimit 64 MiB liegt unter der gemessenen größten GameTracking-Zeile; diese Importgrenze muss C2 vor dem echten Import verlustfrei behandeln.
