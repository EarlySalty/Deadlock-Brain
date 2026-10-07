# H: unabhängige native Layoutprüfung

Stand: 7. Oktober 2026. Eine gebündelte Erstpass-Runde, keine Produktänderung und keine Nachprüfung nach Fixes.

## Urteil

**Die synthetische Vorschau ist grundsätzlich lesbar und strukturiert. Die mobile Grafik und die Freigabeformulierung brauchen jedoch Korrekturen vor einer visuellen Abnahme.** Kein Gate-Urteil, keine Veröffentlichungsfreigabe und kein Livebeweis.

Der echte H-Rust-Renderer wurde ausschließlich unter `http://127.0.0.1:40044/` mit erfundenen Testhelden und Vorschauwerten betrachtet. Keine historische G6759-Probe, keine echten G-Spielwerte und keine Nutzer- oder Community-Daten wurden verwendet.

## Verfahren und Abgrenzung

- `impeccable` und `no-em-dashes` vor der UI-Prüfung geladen. Impeccable-Kontext einmal gelesen, Audit-Playbook angewendet. Fehlender Produkt-/Designkontext wurde nicht angelegt oder verändert.
- Vorhandenes `/usr/local/bin/google-chrome`, Headless-Modus, eigenes frisch erzeugtes Profil. CDP ausschließlich über Prozess-Pipes, kein eigener Netzwerklistener und keine neue Browser-/Playwright-Installation.
- Hintergrundnetzwerk, Sync, Erweiterungen und Komponentenupdates deaktiviert. Namensauflösung externer Hosts blockiert; Seitenanfragen zusätzlich auf die lokale Vorschau beschränkt.
- Desktop-Emulation 1440 × 1000 CSS-Pixel, Mobil-Emulation 390 × 844 CSS-Pixel, jeweils DPR 1. `innerWidth/innerHeight` direkt im Browser geprüft. Mobile `visualViewport.width`, Dokumentbreite und Dokument-Scrollbreite jeweils exakt 390. Desktop-Dokumentbreite 1425 wegen der normalen vertikalen Scrollbar, Fensterbreite 1440.
- Vier PNGs erzeugt und alle vier einzeln mit dem nativen `Read`-Bildzugriff angesehen. Die beiden zusätzlichen Mobilbilder zeigen denselben Erstpass mit rechts verschobener Grafik beziehungsweise nach vertikalem Scrollen zur Tabelle.
- Browserprozess geschlossen und nur dessen eigenes frisches Profil entfernt. Vorschau nicht geschlossen; `/close` nicht aufgerufen.
- Keine Produkt-, Test-, Manifest-, API-, G- oder K-Dateien verändert. Keine Delegation, Modellwechsel, Git-, Gate-, Merge- oder Deploy-Schritte.

Das beweist das native Chrome-Layout des H-Renderers mit synthetischem Inhalt. Es beweist weder echte mobile Hardware-/Touchbedienung noch Screenreader-Verhalten, vollständige Tastaturbedienung, echte G-Datenkorrektheit, K-Integration, Veröffentlichung oder den produktiven Laufzeitpfad. Der echte G-/K-/Livebeweis bleibt offen und benötigt unabhängig freigegebene echte Daten und den vorgesehenen Integrations-/Produktionspfad.

## Befunde und konkrete Fixes

### P1: Vorschau behauptet eine öffentliche Freigabe

**Ort:** SVG-Fußbereich und Liste „Stand und Bedingungen“; sichtbar in Desktop- und Mobilnachweisen.

Die isolierte Statuszeile „Öffentlich freigegeben“ steht neben ausdrücklichen Aussagen, dass keine echten Quellen freigegeben sind. Der Warnhinweis am Seitenanfang ist klar, schützt aber einen isolierten Grafikexport oder einen nach unten gescrollten Ausschnitt nicht zuverlässig vor diesem widersprüchlichen Eindruck.

**Nötiger Fix:** In der synthetischen Vorschau diesen Status unmittelbar als simuliert kennzeichnen, etwa „Freigabe nur für diese Vorschau simuliert“. Eine echte Freigabeprüfung dabei nicht abschwächen. Auch der allein exportierte Grafikbereich muss den synthetischen Charakter eindeutig zeigen.

### P2: Kleine mobile SVG-Schrift

**Ort:** mobile Vergleichsgrafik, Achsen, innere Legende und Fußbereich.

Das SVG hat einen `viewBox` von 800 × 750 und wird mobil mit 640 × 600 CSS-Pixeln gerendert. SVG-Schrift von 14 Einheiten entspricht dadurch 11,2 sichtbaren CSS-Pixeln, Fußtext von 13 Einheiten nur 10,4. In den nativen Mobilbildern ist die Grafik deutlich schlechter lesbar als der umgebende 16-Pixel-Text. Die gut lesbare Tabelle bietet einen Ausweg, behebt aber die Diagrammlesbarkeit nicht.

**Nötiger Fix:** Mobile Grafikschrift in sichtbaren CSS-Pixeln vergrößern oder das Diagrammlayout entsprechend anpassen. Mindestens 14 sichtbare Pixel für wichtige Labels anstreben. Metadaten müssen nicht zusätzlich in winziger Schrift im horizontal verschiebbaren SVG wiederholt werden, wenn sie bereits lesbar darunter stehen. Keine formale WCAG-Mindestschriftgrößenverletzung behauptet.

