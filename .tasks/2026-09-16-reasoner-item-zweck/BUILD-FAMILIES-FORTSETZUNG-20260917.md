# Buildfamilien-Fortsetzung: geprüfter Featurestand, keine Releasefreigabe

Stand: 17.09.2026. Bestehender Branch `fix/reasoner-mechanics-completion`, Worktree `brain-purpose-finish`, Ausgangscommit `1147aadb146941889de09152f68c15d4de092a96`. Die vorgefundenen Familien-, Composer-, Planner- und Retrieval-Änderungen wurden fortgeführt, nicht durch eine zweite Architektur ersetzt. Keine neuen bezahlten Modell-Worker, keine DB-Schreibvorgänge, kein produktiver Restart, kein Main-Merge und kein Steam-Publish in dieser Fortsetzung.

## Fortgeführte Integration

Der bestehende Planner bekommt getrennte, datengetrieben erkannte Familienkontexte. Spieler-/Autorenquellen, populationsbezogene Itemhäufigkeiten, Kaufpositionen, Imbues, Skillorders und nachweislich aktuelle Paarstatistiken werden familienbezogen verarbeitet. Historische Matches zählen nicht als aktuelle Patch-Winrate. Namen dienen nicht als numerische Hero-/Item-Sonderregel. Die Ausgabe enthält dominante Familie und getrennte Varianten; Retrieval mischt deren Käufe, Bindungen und Skillorders nicht. Der alte Einzelbuild-Publisher verwirft zusätzliche Varianten nicht still, sondern verweigert eine solche Veröffentlichung vor dem Queue-Schreibvorgang.

Historische Kostenbandzahlen erzwingen in der empirischen Planung keine Auffüllkäufe mehr. Unbekannte Mechaniken bleiben auch für früh gekaufte und später verkaufte Items sichtbar und begrenzen die Sicherheit. KI-Kritik verändert keine numerische Einkaufsliste mehr. Dies ist noch keine vollständige fachliche Abnahme dieser gesamten Integration; die konkreten offenen Punkte stehen unten.

## In dieser Fortsetzung belegte und korrigierte Fehler

1. **Familien-ID-Kollision:** Zwei mechanisch getrennte Gruppen mit denselben sechs frühen Itemwurzeln erhielten beide `family-a3e956c439195de2`. Die bisherige verkürzte ID hätte Scores überschreiben und Holdout-Zuordnungen vermischen können. Die ID berücksichtigt jetzt die vollständige, nach Featuredomänen getrennte mechanische Signatur. Nur für den Fingerabdruck werden Anteile auf ein Millionstel quantisiert, nicht für Clustering oder Scores. Ein zusätzlicher Schutz in `plan_build` bricht bei einer doppelten ID ab, statt eine Variante zu überschreiben. Die neue ID-Version ist kohortenbezogen, kein über Datenänderungen unveränderlicher Playstyle-Schlüssel.
2. **Importreihenfolge änderte Patchbelege:** Widersprüchliche Einträge derselben Match-ID ergaben nach Umkehr der Eingabereihenfolge null beziehungsweise zwölf Nach-Patch-Belege. Eine gemeinsame Kanonisierung entfernt identische Duplikate und weist widersprüchliche Identitäten aus. Ohne belegte Quellversion wird kein Payload bevorzugt; die betroffene Identität wird ausgeschlossen. Warnungen identischer Belege werden vereinigt, nicht verloren.
3. **Importreihenfolge änderte Holdout-Population:** Derselbe Schlüssel mit verschiedenen Einkaufslisten gab je nach Reihenfolge eine andere Pickrate. Holdout-Priors und Zuordnungen verwenden jetzt dieselbe Kanonisierung; ausgeschlossene Identitäten bleiben mit Grund sichtbar.
4. **Doppelte Matches kamen nach dem Clustering zurück:** Zwölf deduplizierte Matches wurden beim Erzeugen der Familienstatistik wieder zu 120 Beobachtungszeilen. Familienkonditionierung und Trainingszentroiden bleiben jetzt dedupliziert und numerisch stabil sortiert.
5. **Teilnehmer-Leakage zwischen Quelltypen:** Die separate Eingangsprüfung in `ASTRA-EINGANGSPRUEFUNG-20260917.md` fand dieselben Accounts als Spieler und Autor auf verschiedenen Holdout-Seiten. Eine eigene Gegenprobe scheiterte bereits bei Account 0. Die bestehende Spielerpartition bleibt erhalten; Autoren desselben Accounts folgen derselben Partition. Der Quelltyp erzeugt keine zweite Person. Nachher: null Überschneidungen in allen sechs echten Heldeneingaben und null Abweichungen bei 1.000 synthetischen Accounts.

Die ersten vier neuen Tests wurden zusammen tatsächlich rot ausgeführt (13 bestehende Regressionstests grün, vier neue rot). Der fünfte Test wurde separat rot ausgeführt. Danach bestanden alle 31 Familientests. Die Korrekturen verändern weder Staple-Schwelle noch Stichprobengrenzen noch die Ähnlichkeitsschwellen. Die Eingangsprüfung ist keine externe fachliche Gesamtfreigabe.

