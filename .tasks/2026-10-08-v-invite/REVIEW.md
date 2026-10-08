# V: Mängelliste der ersten Teilbau-Prüfung

## Tatsächlicher Source und regulärer Gate

Brain-Source 7c8ba75e3e48745b8113908e360e601065b6b9fd, eigener Featurebranch feat/brain-v-invite-20261008. V hat HEAD und tatsächlichen Remote-Branch unabhängig bestätigt. /tmp/brain-v-invite-f1-20261008/brain-gate.log enthält:

```text
[gpt-6.1-sol] ALLOW: No blocking defect established by the supplied diff.
```

Zwei nicht blockierende Hinweise bleiben ausdrücklich für die Restintegration: projection prüft die Zahl der Scopes, der Aufrufer muss Request-/Personenownership sicherstellen; die vier Modultests sind mangels Export/Dependencyanschluss noch nicht ausführbar. Kein Brain-Compiler-, Test- oder Livebeweis aus diesem ALLOW.

Bots-Source 08828c1aa83bdf186894419450f6a929f1244ad9, eigener Featurebranch feat/brain-v-invite-consumer-20261008. Nach BLOCK hat V git status und log -1 einzeln geprüft: Produktstand committed, eigener Aufgabenordner noch untracked. Nicht gepusht. /tmp/brain-v-invite-f1-20261008/bots-gate.log enthält:

```text
[gpt-6.1-sol] BLOCK: The new tool rejects valid MCP calls with omitted arguments.
```

## Bestätigter Fund und enger Fix

1. mcp.rs:191 indiziert params.arguments unmittelbar. Ausgelassene Argumente werden JSON null. self_invite.rs:75-76 verlangt ein leeres Objekt. Dadurch antwortet der gültige parameterlose MCP-Aufruf mit 403. V hat Source und Gatelog nachgelesen. Zwillingsprüfung am bestehenden internen Dispatcher zeigt bereits Normalisierung fehlender Argumente; neue öffentliche Statusroute muss denselben no-argument-Vertrag erfüllen.

Frischer Fixer V/F2 erhält ausschließlich dieses offene BLOCK-Delta: fehlende Argumente im eigenen Statuszweig auf {} normalisieren und echten Endpunktfall für Omission ergänzen. Explizite null-/Skalar-/Array-/Fremdparameter bleiben verboten. Personen-, Authentifizierungs-, Request-, Rechte- und Postgresbindung bleiben unverändert. Kein Quoten- oder Shared-Delta.

## Tatsächliche Prüfgrenze von F1

Laut nativer Rückgabe Formatprüfung der drei Produktdateien erfolgreich. Bots-Compiler und Tests erreichten innerhalb ihrer jeweils 600 Sekunden den Cargo-Slot nicht. Clippy ebenfalls wartend und vom Worker vor Rückgabe beendet. Keine bestandenen Compiler-/Clippy-/Testläufe und keine Testanzahlen ermittelt. Der eigene Wegwerf-Testcontainer wurde laut Worker beendet. V meldet daraus keine grüne Prüfung.

F1 legte entgegen Briefing keine technischen Rückgabedateien an. Die native Abschlussmeldung und die nachgelesenen Gatelogs sind erhalten; diese Akte hält die tatsächliche Rückgabe fest. F2 muss lesbare Rückgabe samt Source-SHA, Exitnachweisen, Testzahlen und Featurebackupzustand liefern.

Kein Mainmerge, Deployment, Neustart, Livebeweis oder Cleanup. Voller Invitepfad bleibt offen bis zur Restübergabe.

## F2: BLOCK am Source behoben, Compilerbeweis noch offen

Frischer nativer Fixer ae648de4a76af9b4a korrigierte den engen Omissionfall auf Bots-Source 830fbea54caecdec48759cca0fff31e7dffa7e0c. V hat den tatsächlichen HEAD, Source-Diff und /tmp/brain-v-invite-f2-20261008/bots-gate.log geprüft:

```text
[gpt-6.1-sol] ALLOW: Omitted arguments now default to `{}`; endpoint tests cover omission. No blocking regression found.
FIXED: rust/bin/dl-bot/src/mcp/self_invite.rs:75
```

Nur fehlender Schlüssel wird im bestehenden Statuszweig auf {} gesetzt. Explizites null, Skalar, Array, fremde Parameter und fehlende Zugangsbindung bleiben gesperrt. Direkter Endpunktfall prüft Omission einschließlich eigener Enum-/Zeitprojektion und fehlender Auth-/Personen-/Requestbindung. Kein Produktdelta an Quote, K-Consumer oder Shared-Dateien.

F2: Format/Diff Exit 0 laut Rückgabe. Regulärer Testlauf erhielt Slot 3 und kompilierte Abhängigkeiten; nach 900 Sekunden laut Worker Exit 124, noch kein abgeschlossener dl-bot-/Testbinary-Compilerbeweis und keine Testzahlen. Clippy erhielt Slot 2, wartete auf die Buildverzeichnissperre und endete laut Worker nach 120 Sekunden mit Exit 124. Logmarker bestätigen Compilebeginn und fehlenden Testabschluss. Deshalb noch kein Bots-Featurepush. Eigener F2-Scratchcontainer wurde laut Rückgabe beendet.

Die Primary setzt den tatsächlichen unveränderten Bots-Sourcecompiler-/MCP-Testlauf mit regulärem cargo-slot, eigenem vorhandenen Targetcache und eigener isolierter Wegwerf-Postgresinstanz fort. Vollständiger Log /tmp/brain-v-invite-primary-proof-20261008/bots-tests.log. Nach 600 Sekunden wurde dieser vom Harness als eigener Hintergrundtask b3ieldg18 übernommen, nicht beendet. Auch in der anschließenden Wartephase entstand kein Slotmarker und keine Prüfausgabe; der vollständige Log blieb 0 Bytes. Die Primary stoppte deshalb ausschließlich diese eigene Slotwarteschleife per TaskStop und beendete ihre identifizierte Wegwerf-Postgresinstanz. Kein Compiler- oder Testharness gestartet, kein Compilerexit oder Testergebnis erfunden. Kein Slot-/Lockbypass, keine fremden Prozesse berührt. Verbleibender tatsächlicher Prüfblocker ist die fehlende reguläre Compilerressource. Primäres `rustfmt +1.97.1 --edition 2021 --config skip_children=true --check` auf den drei Produktdateien danach Exit 0.

Auch F2 legte wegen einer gemeldeten übergeordneten Rückgaberegel keine Berichtdateien an. Die Primary bewahrt die native Rückgabe und Gatelogs in dieser Akte, ohne einen Dokumentabschluss des Workers zu behaupten.
