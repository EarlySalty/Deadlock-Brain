status: aktiv
Datum: 2026-10-03

# Prüfziel für Bereich A

## Daten und Abdeckung

Der Nenner für eine vollständige Übernahme stammt aus einem tatsächlich durchlaufenen, fortgesetzten Namespace-/Seiteninventar. siteinfo-Statistik und Artikelanzahl ersetzen dieses Inventar nicht. Erfolgreiche Teilabrufe rechtfertigen keine Aussage über Vollständigkeit.

Für jede übergebene Quelle müssen stabile Seitenkennung, belegte Revision, UTC-Beobachtung, Hash über den exakten Inhalt, Sprache, Herkunft und Lizenzbezug nachvollziehbar sein. Ein vorhandener alter Rohcache belegt seinen historischen Stand, keinen aktuellen Wiki-Stand. Eine unbekannte Revision bleibt im Inventar gesondert sichtbar.

## Seitentypen

Redirects, Templates, Kategorien, Module, Data und Update bleiben als eigene Seitentypen erhalten. Der Text einer Artikelrevision und dynamisch eingebundene Abhängigkeiten haben unterschiedliche Versionsgrenzen. Aus Templates oder Lua wird ohne belegte Expansion kein aktueller Spielwert gerechnet. Historische Patchaussagen bleiben historisch.

## Wiederholung und Zugriff

Gleiche Seite, Revision und Inhalt erzeugen keinen zweiten Datensatz. Gleiche Seite und Revision mit anderem Inhalt wird als Konflikt sichtbar und überschreibt keine Quelle. Checkpoints werden erst nach dauerhafter Sicherung der zugehörigen Rohdaten weitergesetzt. Eine Unterbrechung verliert keine bestätigte Seite.

HTTP 403 oder eine Zugriffsschutzseite beendet den betroffenen Netzlauf. Der Sammler wechselt dabei weder Host noch Identität noch Endpunkt zur Umgehung. Netzwerkzugriff bleibt standardmäßig ausgeschaltet. Der tatsächliche bisherige Live-Lauf ist nach der Cloudflare-Challenge beendet; Offline-Arbeit läuft weiter.

## Lokale Prüfung und Übergabe

Eine unabhängige native Prüfung bewertet Code, Vertrag, Datenhashs, Wiederaufnahme und die belegten Restlücken. Compiler- und Laufzeitnachweise werden für den tatsächlichen Modulstand erfasst. C ist Eigentümer der Modulregistrierung, der gemeinsamen Manifestdateien und des endgültigen Imports. A meldet gebaut, lokal geprüft, gemergt und live getrennt. Der unabhängige Gesamt-Gate erfolgt bei C auf dem integrierten SHA.
