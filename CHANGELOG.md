# Changelog

## #17 — Brain-Wissen kann in die zentrale Postgres-Timeline exportiert werden

Das Brain lag bisher weiter in einer eigenen SQLite-Datenbank. Für eine echte Historie aus alten Patchständen, Forum-Funden, Reworks und aktuellen Gewinnerdaten ist das nur als Übergang sinnvoll, aber nicht als zentrale Wissensquelle.

Der neue PG-Export schreibt die lokalen Quellen, Snapshots, Patch-Events und Forum-Claims idempotent in das zentrale `brain`-Schema der Postgres-Datenbank. Zusätzlich entstehen vereinheitlichte Wissens-Events für die Timeline und ein aktueller Snapshot-Stand, bei dem vertrauenswürdige aktuelle Quellen nach Priorität gewinnen, während Forum-Claims in ihrer historischen Quarantäne bleiben.

Damit kann die bestehende SQLite-Basis sicher in die zentrale Datenbank überführt werden, ohne alte Daten zu verlieren oder alte Forum-/Patch-Aussagen als heutige Wahrheit zu behandeln.

## #16 — Forum-Daten landen zuerst als historische Claims mit Link und Schutzstatus

Forum-Beiträge enthalten wertvolle Bug-, Exploit-, Meta- und Entwicklerhinweise, können aber alt, gefixt oder durch Reworks überholt sein. Wenn solche Aussagen direkt wie aktuelle Fakten behandelt würden, könnte das Brain falsche Builds, alte Werte oder nicht mehr existierende Mechaniken weiterreichen.

Aus importierten Forum-Posts entsteht deshalb jetzt eine getrennte historische Claim-Schicht. Jeder Claim bekommt den Link zum konkreten Thread-/Post-Anker, Datum, Autor-/Rollenhinweis, Vertrauensart, Gültigkeitsstatus und einen festen Quarantäne-Status: historische Forumdaten überschreiben keine aktuellen API-, Patch- oder Sheet-Daten und werden nicht automatisch als heutige Wahrheit in Antworten verwendet. Entwicklerantworten wie „fixed internally“ werden als historischer Fix-/Obsolete-Hinweis gespeichert, Community-Reports bleiben als unbestätigte historische Reports markiert.

Damit kann das Forum vollständig eingesammelt werden, ohne das aktuelle Wissen zu vergiften: alte Bugs, Exploit-Risiken, Meta-Diskussionen und Dev-Bestätigungen bleiben nachlesbar und zitierbar, aber sauber von aktuellen Fakten getrennt.

## #15 — Forum-Wissen kann schonend in Wellen ins Brain fließen

Das offizielle Deadlock-Forum war bisher nur indirekt über einzelne Patchnotes im Wissensspeicher vertreten. Diskussionen zu Bugs, Meta-Fragen, Heldenfeedback und Entwicklerantworten lagen dadurch außerhalb der Datenbasis, obwohl sie öffentlich und thematisch sehr wertvoll sind.

Das Brain kann jetzt öffentliche Forum-Threads aus der Sitemap von alt nach neu in kleinen, wiederholbaren Wellen importieren. Jede Thread-Seite wird als Rohquelle gespeichert und zusätzlich in strukturierte Thread- und Beitragsdaten zerlegt; bereits gespeicherte Threads werden beim nächsten Lauf übersprungen, damit ein großer Backfill sauber fortgesetzt werden kann. Bereiche, die das Forum per Robots-Regeln ausschließt, werden nicht gecrawlt; Attachment-Links werden nur als Referenz gesichert und nicht heruntergeladen.

Damit steht der sichere Grundkanal, um Forum-Inhalte erst vollständig zu sichern und danach getrennt auszuwerten: Bugs, Exploits als Risikosignale, Meta-Diskussionen, Heldenfeedback und später auch Bildhinweise aus erlaubten Quellen.

## #14 — Sehr lange Stream-Mitschnitte werden auswertbar (abschnittsweise)

