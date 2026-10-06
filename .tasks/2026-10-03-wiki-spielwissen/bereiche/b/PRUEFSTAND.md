status: aktiv
Datum: 2026-10-03

# Lokaler Prüfstand

Rust-Bau und Datenlauf werden getrennt geprüft. Der Modul-Driver arbeitet mit den unveränderten Originaldateien und schreibt JSONL sowie Inventar außerhalb Git. Er verwendet denselben Modulcode, der an C übergeben wird.

Der native Bau-Worker wartet blockierend auf die beiden vorgeschriebenen Hostsperren. Die Sperren sind durch Gegenproben als belegt bestätigt. Kein fremder Compiler oder Dienst wird gestoppt. Vor jedem eigenen rustc-Start folgt eine frische NonZombie-Probe. Rust 1.97.1 entspricht den vorhandenen, ausschließlich lesend verwendeten Dependency-Artefakten. Es gibt keinen Cargo-Releasebau.

Ein frischer nativer Rust-/Security-/Vertragsprüfer arbeitet unabhängig und zunächst lesend. Seine Freigabe ist kein Merge-Gate und kein Live-Nachweis. Bis Compiler, konkrete Testzahlen und tatsächliche Quellenläufe vorliegen, bleibt gebaut=nein.

Belegte Rohdatenprüfung: 237/237 GameTracking-Dateien und 431/431 Snapshotdateien stimmen mit den exakten Git-Blob-Hashes überein. Kein offizieller Steam-Spielbestand wurde erfolgreich geladen; beide offiziellen anonyme Downloadversuche endeten mit Exit 8, No subscription.

Ausgelassene Hilfsprüfung: Ein zusätzlicher nur lesender Node-Aufruf zur Formatstatistik wurde vom lokalen Git-Hook als nicht prüfbare indirekte Push-Ausführung abgewiesen. Der Aufruf enthielt keinen Git-Push. Er wurde nicht wiederholt oder durch eine Umgehung ersetzt. Die eigentliche Rust-Extraktion bleibt davon unabhängig.

Nächster Schritt: echten Compiler-/Testabschluss abwarten und beide Quellen mit dem Rust-Driver extrahieren.
