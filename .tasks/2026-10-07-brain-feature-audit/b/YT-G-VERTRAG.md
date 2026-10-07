# Nachtrag B: Anschlussgrenze des G-Entityvertrags

Stand: 07.10.2026. Eigene statische Recherche von B. Kein Code geändert, keine DB gelesen, kein Modell oder Dienst aufgerufen.

## G-Quelle und belegter Stand

G-Worktree `/home/nathanael/.worktrees/brain-g-v2-20261007`, im Nachtrag gemessener HEAD `ce21a4576444c2afc9f2412f090857a43f9a0e2e`. Brain-main wurde erneut per `git ls-remote` als `9711cb630aebacfe959ed4783595b071f479be36` bestätigt. Hauptbaum unverändert alt bei `2734c2da4e814ff79953e8e825275b0216a6af16`. Main-Verträge und G-Sollvertrag sind getrennte Belege.

`G/PLAN.md` wurde aktuell gelesen. Relevante Fundstellen:

- `:45-56`: aktueller lokaler E-Spiegel, vollständiger Run, einmaliger Anfrage-Pin, Herkunftsbeleg; `client_version` ist keine automatisch bestätigte Balancepatchzuordnung.
- `:62`: `EntityRef` als Entitätsart plus positive API-ID; ein Name oder Klassenalias ersetzt die ID nicht.
- `:63-68`: MeasuredValue und deterministische Rechnung aus belegten Eingaben; qualitative Freitextaussagen gehören nicht in diesen Zahlenvertrag.
- `:71`: vorhandene Fact-/Rule-/Build-/Card-Verträge bleiben bis zum nachgewiesenen Abbau kompatibel; ein Katalog-Build ist kein neuer Planer.
- `:89`: Beleg pro Fakt mit Version, Art, Sprache, Entity-ID und JSON-Pointer; Herkunftstyp darf nicht als Wiki/Spieldatei falsch etikettiert werden.
- `:151-162`: gemeinsame Werkzeugausgabe und vorhandenes `entity_profile`. Die Ansicht liefert Daten beim Entitätsabruf, ein Text-Steckbrief muss nicht vorher veröffentlicht sein. Kein neues konkurrierendes Entitätswerkzeug aus diesem Audit ableiten.
- `:189-195`: Rechte, Quellenabhängigkeiten, Pin, Budgets und Cacheprüfung gelten vor Modellweitergabe und Ausgabe.
- `:203-210`: laufende Contract-/Provider-/Kernel-/Integrationsarbeit hat feste Eigentümer; B verändert diese Dateien nicht.

Direkt im G-Commit gelesen:

| Vertrag | Beleg |
| --- | --- |
| `ToolEntityRef` mit `kind` und `id: u64` | `rust/crates/brain-contracts/src/tools.rs:90-93` |
| `EntityProfileRequest` mit Entitätsreferenz, Feldauswahl, optionalem Szenario/Analytics | gleiche Datei `:240-246` |
| `PinnedGameContext` mit Clientversion, Sprache, Mechanikrevision | gleiche Datei `:773-785` |
| `ToolEvidenceDependency` mit konkreter Toolanfrage, Spielkontext und Belegen | gleiche Datei `:789-793` |
| `ToolExecution` mit Ergebnis, vollständigen Abhängigkeiten und Usage | gleiche Datei `:796-800` |

Der Commit enthält keinen bereits vollständigen YT-Wissensabschnitt oder dafür nachgewiesenen Fachadapter. Die Ergebnisform bleibt generisch; der typisierte Wissensanschluss ist ein Vorschlag an G, keine behauptete Implementierung. Die öffentlich beschriebenen zusätzlichen Reasoner-Entitätstypen im Plan dürfen ebenfalls nicht allein aus dem Plan als fertiger produktiver Rechenkern ausgegeben werden.

## Vorhandener Main-Profilvertrag

Brain `9711cb63`, `rust/crates/brain-contracts/src/entity_profile.rs`:

- `:17-20`: `ProfileSourceKind` kennt ausschließlich `GameFile` und `Wiki`.
- `:24-35`: `PatchValidity` hat `Unknown` mit Grund oder `Known` mit Grenzen und Beleg.
- `:38-44`: `EntityIdentity` enthält String-Entitykey, Art, Namen/Aliasse und Identitätsbelege. Das ist noch kein Nachweis einer kanonischen aktuellen API-ID jedes Altdatensatzes.
- `:47-55`: Herkunft mit Originalrevision, `observed_at`, Quellspanne, Lizenz und Metadaten.
- `:58-68`: Fakt-ID, Subjekt, Prädikat, Wert, Einheit, Qualifizierer, Status, Gültigkeit und Herkunft.
- `:79-89`: Profil als strukturierte Ausgabe mit Facts, Context, Konflikten, Patchgeschichte und Lücken.

Brain `rust/crates/brain-storage/src/entity_profile.rs` auf demselben main:

- `:60-63`: Originaldokumentprojektion akzeptiert nur `wiki` oder `game_file` als Herkunft.
- `:126-128`: gewöhnliche Faktprojektion setzt unbekannte Patchgültigkeit, wenn die Quelle keine Grenzen belegt.
- `:150-166`: unbekannte Gültigkeit erfüllt einen angefragten Patch nicht.
- `:288-290`: bestehende Quellenpriorität bevorzugt Spieldaten für Zahlen und Wiki für Beschreibungen. Diese Priorität ist kein Wahrheits- oder Aktualitätsbeweis einer neuen YT-Aussage.
- `:311-324`: unbekannter Patch wird als Lücke markiert; ein `source_statement`-String kommt in `context`. Ein solcher Kontextstring ist dadurch nicht aktuell verifiziert.
- `:554-558`: bei expliziter Patchanfrage werden Fakten ohne passende Gültigkeit ausgeschlossen.

YT-Aussagen können deshalb nicht einfach als Wiki-Fakten in diesen Leser geschoben werden. Bestehende Herkunfts-, Status-, Konflikt- und Rechtekonzepte sind wiederverwendbar, der tatsächliche neue Aussage-/API-ID-Anschluss fehlt.

## Passender Vorschlag, ausdrücklich kein Bau

G soll einen getrennten qualifizierten Wissensabschnitt im bestehenden Entitätsabruf definieren. Grundlage bleiben E/G-Entitätsdaten und aktuelle deterministische Werte. Kandidatenaussagen aus dem bestehenden Claimsbestand werden an aufgelöste kanonische Entitätsreferenzen gebunden; es entsteht keine zweite Profiltextablage oder zusätzliche Profilpublikation.

Je Aussage werden Claim-ID, Typ, beteiligte EntityRefs/Rollen, Bedingungen, Behauptung, Originalquelle/Span, Aussagezeit, letzte fachliche Prüfung, Prüfbeleg, Versions-/Patchgültigkeit und Unsicherheits-/Widerspruchszustand mitgeführt. Herkunfts- und Quellenfreigaben bleiben Teil derselben Toolabhängigkeiten und Cache-Neuprüfung. Zahlen aus alten Videos bleiben historische Quellbestandteile und überschreiben niemals E/G-Werte. Eine qualitative Aussage wird nicht durch das Entfernen ihrer Zahlanteile automatisch gültig.

Die bestehende Zuordnung und Prüfung einzelner YT-Claims wird von den separaten Rechercheworkern bewertet. Dieser Vertragsbericht bestätigt keine aktuelle Gültigkeit einer realen YT-Aussage.