Die längsten Quellen blieben bisher außen vor: mehrstündige Stream- und Coaching-Mitschnitte mit teils über einer Million Zeichen Transcript. Die Auswertung war auf Videos üblicher Länge begrenzt; alles darüber wurde übersprungen — obwohl gerade diese VODs besonders dichtes Spielwissen tragen.

Solche Mitschnitte werden nun in überlappende Abschnitte zerlegt, einzeln ausgewertet und ihre geprüften Aussagen anschließend wieder zusammengeführt; die Überlappung verhindert, dass eine Aussage an einer Abschnittsgrenze zerrissen und übersehen wird. Die Pipeline erkennt von selbst, ob eine Quelle als langer Mitschnitt oder als normales Video behandelt wird. Offensichtlich themenfremde Streams — etwa wenn ein Creator ein ganz anderes Spiel zockt — lassen sich gezielt aussortieren, bevor Auswertung in Inhalte fließt, die nichts über Deadlock aussagen.

Damit ist der Weg frei, auch die bisher offenen über hundert langen Mitschnitte in den geprüften Wissensschatz zu holen — abschnittsweise, gegen Dopplungen abgesichert und ohne themenfremden Ballast. Wie gewohnt wird vor jedem Schreibvorgang die Datenbank vollständig gesichert.

## #13 — Ertraglose Videos werden nicht mehr in jeder Welle neu versucht

Die Auswertung griff bei jedem Lauf erneut nach Videos, deren Transcript schlicht nichts Prüfbares hergibt — kurze Clips ohne belastbare Aussage. Das System merkte sich bisher nur, welche Videos bereits Wissen geliefert hatten, nicht aber, welche schon erfolglos versucht worden waren. Dadurch landeten dieselben ertraglosen Kurzvideos Welle für Welle wieder im Stapel und verbrauchten Auswertung, ohne je etwas beizutragen.

Das Brain führt jetzt ein eigenes Verarbeitungs-Verzeichnis: zu jedem ausgewerteten Video wird festgehalten, dass es versucht wurde und wie viele geprüfte Aussagen dabei heraus kamen — ausdrücklich auch dann, wenn es null waren. Die Auswahl für neue Wellen überspringt fortan alles, was schon einen Versuch hinter sich hat, und beschränkt sich von sich aus auf Videos in auswertbarer Länge; sehr lange Mitschnitte bleiben einem eigenen Verfahren vorbehalten. Die 42 bereits erfolglos geprüften Kurzvideos wurden in einem Zug als erledigt vermerkt, ohne sie erneut durch die Auswertung zu schicken. Wer es braucht, kann die Längenschranke gezielt aufheben.

Jede neue Welle zieht damit nur noch frische, lohnende Videos statt Ertragloses zu wiederholen; vor jedem Schreibvorgang wird die Datenbank wie gewohnt vollständig gesichert. Die Auswertung ist so sauber wiederholbar und bereit, auf die noch offenen Fälle — sehr lange Mitschnitte und Videos ohne Untertitel — ausgeweitet zu werden.

## #12 — Geprüftes Creator-Wissen vervielfacht: das Brain weiß jetzt deutlich mehr

Die neue Lernpipeline war fertig, aber erst an einer Handvoll Videos erprobt — das Wissens-Paket zu vielen Fragen blieb dünn, weil zu wenige geprüfte Creator-Aussagen in der Datenbank lagen (zu manchen Helden gab es nur eine einzige belegte Aussage).

Die Pipeline lief nun in zwei Wellen über rund 170 Strategie-Videos mit Transcript bis zu üblicher Länge. Jedes Video wurde ausgewertet und jede einzelne Aussage gegen die gesicherten Spieldaten geprüft und eingestuft: bestätigt, ungewiss, nicht prüfbar oder widerlegt. Konkrete Patch-Werte und Karten-Daten stützen die bestätigten Aussagen; die wenigen klar falschen Creator-Behauptungen — etwa eine verwechselte Fähigkeit oder eine falsche Zahl — werden ausdrücklich als widerlegt markiert, damit sie nicht weitergereicht werden.