## Tatsächlich ausgeführte abschließende Prüfungen

Alle Cargo-Aufrufe verwendeten `/home/nathanael/.cargo/bin/cargo` aus `rust/`; das bloße `cargo` verweist in diesem Toolkontext auf eine zu alte Version für die Lockdatei.

- `test -q -p dbrain-reasoner -p dbrain-retrieval --lib --examples`: Exit 0. Reasoner 249 bestanden / 16 ignoriert; Retrieval 22 bestanden / 12 ignoriert. Zusammen zusätzlich 29 Example-Testausführungen bestanden. Darin enthalten sind drei Tests des parallel angelegten Eingangsprüfungs-Examples, dessen Datei nicht von dieser Session erstellt wurde. Kein ignorierter Datenbanktest wird als bestanden gezählt.
- `clippy -q -p dbrain-reasoner -p dbrain-retrieval --lib --examples -- -D warnings`: Exit 0.
- `check -q --workspace --all-targets`: Exit 0. Kompilationsbeleg, kein ausgeführter Workspace-/Datenbanktestlauf.
- Fokus-Formatter über `target/families-format/Cargo.toml` und `skip_children=true`: nach Formatierung `--check` Exit 0; nur die explizit aufgeführten Integrationsdateien, keine repoweite Formatierung.
- `git diff --check`: Exit 0.
- Release-Build des Examples `family_evaluation`: Exit 0.
- Drei vollständige Release-Planläufe über dieselbe eingefrorene Eingabe, jeweils Exit 0. Alle 15 Varianten enthalten. Paarweise Dateivergleiche A/B und A/C mittels `git diff --no-index --exit-code`: beide Exit 0, die vollständigen JSON-Dateien sind bytegleich.
- Ein zusätzlicher früherer Debug-Gesamtlauf meldete auf Tool-Ebene Timeout. Seine anschließend vorhandene Datei wurde vollständig mit `report` gelesen, wird aber nicht als erfolgreicher Exit-0-Lauf gezählt. Der Dreifachnachweis stammt ausschließlich aus den drei späteren erfolgreichen Release-Läufen.
- Neuer vollständiger Holdout-Lauf nach Teilnehmerkorrektur: Exit 0 für die Berechnung, aber fachliche Gates überwiegend rot. Erfolg des CLI-Aufrufs ist keine Releasefreigabe.

## Neue Messung auf dem eingefrorenen Datenstand

`families-20260917-replay-release-a.json`, dominante Variante je Held; Viscous mit beiden ausreichend unterstützten Varianten. Die Familienzahl ist ein Clustering-Ergebnis, keine Anzahl fachlich freigegebener Veröffentlichungen.

| Held / Variante | Familien gesamt | Matches dieser Familie | Core / Familien-Staples | Familien-Kendall | Familien-Jaccard@12 |
|---|---:|---:|---:|---:|---:|
| Warden / Gun-Spirit | 1 | 3474 | 10 / 10 | 0,866667 | 0,833333 |
| Viscous / Melee | 2 | 785 | 10 / 10 | 0,340997 | 0,833333 |
| Viscous / Spirit | 2 | 518 | 9 / 9 | 0,253546 | 0,750000 |
| Infernus / dominante Familie | 3 | 3037 | 13 / 13 | 0,493548 | 0,846154 |
| Abrams / dominante Familie | 4 | 1369 | 7 / 7 | 0,000000 | 0,583333 |
| Lady Geist / dominante Familie | 3 | 1322 | 13 / 13 | 0,169967 | 0,846154 |
| Vindicta / dominante Familie | 2 | 2394 | 12 / 12 | 0,229014 | 1,000000 |

Warden enthält die zehn bisherigen globalen Staples. QSR und Mercurial Magnum bleiben an Fähigkeit 2656490109 gebunden. Der Core wird nicht auf 16 oder 18 Einträge aufgefüllt. Viscous hat getrennte Melee- und Spirit-Käufe und unterschiedliche beobachtete Imbue-Ziele 1020817390 beziehungsweise 3247040238. Beim Melee-Präfix beträgt der Skillorder-Support nur 0,1898; eine stabile Familien-Skillorder ist damit nicht belegt. Eine Gun-/Support-Variante wird nicht künstlich ergänzt. Das Fehlen globaler Spirit-Staples im Viscous-Melee-Plan bleibt in der globalen Vergleichsmetrik sichtbar und ist kein Grund, die Familien wieder zu vermischen.

### Neuer Holdout nach Leakage-Korrektur

| Variante | Zugeordnete Holdout-Matches | Staple-Gate | Kendall | Jaccard@12 |
|---|---:|---|---:|---:|
| Warden | 522 | rot | 0,844072 | 0,916667 |
| Viscous Melee | 89 | grün, Stichprobe zu klein | 0,322031 | 0,833333 |
| Viscous Spirit | 61 | rot, Stichprobe zu klein | 0,228665 | 0,615385 |
| Infernus dominant | 367 | rot | 0,477429 | 0,846154 |
| Abrams dominant | 168 | rot | -0,066667 | 0,500000 |
| Lady Geist dominant | 231 | grün | 0,106752 | 0,846154 |
| Vindicta | 194 | rot | 0,629253 | 0,833333 |

