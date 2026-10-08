# G: Antwortanschluss für K

Stand: 07.10.2026. An den tatsächlichen G-Quellen nachgelesen, Produktstand dbce14aedadd94881a3cb21151d9840994094cd9. Dieser Stand ist inzwischen als Vorfahr des gesicherten Featurecheckpoints F2 `2b67796fb80ae3440a0c9e76671dfc8169032844` auf origin belegt: Bereichsführung bestätigte ls-remote und `merge-base --is-ancestor` mit Exit 0. Diese Dokumentdatei selbst ist noch nicht committed. Keine uncommittierten Quellen kopieren. K besitzt zentrale service.rs-Verdrahtung und Consumer; G baut dort keinen parallelen Anschluss.

## Ein bestehender Kernel

`brain_kernel::Kernel::with_tools(port, resolver, provider_identity)` ergänzt den vorhandenen Kernel. Der Toolport und GameContextResolver sind Send + Sync + 'static. Provideridentität stammt aus der bestätigten zentralen Konfiguration, kein neues Modell und kein Connector. Der bestehende CachedKernel bleibt der gemeinsame Cache-/Flightweg.

`GameContextResolver::resolve(&Query, &AuthorizedContext) -> Result<Option<PinnedGameContext>, PortError>` wählt den echten Spiegel-Pin einmal je Anfrage. `validate(&Query, &AuthorizedContext, Option<&PinnedGameContext>) -> Result<(), PortError>` prüft ihn erneut. Keine statische Startup-Version, kein Pin aus Modellargumenten. Historische DomainRequest-Anfragen behalten ihren bestehenden Pfad.

`AnswerKernelPort::answer_accounted` und `answer_for_publication_accounted` liefern `Accounted<AnswerResponse>`. Bots benutzen den Veröffentlichungszweck; InternalRead ist ein anderer Zweck. Die kompatiblen alten Methoden bleiben erhalten. Ihre Traitdefaults ersetzen keine vollständige Abrechnung: Der Default-Kernelanschluss markiert seine Abrechnung ausdrücklich unaccounted.

Fundstellen: brain-kernel/src/lib.rs:19,62,118 sowie die tatsächlichen Implementierungen ab :273.

## Providerwrapper vollständig delegieren

AnswerProviderPort enthält:

- answer_accounted(query, context, evidence) -> Result<Accounted<ProviderAnswer>, PortFailure>
- answer_turn_accounted(query, context, evidence, tools, conversation) -> Result<Accounted<ProviderTurn>, PortFailure>
- kompatible answer und answer_turn.

Beide konkreten G-Provider implementieren diese Methoden. Bei Werkzeugfolgen werden die vollständigen gesammelten Evidence einschließlich unzitierter Abhängigkeiten übergeben. Kein leerer Ersatzslice und keine abgeschnittene Historie.

Auf dem geprüften G-Stand besitzt service.rs:124 den bestehenden Enum AnswerProvider. Seine Implementierung ab :129 delegiert bisher ausschließlich answer. Der Traitdefault answer_turn verweigert nichtleere Tools/Historie; der Default accounted-Vertrag verliert konkrete Fehlerabrechnung. K muss deshalb seine zentrale Wrapperverdrahtung gegen die echten Methoden integrieren und beide Formen vollständig delegieren. Das ist ein Anschlussbedarf des G-Stands, keine Aussage über inzwischen veränderte K-Quellen. Bei :431 wird bisher Kernel::new ohne Toolbindung erzeugt. Keine zweite Antwortengine zur Umgehung dieser Defaults.

Fundstellen: brain-contracts/src/lib.rs:592; brain-providers/src/lib.rs:119,289; brain-serve/src/service.rs:124,431.

## Abrechnung und deterministischer Build

UsageAccounting trennt observed, reserved und unaccounted. charged() kombiniert beobachteten Verbrauch mit zusätzlicher konservativer Budgetbelastung. Reserved ist keine behauptete gemessene Tokenzahl. PortFailure enthält error und Option<Box<UsageAccounting>>. before_call bezeichnet lokale Ablehnung vor Fremdarbeit; fehlende Abrechnung reserviert im Kernel das gesamte Restbudget und beendet die Anfrage. Kein zweiter Ledger im Consumer und kein neuer Budgetrahmen pro Runde.