Der geprüfte Creator-Wissensschatz ist von rund 100 auf über 2.000 Aussagen gewachsen und deckt jetzt über 130 Videos ab; damit liefert das Wissens-Paket zu deutlich mehr Fragen belegte Treffer. Sehr lange Stream-Mitschnitte und Videos ohne Untertitel stehen noch aus.

## #11 — Creator-Wissen aus Transcripts wird jetzt per wiederkehrendem Befehl geprüft eingespeist

Bisher entstand das transcript-basierte Creator-Wissen in einem einmaligen, von Hand zusammengesteckten Lauf — Sammeln, Prüfen und Einschreiben ließen sich nicht wiederholen, und erst eine Handvoll Videos war erfasst. Es fehlte ein fester, wiederholbarer Weg, neue Videos nachzuziehen, ohne den ganzen Ablauf jedes Mal neu zu orchestrieren.

Der Ablauf ist nun ein fester Befehl mit zwei Stufen. Die erste wählt gezielt die noch unbearbeiteten Strategie-Videos aus, die bereits ein Transcript haben, und stellt sie für die Auswertung bereit. Die zweite nimmt die ausgewerteten und gegen die gesicherten Spieldaten geprüften Aussagen entgegen und schreibt sie verlustfrei in die Wissensdatenbank — jede Aussage genau einmal (Doppelte werden zuverlässig erkannt und übersprungen), versehen mit ihrem Prüf-Urteil und der daraus folgenden Vertrauensstufe. Vor jedem Schreibvorgang legt der Befehl automatisch eine vollständige, in sich konsistente Sicherung der Datenbank an.

Damit lässt sich neues Creator-Wissen in klaren, wiederholbaren Wellen nachziehen, ohne den Ablauf jedes Mal neu zu bauen und ohne Gefahr für die bestehende Datenbasis. Das legt die Grundlage, die Auswertung über alle verbleibenden Strategie-Videos zu skalieren.

## #10 — Mehr Treffer trotz anderer Schreibweise, geordnet nach Relevanz

Auch nach der vorherigen Verbesserung fand das Wissens-Paket manches nicht: Stand eine Aussage in der Datenbank deutsch gebeugt, eine Frage aber englisch (oder umgekehrt), ging der Treffer verloren; gab es zu einer Frage nur ungeprüfte Creator-Aussagen, kam fast nichts zurück; häufige Allerweltswörter zogen beliebige Aussagen herein, während die seltenen, fragebestimmenden Begriffe untergingen; und die intern berechnete Relevanz-Reihenfolge kam nicht zuverlässig in der Ausgabe an.

Häufige Spielbegriffe werden jetzt über ihre Wortformen und nahe Entsprechungen abgeglichen, sodass unterschiedliche Schreibweisen denselben Treffer finden. Liegt zu einer Frage kaum gesichertes Wissen vor, werden die am besten passenden ungeprüften Aussagen als klar gekennzeichneter Notbehelf ergänzt, statt die Antwort leer zu lassen — die Vertrauensstufen bleiben dabei sauber getrennt. Seltene, aussagekräftige Begriffe wiegen bei der Gewichtung jetzt schwerer als allgegenwärtige Füllwörter, und nur Aussagen mit echtem Bezug zur Frage gelangen überhaupt in die Auswahl. Schließlich werden die gesammelten Fakten zuverlässig nach ihrer Passung zur Frage geordnet, bevor die Auswahl begrenzt wird, sodass das Wichtigste oben steht.

Damit liefert eine Frage jetzt auch dann passendes Wissen, wenn die Schreibweise abweicht oder nur ungeprüfte Creator-Aussagen vorliegen — letztere klar als solche markiert — und die relevantesten Fakten stehen vorn. Die vollständige Patch-Historie bleibt für Build-Fragen erhalten, damit Builds an die jüngsten Änderungen angepasst werden können.

## #9 — Wissens-Paket versteht die Frage jetzt richtig und ordnet nach Relevanz