### P2: Seitliches Verschieben ist für sehende Nutzer nicht erklärt

**Ort:** mobile `.chart`-Region.

Die Seite selbst läuft nicht horizontal über. Die Grafik ist korrekt in einer eigenen 358 Pixel breiten Scrollregion mit 640 Pixel Inhalt eingeschlossen. Rechts fehlen zunächst der letzte Boonstand, Endwerte und der zweite innere Legendeneintrag. Die Region hat `role="region"`, `tabindex="0"` und einen passenden zugänglichen Namen. Ein sichtbarer Hinweis fehlt aber; im nativen Mobilbild ist keine horizontale Scrollbar zu sehen.

Programmgesteuertes Verschieben von `scrollLeft=0` auf 282 zeigt, dass der rechte Rand erreichbar ist. Das ist ein geometrischer Scrollnachweis, kein Nachweis einer realen Wischgeste.

**Nötiger Fix:** Direkt vor der mobilen Grafik einen kurzen sichtbaren Hinweis setzen, etwa „Grafik seitlich verschieben“. Die gut lesbare äußere Legende erhalten. Alternativ die Grafik ohne Abschneiden an den schmalen Viewport anpassen, dabei die Schrift nicht weiter verkleinern.

### P2: Verzerrte Endlabels und zerrissene Wörter im SVG

**Ort:** rechte Serien-Endlabels und SVG-Fußtext.

Die Heldennamen am Linienende wirken im Desktop- und rechten Mobilbild auffällig breit gezogen. Die ausgegebenen Elemente verwenden `textLength="134"` zusammen mit `lengthAdjust="spacingAndGlyphs"`. Der Fußtext trennt außerdem „G-Probe“ in „G-Prob“ und eine eigene Zeile „e“, sowie „Veröffentlichungsfreigabe“ mitten im Wort. Die normale HTML-Liste darunter bricht deutlich besser um.

**Nötiger Fix:** Die Namen mit natürlicher Glyphenbreite setzen und ihre Position beziehungsweise den Platz im Layout anpassen, statt die Schrift künstlich zu dehnen. Fußtext an Wortgrenzen umbrechen oder die bereits vorhandene lesbare HTML-Metadatenliste als alleinigen Detailbereich nutzen. Den eigenständigen SVG-Export dabei weiterhin verständlich und eindeutig synthetisch halten.

## Positive Befunde

- Klare Überschrift, Einheit „Schaden/s“ und gemeinsame Bedingung „bei gleichen Boonständen“.
- Achsen beginnen bei null; Werte verwenden deutsche Dezimalkommas.
- Serien sind nicht nur durch Farbe getrennt: durchgezogene Linie und Kreismarker gegenüber gestrichelter Linie und quadratischen Markern. Die äußere Legende bleibt mobil vollständig sichtbar und bricht sauber in zwei Zeilen um.
- Haupttext und Sekundärtext sind gut kontrastiert. Gemessen gegen `#111110`: `#f3efe5` 16,45:1, `#c9c3b4` 10,75:1. Die Linienfarben erreichen 5,90:1 und 3,18:1 gegen den Hintergrund. Der dunklere zweite Verlauf ist weniger auffällig, aber kein belegter Kontrastfehler unter 3:1.
- Tabelle im nativen Mobilnachweis vollständig lesbar, ohne seitliches Überlaufen. Drei Boonstände und beide Helden passen in 358 Pixel; Zahlen sind rechtsbündig und leicht vergleichbar.
- Die HTML-Bedingungen sind mobil ordentlich umbrochen und mit echten Umlauten geschrieben. Dokument `lang="de"`; keine Em-Dashes im untersuchten Seiteninhalt.
- Kein globaler horizontaler Overflow. Die Grafikscrollregion grenzt den großen Inhalt korrekt ein. Die Tabelle besitzt ebenfalls eine benannte, fokussierbare Region und braucht bei dieser Probe kein seitliches Scrollen.

## PNG-Nachweise

Alle Dateien sind ausschließlich **synthetische Layoutnachweise**, niemals Livebeweise.

1. Desktop, 1440 × 1000: `/home/nathanael/.worktrees/brain-h-grafik-20261007/.tasks/2026-10-07-brain-grafik-ki/H/sicht/desktop-1440x1000-synthetisch.png`
2. Mobil, 390 × 844, Ausgangsansicht: `/home/nathanael/.worktrees/brain-h-grafik-20261007/.tasks/2026-10-07-brain-grafik-ki/H/sicht/mobil-390x844-synthetisch.png`
3. Mobil, 390 × 844, Grafik rechts: `/home/nathanael/.worktrees/brain-h-grafik-20261007/.tasks/2026-10-07-brain-grafik-ki/H/sicht/mobil-390x844-grafik-rechts-synthetisch.png`
4. Mobil, 390 × 844, Tabelle und Bedingungen: `/home/nathanael/.worktrees/brain-h-grafik-20261007/.tasks/2026-10-07-brain-grafik-ki/H/sicht/mobil-390x844-tabelle-synthetisch.png`

PNG-Abmessungen zusätzlich aus den Bildheadern geprüft. Desktop-Tabelle unterhalb des ersten Viewports nur geometrisch geprüft; die native Tabellen-Sichtprüfung erfolgte im mobilen Zusatzbild. Keine zweite Abnahmerunde und keine vollständige Accessibility-/Performance-/Theming-Zertifizierung.