ToolExecutionPort::execute_accounted erhält die ursprüngliche Anfrage, den AuthorizedContext, den serverseitigen Pin, call_id und typisierten ToolRequest. validate_dependencies prüft die vollständigen typisierten ToolEvidenceDependency für den angefragten Zweck. Defaults verweigern fehlende kanonische Prüfanschlüsse.

validate_build_plan erhält tatsächlichen BuildPlanRequest, ToolExecution, PinnedGameContext, ursprünglichen Kontext und AnswerPurpose. I liefert Fs echten reinen Planeingang; G-V bindet dessen deterministisches Ergebnis daran. Gewöhnliche Belege oder eine bloße BuildPlan-Definition reichen nicht. Ergebnis und Abhängigkeiten bleiben bei Cache-/Flightwiederverwendung prüfbar. Der tatsächliche Produktionsadapter ist noch offen.

Fundstellen: brain-contracts/src/lib.rs:308,335,360,367; brain-contracts/src/tools.rs:874,893,909,923.

## Erstturnfreigabe und Datenschutz

NIT des gemeinsamen Gates: Bei leerer anfänglicher Belegmenge liegt die Anfrage-Egressprüfung beim vertrauenswürdigen Provider. Beide konkreten Provider prüfen auch dann query/context und provider_egress vor Transport. authorize_turn prüft zusätzlich Historie, doppelte IDs und die tatsächlichen Toolbeleg-IDs. Ein eigener Wrapper darf diese Prüfung nicht durch einen ungeprüften Text-/Tooltransport ersetzen.

Die technische Scope-/Egressfähigkeit allein ist keine Freigabe für private oder Community-Inhalte. Maßgeblich ist jetzt ENTSCHEIDUNG-DATENSCHUTZ-NUTZER-2045.md in der korrigierten Fassung: In der Testphase dürfen Discord-/Twitch-Fragen samt minimal nötigem bereinigtem Antwortkontext über den bestehenden Luna-Abo-Provider laufen. Discord-/Steam-IDs, Mitgliederlisten und fremde Personendaten NEVER mitsenden. Die Brücke muss die tatsächlich fragende Person intern prüfen und ausschließlich deren erlaubte Kanalsicht liefern. Keine zusätzliche harte Kategoriesperre. Private Originale und Community-Rohdaten MUST NOT an Codiermodelle oder Git gehen. Kein Modellwechsel oder neuer Provider durch G.

Die dort bestätigte bestehende Invite-Ausnahme bleibt begrenzt auf den eigenen Status als Enum plus Zeitpunkt nach interner Identitäts-/Rechteprüfung. Keine Namen, Steam-IDs, Fremddaten, Rohzeilen oder unbereinigten Fragen-/Kontextdaten; G erweitert diesen eigenen Statusweg nicht. Öffentliche Spielproben ersetzen keine echte private Antwortabnahme.

Fundstellen: brain-providers/src/hardening.rs:29,81; Gate-NIT execution.rs:207; aktuelle Entscheidung im zentralen Aufgabenort .tasks/2026-10-07-brain-fertigstellung-astra/ENTSCHEIDUNG-DATENSCHUTZ-NUTZER-2045.md. ENTSCHEIDUNG-PARALLEL-FERTIGSTELLEN.md gestattet K die sofortige Integration gegen gesicherte G-Commits ohne Warten auf G-main.

## Beweisgrenze

JSON-/Fehlervertrag a6568629 separat regulär ALLOW. Gemeinsamer Provider-/Kernelgate a6568629..dbce14ae, Task bejusihil, Exit 0: [gpt-6.1-sol] ALLOW: No blocking defect established by the supplied code. Sechs konkrete HTTP-Loopbackanschlüsse bestanden; vollständige betroffene Suites 181 passed/25 failed/0 ignored, 423/423 Quellbindung bestätigt. Werkzeugdaten und Spielbindung dort Fakes, kein echter Luna-/Spiegel-/F-Build-/Discord-/Twitchbeweis.

K integriert die zusammengehörigen Wrapper, Reader und Consumer nach gesicherter G-/I-Lieferung und prüft sie gemeinsam. Diese Beschreibung ist keine integrierte K- oder G-Abnahme. Herkunft, Deadline, kumulierte Budgets und Veröffentlichungsfreigaben bleiben Bestandteil desselben Antwortwegs.