Das neue Frage-Werkzeug lieferte zwar ein Wissens-Paket, verstand die Frage aber kaum: Jede Frage wurde wie eine allgemeine Heldenübersicht behandelt, der genannte Held oder das Item wurde in einem ganzen Satz nicht erkannt, und die gesammelten Aussagen wurden allein nach Quellen-Sicherheit sortiert — eine themenfremde, aber sicher belegte Aussage verdrängte so die eigentliche Antwort. Wissen, das unter einer anderen Bezugsgröße abgelegt war (etwa ein Konter-Item, das beim Konter und nicht beim Ziel steht), tauchte gar nicht auf; zufällige Zeichenketten-Treffer im Wortinneren erzeugten Rauschen; und selbst klar spielfremde Fragen wurden beantwortet, als gäbe es Spieldaten dazu.

Das Werkzeug erkennt jetzt die Art der Frage — Build, Item, Mechanik, Matchup/Konter, Patch-Änderung, Meta — und löst den genannten Helden oder das Item auch mitten in einem Satz auf. Die gesammelten Fakten werden danach geordnet, wie gut sie wirklich zur Frage passen, nicht mehr allein nach Quellen-Sicherheit. Wissen, das unter einer verwandten Bezugsgröße liegt, wird mitgefunden (das Konter-Item zu einem Helden erscheint auch dann, wenn es beim Item und nicht beim Helden verzeichnet ist). Es werden nur noch ganze Wörter abgeglichen, sodass eine Frage nach einem Helden keine zufällig ähnlich geschriebenen Begriffe mehr hereinzieht. Klar spielfremde Fragen werden als solche erkannt und ehrlich beantwortet, statt erfundene Treffer zu liefern. Für Build-Fragen bleibt die vollständige Patch-Historie erhalten, damit ein Build an die jüngsten Änderungen angepasst werden kann; nur reine Hilfsfelder ohne Inhalt bleiben aus dem an das Modell gehenden Text heraus.

Damit enthält das Wissens-Paket jetzt Fakten, die tatsächlich zur Frage gehören, in der richtigen Reihenfolge, mit korrekt aufgelöster Bezugsgröße, einschließlich des Wissens aus verwandten Einträgen und mit einem klaren Signal, wenn eine Frage außerhalb des Spiels liegt. Über alle geprüften Fragetypen hinweg ist die Trefferqualität deutlich gestiegen.

## #8 — Beliebige Frage rein, vertrauenssortiertes Wissens-Paket für ein Sprachmodell raus

Das Brain sammelt und prüft inzwischen Spieldaten und Creator-Wissen, doch es gab keinen Weg, zu einer konkreten Frage die passenden, nach Vertrauen gewichteten Fakten gebündelt an ein Sprachmodell zu übergeben. Man musste den genauen Befehl und den exakten Namen einer Entität kennen, die Ausgaben waren Rohdaten, und die geprüften Creator-Aussagen waren überhaupt nicht mit dem Abruf verbunden — sie lagen ungenutzt in der Wissensdatenbank.

Ein neuer Befehl nimmt eine beliebige Deadlock-Frage — Held, Item, Fähigkeit, Mechanik, Matchup, Build oder freie Frage — und baut daraus ein nach Vertrauen gestaffeltes Wissens-Paket: gesicherte Spieldaten zuoberst, darunter die gegen diese Daten geprüften Creator-Aussagen, darunter die nur teilweise bestätigten mit ausdrücklichem Vorbehalt, und schließlich die nachweislich falschen — letztere klar als Irrtum markiert, damit ein Modell sie nicht wiederholt. Die geprüften Creator-Aussagen fließen sowohl über die erkannte Entität als auch über eine wortgenaue Stichwortsuche ein, die zufällige Zeichenketten-Treffer im Wortinneren vermeidet. Heraus kommt beides: ein strukturiertes Bündel und ein fertiger Prompt, den ein externes Sprachmodell direkt nutzt, um zu antworten oder einen Build zu bauen.

Eine Frage genügt jetzt, und das Brain liefert das vertrauenssortierte Wissens-Paket für das Modell. Gesicherte Spieldaten schlagen bei jedem Widerspruch die Creator-Aussagen, und entlarvte Creator-Irrtümer werden klar als solche mitgeliefert, sodass das Modell aus belegten Fakten schöpft statt zu raten. Das Brain bleibt dabei der Zubringer; die eigentliche Antwort erzeugt das Sprachmodell.