Weitere dünne Familien und alle ungünstigen Einzelmetriken stehen unverändert im vollständigen Holdout-Artefakt. Die Zahl der im Training unterstützten Familien unterscheidet sich teilweise vom Vollkohortenlauf; auch das ist keine belegte Robustheit. Die Katalogprüfung zeigt 251 Itemmodelle je Held, keine unbekannten oder nicht kaufbaren beobachteten Item-IDs. Die fehlenden Holdout-Staples dürfen daher nicht als vermeintlich entfernte Items aus der Auswertung gestrichen werden.

## Konkrete Release-Blocker und nächster fachlicher Arbeitsumfang

**Nach-Patch-Population und Live-Prüfung fehlen.** Der Freeze verarbeitet den Patchbeginn 16.09.2026, 22:16:43 Europe/Berlin; die letzten Spieler-Matches reichen nur bis 16.09.2026, 04:16:28. Alle sechs Helden haben null belegte Matches nach diesem Patch. Eine aktuelle Read-only-Prüfung scheiterte vor dem DB-Zugriff mit `Infisical benötigt einen regulären Credential-Dateideskriptor.` Keine Umgehung, kein Auslesen von Secrets, keine Änderung einer Allowlist. Reeller nächster Schritt ist ein frischer Snapshot über den vorgesehenen autorisierten Credential-/Sync-Startweg und anschließend derselbe Holdout-/Replay-Lauf; nicht das Umetikettieren historischer Matches als aktuelle Patchdaten.

**Core-Identität ist noch zu eng definiert.** Bei vorhandener Population lässt `core_candidates` derzeit nur lokale Staples zu. Das verhindert Füllmaterial, ist aber noch kein vollständiger Nachweis mechanischer Build-Identität. Familienprägende, seltener als 70 Prozent gekaufte Synergiekäufe brauchen eine explizite mechanische/cooccurrence-basierte Zulassung samt robustem Szenarienvergleich. Umgekehrt macht eine hohe Pickrate einen kontextabhängigen Kauf nicht automatisch universell. Keine Schwellenänderung und kein Hinzufügen einer gewünschten Itemliste als Ersatz dafür.

**Holdout und Kaufkurve sind nicht abgenommen.** Warden/Infernus/Abrams/Vindicta haben rote Holdout-Staple-Gates. Viscous hat zu wenige sicher zugeordnete Holdout-Matches. Die dominante Abrams-Reihenfolge liegt im Holdout bei negativem Kendall; Geists Reihenfolge ist ebenfalls schwach. Nächste Untersuchung: Anschlusskäufe/Upgradezeitpunkte und tatsächlichen marginalen Szenariennutzen im bestehenden Planner gegen die beobachtete Kaufkurve erklären, nicht die Einkaufsreihenfolge aus einer Referenz kopieren. Eine Abrams-Variante hat zudem keine belegte ausgegebene Skillorder; nicht durch eine erfundene Folge kaschieren.

**Mechanik-/Variantenfreigabe fehlt.** Sämtliche 15 Vollkohortenvarianten weisen weiterhin Low Confidence aus. Unquantifizierte Effekte in ihren konkreten Kaufzuständen sowie Familienkohärenz, dünne/nahe Cluster und Skillorder-Fallbacks sind noch fachlich abzuarbeiten. Der vorhandene Publisher blockiert ungeprüfte Familien und das stille Weglassen von Varianten; ein vollständig abgenommener Auswahl-/Veröffentlichungsweg für mehrere Varianten ist damit nicht bereits fertig.

Damit sind Main-Merge, Produktionsdeploy, Steam-Publish und anschließende Branch-/Worktree-Löschung weiterhin gesperrt. Der getestete Featurestand darf separat versioniert werden, nicht als fertiger Release ausgegeben werden.

## Artefakte und Zuständigkeiten

Alle folgenden JSON-Dateien liegen unter `rust/target/`, werden nicht mit Roh-Spielerdaten eingecheckt und wurden nicht überschrieben:

- `families-20260917-input.json` (bestehender Read-only-Freeze; Eingangsprüfung nennt SHA-256 `f90ed6be72c66ca37bafbd7f04b4e77ed4d26989c4869dcb1265671541217907`).
- `families-20260917-input-review-after-account-fix.json` (null Teilnehmer-Leaks).
- `families-20260917-replay-release-a.json`, `-b.json`, `-c.json` (drei erfolgreiche, bytegleiche Gesamtläufe).
- `families-20260917-holdout-account-fixed.json` (neue Gates und vollständige Einzelmetriken).
- `families-20260917-continuation-plans-a.json` (gelesener Debuglauf nach Tool-Timeout, nicht Teil des Dreifachnachweises).

Die parallel angelegten Dateien `examples/family_input_review.rs` und `ASTRA-EINGANGSPRUEFUNG-20260917.md` wurden gelesen und ihre Befunde nachgeprüft. Ihr eigener Review-Scope wurde nicht überschrieben. Diese Fortsetzung übernimmt daraus keine unabhängige Gesamtfreigabe.
