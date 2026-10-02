# Gemeinsamer Serverguide

Dieser Stand ist ein lokaler Entwurf. Der neue Pfad wird erst nach gemeinsamer Abnahme mit Discordadapter, C9 und dem Providerumbau aktiviert. Weder Schema noch Dienstkonfiguration werden durch einen normalen Dienststart verändert. Pilot, Begrüßung und Kontaktserien bleiben ausgeschaltet.

Der fachliche Kern liegt in `brain-serve/src/guide.rs`. DM, Erwähnung, eng zugeordnete Folgefragen, Tour, Profilkontrollen und konkrete Feedbackweiterleitung verwenden dieselbe Persona und denselben bereits bestehenden Provider. Der Discordadapter führt keine Modellanfragen und keine Profilpflege aus.

## Interner HTTP-Vertrag

Die typisierten Verträge liegen in `brain-contracts/src/guide.rs`. Kennung ist `guide.v1`. Guild, Mitglied, Kanal, Thread und Nachrichten werden als Snowflake-Strings übertragen. `request_id` trägt zusätzlich den Ereignisnamespace. Bearer-Authentifizierung benötigt den bestehenden internen Token mit explizitem Scope `guide`. Eine Testzulassung verlangt aktivierte Guidekonfiguration und eine nicht leere Mitgliederallowlist.

`POST /v1/guide/turn` erhält `GuideTurn`. Der Adapter attestiert Oberfläche und Adressierung anhand des tatsächlichen Discordereignisses. Öffentliche Fortsetzungen sind an Mitglied, Kanal, Thread, letzte Nachrichten und Inaktivitätsfrist gebunden. Öffentlich werden keine privaten Profilfelder und kein DM-Verlauf geladen. Eine menschliche Hilfe unterdrückt unadressierte Fortsetzungen.

`GuideResult.privacy_epoch` ist die tatsächlich verwendete Profilrevision. Der Adapter prüft sie unter dem gemeinsamen globalen Mitgliedslock direkt vor der Zustellung und hält diesen Lock während des Discordversands. Während der vorherigen Brain-HTTP-Anfrage hält er ihn nicht. Kontrollen erhalten die neue Revision. Die globale Vergessenbestätigung wird erst nach erfolgreicher bestehender globaler Löschung versandt.

`POST /v1/guide/action-result` bestätigt die tatsächliche Feedbackzustellung einmalig. Eine erfolgreiche Zustellung braucht die Discordnachrichten-ID. Normale Antworten werden mit `delivery_id=reply:{conversation_id}` und `reply_message_id` zugeordnet. Fehler bestätigen keine Zustellung. Wiederholungen erzeugen keine zweite Bestätigung.

`POST /v1/guide/server-snapshot` nimmt ausschließlich einen begrenzten öffentlichen Serverstand an. Kanal- und Regelquellen kommen aus der positiven Betreiberallowlist. Moderationskanäle und Ticketinhalte sind ausgeschlossen. Ein Ticket-Einstiegskanal darf ausschließlich als öffentliche Kanalmetadaten vorkommen. Revisionswechsel, entzogene Rechte und gelöschte Regeln ersetzen den aktuellen kanonischen Kopf. Alte Discordsnapshotrevisionen werden entfernt. Vor einer fertigen Antwort werden Aktualität und aktueller Quellkopf erneut geprüft. Der Discordadapter aktualisiert über seinen bestehenden betreuten Timer und verliert bei Abruffehlern die Quellfreigabe.

## Speicherung und Rechte

Die neue additive Migration ist `scripts/migrations/2026-10-03-serverguide-v1.sql`. Der vorhandene Owner-Migrator bietet `guide-up`, `guide-check` und `guide-import`. `guide-import` benötigt eine gültige `guide_retention_seconds` in seiner normalen lokalen Config und bricht ohne sie vor privaten Inhaltsabfragen ab. Es gibt keine private Stagingkopie. Übernommen werden nur enge freiwillige Angaben und höchstens acht erforderliche Verlaufsteile mit ursprünglichem Datum und Herkunft. Erinnerung bleibt nach dem Import ausgeschaltet. Opt-outs und Löschmarkierungen haben Vorrang.

Der Dienst benötigt SELECT, INSERT, UPDATE und DELETE ausschließlich für die eigenen `brain.guide_*` Laufzeittabellen sowie SELECT auf `core.user_privacy`. Er benötigt ausdrücklich keine DDL-Rechte, keine Schreibrechte auf `core.user_privacy`, keine Legacyprofilrechte und keine allgemeinen Quellschreibrechte. Ein Administrator vergibt den vorhandenen Dienstrollen separat EXECUTE auf `brain.guide_set_server_record(text,bigint,jsonb)`. Diese eng validierte Funktion ist der einzige neue Schreibweg in die vorhandenen kanonischen Quelltabellen. Die Migration gibt PUBLIC keine Funktionsrechte. Bestehende Leserechte des Readerpools bleiben ansonsten unverändert.

Für die globale Löschung gilt: `guide_subjects` leeren, Erinnerung und Kontakt abschalten, globale Sperre und Tombstone erhalten, Epoch erhöhen; `guide_turn_claims`, `guide_conversations`, `guide_feedback_outbox` und `guide_feedback_drafts` entfernen. `guide_legacy_imports` enthält ausschließlich den Wiederimportmarker und bleibt erhalten. Öffentliche guildweite Quellsnapshots enthalten keine privaten Profile und werden nicht einer Person zugerechnet. Der neue Kern führt keine weiteren privaten Caches oder Indizes.

Ein konfigurierter Cleanup im bestehenden Brain-Prozess entfernt abgelaufene Gesprächszuordnungen, Feedbackentwürfe, technische Ereignisse, Profilfelder und Verlaufsteile unter denselben Privacylocks. Die technische Ereignisfrist dient nur der Deduplizierung und Auslieferung. Sie ist keine beschlossene private Profilfrist. Ohne `memory_retention_seconds` liest und verwendet der Guide keinen alten privaten Profilkontext und erlaubt keine Aktivierung oder neue dauerhafte Speicherung. Die abschließenden Feld- und Aufbewahrungsentscheidungen stehen noch aus.

## Provider und sichtbarer Hinweis

Private Egress ist separat schaltbar. Die ausdrückliche Freigabe gilt für eigene DM-Eingaben und nötigen eigenen Kontext beim vorhandenen Fireworks-Flash-Provider. Der neue private Aufruf prüft sowohl `private_dm` am Principal als auch tatsächlichen Anbieter und Flashmodell vor dem Transport. Eine spätere CLI- oder Abomodellanbindung erbt diese Freigabe nicht. Es gibt keinen zusätzlichen Umschreibedienst und keinen neuen Fallback.

Der DM-Hinweis erklärt Erinnerungsschalter, Vergessen und die Weitergabe der aktuellen Nachricht mit nötigem eigenen Kontext. Der Adapter zeigt ihn verständlich und entprellt Wiederholungen. Profilkontrollen sind deterministisch und benötigen keinen Anbieteraufruf. Sichtbarer Funktionsname ist konfigurierbar und beginnt mit `Serverguide`.

## Prüfstatus

Code, synthetische Vertragstests und Migration sind geschrieben. Direkte Rustformatierung und `git diff --check` liefen ohne Fehler. Cargo, Clippy, vollständige Suites, synthetische Postgrestests, Selbstreview und die gemeinsame unabhängige Abnahme stehen wegen des gesperrten Hostslots noch aus. Keine Migration, kein Versand, kein Pilot und keine Bereitstellung dieses Entwurfs sind erfolgt.