## #7 — YouTube-Transcripts direkt laden statt aus dem Video raten

Die Lernpipeline hing bisher daran, dass ein externes Modell das Video „anschaut" und zusammenfasst — unzuverlässig, modellabhängig, und bei Videos ohne verfügbares Transcript lieferte es teils erfundene oder leere Ergebnisse. Außerdem war nicht unterschieden, welche Videos überhaupt aus Text auswertbar sind und welche ihren Inhalt im Bild tragen.

Ein neuer Schritt lädt die Untertitel der Videos in der Originalsprache direkt von der Quelle und legt sie als echten Transcript-Text ab — wiederholbar und ohne Dubletten; Videos ohne verfügbare Untertitel werden klar als solche markiert und für eine spätere Tonspur-Transkription vorgemerkt. Ein zweiter Schritt ordnet jedes Video einem Inhaltstyp zu: strategisch-verbal (Build, Lane, Matchup — aus Text auswertbar) gegenüber visuell-mechanisch (Bewegungs- und Technik-Demonstrationen, deren Inhalt das Bild trägt), und markiert rein visuelle Videos, deren Transcript allein wenig hergibt.

Damit steht eine verlässliche Transcript-Grundlage in Originalsprache für die spätere, sorgfältige Aussagen-Extraktion bereit, und visuelle Technik-Videos werden korrekt als transcript-arm erkannt statt überbewertet.

## #6 — Build-Vorschläge treffen jetzt, was starke Spieler wirklich kaufen

Der Build-Vorschlag pro Held empfahl im Schnitt nur rund ein Drittel dessen, was echte Quality-Spieler-Builds tatsächlich kaufen — bei spirit-lastigen Helden kippte er sogar in einen Waffen-Carry und ließ die eigentlichen Skalierungs-Items komplett weg. Ursache: Der deterministische Vorschlag ignorierte die bereits vorhandene, treffsichere Build-Analyse und optimierte vorrangig die Shop-Bonus-Ökonomie statt die tatsächliche Schadensquelle des Helden; ganze Item-Klassen wie Cooldown-Reduktion oder defensive Aktiv-Items wurden gar nicht bewertet.

Die Build-Analyse fließt jetzt als harter Faktor in den Vorschlag ein: Als Kern erkannte Items werden hochgewichtet, als fraglich markierte abgewertet, und Items, die häufig in echten Quality-Builds vorkommen, bekommen einen Bonus. Das Schadensprofil eines Helden (Spirit- gegenüber Waffen-Fokus) wird belastbarer bestimmt und steuert die Item-Gewichtung — bei spirit-dominanten Helden werden fehlplatzierte Waffen-Kernitems gezielt abgewertet. Drei zuvor unbewertete Item-Klassen — Cooldown-Reduktion, Spirit-Resistenz-Verringerung und defensive Aktiv-Items — werden nun nach Heldenbedarf gewertet, während die Shop-Bonus-Heuristik als Haupttreiber zurückgestuft wurde, sodass effektlose Allzweck-Items keine Kernränge mehr belegen.

Der Vorschlag deckt sich dadurch im Schnitt mehr als doppelt so stark mit echten Quality-Builds; spirit-lastige Helden erhalten ihre Skalierungs- und Cooldown-Kerne statt deplatzierter Waffen-Items, und die Item-Detailanzeige stellt Vorzeichen und Werte wieder korrekt dar.

## #5 — Mehr Patch-Änderungen und Creator-Aussagen korrekt einem Helden, Item oder einer Fähigkeit zugeordnet

Viele Patch-Änderungen und Creator-Aussagen lagen ohne klaren Besitzer im Wissensspeicher: Der Betreff einer Patch-Zeile oder der genannte Bezug einer Aussage ließ sich nicht auf einen bekannten Helden, ein Item oder eine Fähigkeit abbilden, weil der Entitäten-Katalog zum Zeitpunkt der Verarbeitung noch unvollständig war. Solche Einträge blieben unzugeordnet und tauchten daher nicht auf, wenn man gezielt nach dem betreffenden Helden oder Item fragte.

Ein neuer, bewusst zurückhaltender Abgleich ordnet diese besitzerlosen Einträge dem inzwischen deutlich reicheren Entitäten-Katalog zu — einschließlich der neuen Spieldaten-Aliasse und der im Spiel sichtbaren Anzeigenamen. Zugeordnet wird nur, wenn der bereinigte Name eindeutig zu genau einer bekannten Entität passt; zusammengesetzte und annotierte Bezeichnungen (Schrägstriche, in Klammern genannte Fähigkeiten, Sprach-Zusätze) werden zerlegt und Teil für Teil geprüft, während mehrdeutige oder unauflösbare Namen absichtlich unangetastet bleiben. Ein erneuter Lauf verändert nichts bereits Zugeordnetes.

Dadurch ist ein spürbarer Teil der zuvor besitzerlosen Patch-Änderungen und Creator-Aussagen jetzt korrekt mit seinem Helden, Item oder seiner Fähigkeit verknüpft — auch über die Anzeigenamen, die Spieler tatsächlich sehen. Die verbleibenden offenen Einträge sind entweder wirklich allgemein (Spielsysteme, globale Regeln) oder zu mehrdeutig für eine sichere Zuordnung.

## #4 — Vertrauenswürdige Spieldaten-Quelle: Scaling, Helden- und Item-Details, strukturierte Patch-Daten

Dem Wissensspeicher fehlten die maßgeblichen, strukturierten Spieldaten — vor allem die Skalierungswerte der Helden sowie die Detailangaben zu Fähigkeiten und Items. Diese Informationen lagen bislang nur auf der gerenderten Wiki-Seite, die von diesem System aus nicht erreichbar ist; die vorhandenen Patch-Daten stammten aus weniger strukturierten Quellen.

Neu ist eine als vertrauenswürdig gekennzeichnete Datenquelle, die das offizielle Daten-Repository des Spiels anzapft — dieselbe Ursprungsquelle, aus der auch die Wiki-Seiten erzeugt werden. Sie hält dieses Repository lokal aktuell und liest daraus Helden, Fähigkeiten und Items samt ihrer Skalierungsfaktoren sowie die strukturierten Änderungsprotokolle der Patches. Alle so gewonnenen Datensätze werden als gesicherte Grundwahrheit markiert und damit klar von den noch ungeprüften Creator-Aussagen abgegrenzt. Der Import ist gefahrlos wiederholbar: unveränderte Inhalte erzeugen keine Dubletten, und zwar auch dann, wenn die Ursprungsquelle zwischenzeitlich aktualisiert wurde — die Wiedererkennung hängt am Inhalt, nicht an der Herkunftsversion.

Damit enthält der Wissensspeicher jetzt die belastbaren Skalierungswerte und strukturierten Entitätsdaten, die zuvor fehlten. Wiederholte Läufe und Aktualisierungen der Quelle reichern den Bestand an, ohne ihn aufzublähen, und die gesicherten Daten sind als solche erkennbar.

## #3 — Brain auf Rust umgestellt: eine Sprache, gleiche Befehle

Das Brain — das Werkzeug, das Patchnotes, Statistiken, Sheet- und Creator-Wissen einsammelt und zu abrufbarem Spielwissen verdichtet — lief bisher in Python, während der Rest der Plattform längst auf Rust läuft. Eine in zwei Sprachen geteilte Codebasis ist schwerer zu warten und weiterzuentwickeln.

Die gesamte Funktionalität wurde nach Rust überführt. Ein gemeinsamer Kern trägt jetzt Datenbankzugriff, Datenbankschema, Konfiguration, die Netz-Abrufe und die Modellanbindung; darauf sitzen klar getrennte Fachbausteine für Quellen-Import, Normalisierung der Entitäten, Anreicherung der Patch-Daten, die Lern-Auswertung von Builds und Matches sowie den Abruf von Kontext, Timeline und Reviews. Alles ist unter einem einzigen Kommandozeilen-Werkzeug zusammengefasst, das exakt dieselben Befehle anbietet wie zuvor. Das Datenbankschema wurde aus dem Live-Bestand übernommen und nachweislich deckungsgleich nachgebaut, sodass beide Fassungen dieselbe Wissensdatenbank teilen.

Das Rust-Werkzeug arbeitet damit gegen dieselbe Datenbank und liefert dieselben Befehle; gegen eine Kopie der echten Datenbank wurden alle Abruf-Befehle erfolgreich gegengeprüft. Die bisherige Python-Fassung bleibt vorerst der aktive Hintergrunddienst — die Umstellung der Automatik auf die Rust-Fassung folgt als eigener, geprüfter Schritt. Die Vektor-Ähnlichkeitssuche ist bewusst noch ausgeklammert und kommt später.

## #2 — Autonomes YouTube-Lernen: Creator-Wissen fließt in die Wissens-DB

Das System sammelt zwar seit Längerem Videos der kuratierten Deadlock-Creator ein, doch das darin steckende Spielwissen wurde bisher nicht nutzbar gemacht — es gab keinen Weg, die Inhalte automatisch zu verstehen und strukturiert abzulegen. Wer die Erkenntnisse aus einem Coaching- oder Meta-Video wollte, musste es selbst schauen.

Neu ist ein eigenständiger Lern-Lauf: Aus den festen Creator-Feeds werden die frischesten Videos gezogen, und jedes davon wird in zwei Schritten ausgewertet. Zuerst wird das Video vollständig angeschaut und sein Inhalt in natürlicher Sprache zusammengefasst; anschließend wird diese Zusammenfassung in einzelne, überprüfbare Aussagen zerlegt — Builds, Item-Timings, Matchups, Mechaniken, Combos und Meta-Einschätzungen — und jede Aussage mit Bezug auf den betroffenen Helden oder das Item sowie einer Einschätzung ihrer Eindeutigkeit in der Wissensdatenbank abgelegt. Videos ohne verwertbares Spielwissen werden als geprüft, aber leer vermerkt; einzelne Fehlschläge stoppen den Lauf nicht, sondern werden beim nächsten Mal erneut versucht. Bereits ausgewertete Videos werden übersprungen, sodass ein Lauf gefahrlos wiederholbar ist.

Damit füllt sich die Wissensbasis bei jedem Lauf von selbst und ist so gebaut, dass sie unbeaufsichtigt im Hintergrund läuft, ohne manuelles Zutun. Die gesammelten Aussagen sind zunächst als ungeprüft markiert; eine spätere Stufe gleicht sie gegen die harten Patch- und Statistikdaten ab und gewichtet sie.

## #1 — Sheet-Sync: Enrichment-Schritt brach jeden Lauf ab

Der periodische Sheet-Sync-Job (Lauf alle 4 Stunden) hat die ersten Schritte — Sheet-Abgleich, Normalisierung und die Build-Analyse — sauber durchlaufen, ist dann aber im Enrichment-Schritt jedes Mal mit einem Fehler abgebrochen. Weil die Schritte als verkettete Abfolge laufen (jeder startet nur, wenn der vorige ohne Fehler endet), wurden die nachgelagerten Teile — die Patch-Impact-Analyse und die Meta-Trends — danach nie mehr ausgeführt. Der gesamte Job galt damit als fehlgeschlagen.

Ursache war ein fehlender optionaler Schalter: Der Patch-Impact-Teil entscheidet intern anhand eines Trockenlauf-Schalters, ob er nur die Anfrage bauen oder das Modell wirklich aufrufen soll. Dieser Schalter wurde im Code abgefragt, war für genau diesen Teilbefehl aber nie als Option deklariert. Der Zugriff darauf lief deshalb sofort in einen Fehler — noch bevor irgendeine eigentliche Arbeit passierte. Der fehlende Schalter wurde nachgetragen.

Jetzt ist der Trockenlauf-Schalter beim regulären Lauf standardmäßig aus, der Enrichment-Schritt läuft normal durch, und die Kette bis zu den Meta-Trends wird wieder vollständig abgearbeitet.
