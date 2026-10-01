# Deadlock Brain: Konsolidierungs- und Jev-Architekturstudie

**Stand: 24. September 2026**

## Gesamturteil

Dein gewünschtes Ziel ist aus meiner Sicht richtig: **`Deadlock-Brain` sollte künftig das einzige aktive Repository für die gesamte AI-/Knowledge-Infrastruktur sein.** Aber wichtig ist die Form: nicht „alles in eine große Anwendung werfen“, sondern **ein Monorepo mit einem einzigen zentralen Answer-/Retrieval-Kernel und mehreren dünnen Adaptern**.

Der jetzige Zustand ist nicht einfach „ein RAG, das nur etwas verteilt liegt“. Es gibt mehrere unterschiedliche Verantwortlichkeiten, die historisch in verschiedenen Repositories gelandet sind:

| Bereich | Heute erkennbar | Ziel |
|---|---|---|
| Deadlock-Domainlogik | `Deadlock-Brain` | bleibt Kern |
| öffentliche Deadlock-Fragen | RAG-/Bot-Pfade | zentraler Answer Kernel |
| Twitch-Antworten | `Deadlock-Twitch-Bot` mit eigener Integration | dünner Adapter zum Brain |
| internes Wissen | `Deadlock-2nd-Brain` Markdown | internes Knowledge-Scope im Brain |
| Knowledge-Erzeugung | Brain Feeder in `Deadlock-Docs` | Ingestion-Worker im Brain |
| Dokumentation/Inhalte | `Deadlock-Docs` | Knowledge Source / ggf. Docs-App im Brain |
| LLM-Auswahl | teilweise Consumer-spezifisch | zentral im Brain |
| Retrieval/Routing | historisch mehrere Pfade | genau eine Retrieval-Pipeline |
| Jev | noch nicht Teil des Systems | Router, Filter, Gate, Scorer vor dem LLM |

Der wichtigste Architekturwechsel lautet deshalb:

> **Nicht die Repositories zusammenkopieren und danach weitermachen wie bisher. Stattdessen die AI-Entscheidungslogik zentralisieren.**

Das Zielbild sollte nur noch diesen Weg kennen:

```text
Twitch ─────────┐
Web / API ──────┤
MCP / Agent ────┤
CLI ────────────┤
Interne Frage ──┘
        │
        ▼
┌──────────────────────────────┐
│      Deadlock Brain API      │
│ Auth · Scope · Query Context │
└──────────────┬───────────────┘
               │
               ▼
       ┌───────────────┐
       │ Jev Decisions │
       │ Route / Budget│
       └───────┬───────┘
               │
               ▼
       Hybrid Retrieval
    lexical + dense + filter
               │
               ▼
       Jev Relevance Gate
               │
          5–8 Evidenzen
               │
               ▼
      Finales Antwortmodell
               │
               ▼
 Antwort + Quellen + Confidence
```

Jev passt dafür ungewöhnlich gut. TypeSafe beschreibt Jev explizit **nicht als Chat-/Textmodell**, sondern als Modell für strukturierte Entscheidungen: Ein `state` wird mit typisierten Fragen bewertet; zurück kommen beispielsweise Choice-, Score- oder Wahrheitswahrscheinlichkeiten. Mehrere solcher Fragen können in einem Aufruf verarbeitet werden. citeturn4view2 Das ist praktisch genau die Arbeit, die du **vor** einem teuren generativen Modell erledigen möchtest.

Mein Urteil über die heutige Stärke von `Deadlock-Brain` ist daher zweigeteilt:

**Als Deadlock-Domaincode ist das Repository bereits deutlich stärker als ein kleines RAG-Projekt. Als zentraler „Brain Service“, durch den wirklich jede AI-Antwort läuft, ist die Architektur aber noch nicht konsequent genug.**

Im aktuellen Python-Paket sind bereits umfangreiche Komponenten wie `brain_pipeline.py`, `build_learning.py`, `build_optimizer.py`, `entity_normalizer.py`, `lineage.py`, `match_coaching.py`, `meta_trends.py` und `minimax_client.py` zu sehen. Das spricht für eine inzwischen recht breite Domain-Engine statt eines einzelnen Frage-Antwort-Skripts. fileciteturn13file0L1-L2 Gleichzeitig existieren im Repository bereits weitere Bereiche wie Service-, MCP- und Rust-Komponenten. **Der richtige Anker für die Konsolidierung ist deshalb klar `Deadlock-Brain`, nicht eines der anderen Repositories.**

## Was heute über die Repositories verteilt ist

### Deadlock-Brain

`Deadlock-Brain` ist nach dem aktuellen Repository-Inventar inzwischen wesentlich mehr als nur Knowledge Retrieval. Schon das Hauptpaket beinhaltet Analyse-, Normalisierungs-, Pipeline-, Build-Learning-, Build-Optimierungs-, Lineage-, Match-Coaching- und Meta-Funktionalität. fileciteturn13file0L1-L2

Das ist positiv, weil damit die schwerste fachliche Logik bereits dort sitzt, wo sie langfristig hingehört.

Problematisch ist eher die **Abgrenzung nach außen**: Ein zentraler Brain-Service ist nur dann wirklich zentral, wenn Twitch, interne Agents, MCP, Website usw. nicht selbst entscheiden,

- welches Modell verwendet wird,
- welche Wissensquelle durchsucht wird,
- ob RAG benutzt wird,
- welche Dokumente ins Prompt kommen,
- wie ein Query umgeschrieben wird,
- wann keine ausreichende Evidenz vorhanden ist,
- wie Quellen und Confidence behandelt werden.

Genau diese Dinge sollten künftig Eigenschaften des Brain sein und nicht Eigenschaften des Consumers.

### Deadlock-2nd-Brain

Das bereitgestellte `Deadlock-2nd-Brain`-Archiv ist architektonisch etwas ganz anderes als der Name zunächst erwarten lässt.

In `CLAUDE.md` wird es ausdrücklich als **Markdown-basiertes Wissensarchiv und nicht als Code-Projekt** beschrieben. Die enthaltene Struktur besteht unter anderem aus Bereichen für Infrastruktur, Projekte, Systeme, Entscheidungen, Daily Notes, Wissen, Ideen und Debugging.

Das ist wichtig: **`Deadlock-2nd-Brain` ist eigentlich ein Corpus, kein Service.**

Die im Archiv dokumentierte Kette sieht sinngemäß so aus:

```text
GitHub-Aktivität
OpenClaw Sessions
Dokumentation
weitere Signale
       │
       ▼
Brain Feeder
(in Deadlock-Docs)
       │
       ▼
Claude Haiku
Relevanz / Extraktion
       │
       ▼
strukturierte Markdown-Dateien
       │
       ▼
Deadlock-2nd-Brain
```

Die Projektbeschreibung `projekte/brain-feeder.md` dokumentiert den Feeder als tägliche Pipeline aus `Deadlock-Docs`, historisch unter `scripts/brain/`, einschließlich Scan-State und GitHub-Workflow. Das heißt: **Der Prozess, der das Second Brain erzeugt, sitzt nicht einmal im Second-Brain-Repo selbst.**

Noch wichtiger ist `projekte/ai-zweitgehirn.md`: Dort wird das Second Brain als Knowledge Base beschrieben, die **nicht selbst als RAG/API angeboten wird**. Menschen beziehungsweise Agents lesen die Markdown-Dateien. Parallel dazu wird ein separates „Deadlock-Rag“ als RAG-Pfad erwähnt.

Damit hast du aktuell zwei verschiedene Konzepte:

```text
Second Brain
= persistentes internes Kontext-/Projektgedächtnis

RAG
= Laufzeit-Suche für Fragen
```

Diese Konzepte müssen nicht verschwinden. **Sie sollten aber zwei Schichten desselben Systems werden.**

Das Second Brain ist künftig einfach eine interne Knowledge Source des Deadlock Brain:

```text
knowledge/internal/...
        │
        ▼
gemeinsame Ingestion
        │
        ▼
interner Retrieval-Scope
```

### Deadlock-Docs

Das strukturelle Problem von `Deadlock-Docs` ist weniger „zu viele Docs“, sondern dass es laut den Unterlagen zusätzlich **Automatisierungsverantwortung für das Brain übernommen hat**.

Insbesondere der Brain Feeder gehört konzeptionell nicht in ein Docs-Repository.

Das führt heute zu dieser merkwürdigen Abhängigkeit:

```text
Deadlock-Docs
   │
   ├── Dokumentation
   │
   └── Brain Feeder
          │
          ▼
Deadlock-2nd-Brain
```

In der Zielarchitektur sollte daraus werden:

```text
Deadlock-Brain
│
├── knowledge/public/...
├── knowledge/internal/...
│
└── apps/worker
       │
       └── ingestion / feeder / sync
```

Falls `Deadlock-Docs` zusätzlich eine echte Website oder Dokumentationsanwendung enthält, muss diese nicht verschwinden; sie wird zu einem **Deployable innerhalb desselben Monorepos**, etwa `apps/docs-site/`. Entscheidend ist, dass sie keine eigene AI-Plattform mehr ist.

### Deadlock-Twitch-Bot

Besonders relevant für deine Frage nach den „KI-Antwortpfaden“ ist die historische Dokumentation in `Deadlock-2nd-Brain/systeme/twitch-bot.md`.

Dort wird der Twitch Bot als Node.js-Anwendung mit `src/bot.js` beschrieben. AI-Antworten laufen laut Dokumentation über OpenRouter; für Deadlock-Fragen wird ein RAG-Backend verwendet. Die interne Notiz dokumentiert außerdem einen Wechsel am **23. Februar 2026** von Citation-Validator/Promptfoo-orientierten Pfaden auf „Deadlock-Rag“.

Das zeigt genau das Problem, das du vermutet hast:

```text
Twitch Chat
    │
    ▼
Twitch Bot
    │
    ├── eigene Commands
    ├── eigener AI-Provider-Pfad
    └── eigener RAG-Integrationspfad
               │
               ▼
          Deadlock-Rag
```

Das sollte **nicht** die langfristige Architektur bleiben.

Der Twitch Bot sollte von AI möglichst wenig wissen:

```text
Twitch Chat
   │
   ▼
Deadlock Twitch Adapter
   │
   │ POST /v1/answer
   ▼
Deadlock Brain
   │
   ├── Retrieval
   ├── Jev
   ├── LLM
   ├── Quellen
   ├── Cache
   └── Policy
   │
   ▼
kurze Twitch-Antwort
```

Der Bot darf weiterhin Twitch-spezifische Dinge besitzen: Authentifizierung, Chat-Events, Rate Limits, `!ask`, `!item`, Moderation, Message Splitting, Emotes und so weiter.

**Er sollte aber keinen eigenen RAG-Stack, kein eigenes Prompting und möglichst auch keine direkte OpenRouter-Entscheidungslogik mehr besitzen.**

### Der auffällige „Deadlock-Rag“-Rest

Ein wichtiger Befund aus den historischen Knowledge-Dateien ist der wiederholte Begriff **`Deadlock-Rag`**, obwohl dein heutiges Zielrepo `Deadlock-Brain` heißt.

Das sieht nach Architektur-/Namensdrift aus:

```text
früher:
Deadlock-Rag = Retrieval/QA

später:
Deadlock-Brain = wesentlich breitere Domainplattform

aber:
ältere Consumer / Dokumentation
referenzieren weiterhin Deadlock-Rag
```

Ich würde „Deadlock-Rag“ deshalb nicht als eigenes zukünftiges System erhalten.

**RAG wird eine interne Capability von Deadlock-Brain:**

```text
Deadlock Brain
└── retrieval/
    ├── lexical
    ├── dense
    ├── hybrid
    ├── reranking
    ├── filters
    └── evidence
```

Der Produkt-/Servicebegriff sollte nur noch **Deadlock Brain** sein.

## Wie die Antwortpfade künftig aussehen sollten

Der wichtigste Umbau ist ein zentraler **Answer Kernel**.

Heute sind Knowledge-Erzeugung, Knowledge-Speicherung, Retrieval und finale Antworten teilweise voneinander getrennt. Künftig sollte jede Frage zunächst durch dieselbe Pipeline laufen.

### Ein universelles Query-Objekt

Nicht Twitch, interne Fragen und normale Deadlock-Fragen als getrennte APIs bauen. Ein Request sollte ungefähr so aussehen:

```json
{
  "query": "Was countert aktuell Haze im Midgame?",
  "channel": "twitch",
  "actor": {
    "type": "anonymous"
  },
  "scopes": [
    "deadlock.public"
  ],
  "locale": "de-DE",
  "conversation_id": "optional",
  "response_profile": "twitch-short"
}
```

Eine interne Frage wäre derselbe Vertrag:

```json
{
  "query": "Warum haben wir den Citation Validator beim Twitch Bot entfernt?",
  "channel": "internal",
  "actor": {
    "type": "developer",
    "id": "..."
  },
  "scopes": [
    "deadlock.public",
    "brain.internal",
    "github.code",
    "architecture"
  ],
  "locale": "de-DE",
  "response_profile": "internal-detailed"
}
```

Das ist ein gewaltiger Unterschied.

Die **Frage** ist nicht mehr „welchen Endpoint rufe ich an?“, sondern:

> Welche Scopes darf dieser Benutzer durchsuchen, und welche Darstellung benötigt der Consumer?

### Scope und Berechtigungen müssen vor AI kommen

Da öffentliche Deadlock-Daten und dein internes Zweitgehirn im selben Repository landen sollen, ist eine Sache besonders wichtig:

**Ein Repository darf ein gemeinsamer physischer Ort sein. Das bedeutet nicht, dass alles ein gemeinsamer Retrieval-Pool sein darf.**

Ich würde mindestens folgende Scopes definieren:

```text
deadlock.public
deadlock.analytics
brain.internal
github.code
infra.internal
stream.context
personal.private
```

Die Berechtigung darauf wird **normaler Code**, nicht Jev und nicht das LLM.

Also:

```text
User
 │
 ▼
Auth
 │
 ▼
Allowed scopes
 │
 ▼
Retrieval
```

und niemals:

```text
User
 │
 ▼
Jev/LLM entscheidet,
ob interne Daten okay sind
```

Das verhindert, dass eine öffentliche Twitch-Frage versehentlich einen Chunk aus internen Deployment-, Infrastruktur- oder Projektinformationen bekommt.

### Ein gemeinsames Dokumentmodell

Alle Quellen sollten beim Ingest auf dasselbe Schema normalisiert werden:

```yaml
document:
  document_id: "..."
  source: "github"
  repository: "Deadlock-Brain"
  path: "knowledge/internal/systeme/twitch-bot.md"
  commit_sha: "..."
  title: "Twitch Bot"
  domain: "architecture"
  visibility: "internal"
  language: "de"
  created_at: "..."
  updated_at: "..."
  content_hash: "..."

chunk:
  chunk_id: "..."
  document_id: "..."
  heading: "AI Antworten"
  text: "..."
  token_count: 247
  entities:
    - "Deadlock-Twitch-Bot"
    - "OpenRouter"
  valid_from: "..."
```

Vor allem diese Felder sind nicht optional:

```text
source
path
commit/version
visibility
domain
updated_at
content_hash
```

Damit kann das System bei jeder Antwort sagen, **woher** das Wissen stammt und **wie frisch** es ist.

### Source of Truth und generiertes Wissen trennen

Ich würde nicht einfach alle `.md`-Dateien nebeneinander legen.

Es sollte einen klaren Unterschied geben:

```text
knowledge/
├── public/
│   └── deadlock/
│
├── internal/
│   ├── architecture/
│   ├── projects/
│   ├── infrastructure/
│   └── decisions/
│
└── generated/
    ├── github-digests/
    ├── session-digests/
    └── daily/
```

Ein vom Brain Feeder erzeugter Tagesbericht ist nämlich nicht dasselbe wie eine von dir bewusst gepflegte Architekturentscheidung.

Das ermöglicht später Gewichtungen:

```text
authoritative docs         weight 1.0
code / config              weight 1.0
curated decisions          weight 0.9
generated project summary  weight 0.7
daily/session memory       weight 0.5
```

Die konkreten Zahlen müssen evaluiert werden; wichtig ist das Prinzip.

### Ein Answer Kernel statt Consumer-spezifischer Prompts

Die zentrale Pipeline sollte ungefähr so aussehen:

```text
request
  │
  ▼
identity + allowed scopes
  │
  ▼
query normalization
  │
  ▼
Jev query decisions
  │
  ├── domain
  ├── intent
  ├── freshness requirement
  ├── retrieval breadth
  └── answer complexity
  │
  ▼
candidate retrieval
  │
  ├── lexical
  ├── dense/vector
  ├── metadata
  └── domain-specific retrieval
  │
  ▼
Jev candidate relevance
  │
  ▼
evidence pack
  │
  ▼
answerability gate
  │
  ├── insufficient → broaden / no-answer
  │
  └── sufficient
  ▼
final LLM
  │
  ▼
source verification
  │
  ▼
response formatter
  │
  ├── Twitch
  ├── API
  ├── MCP
  └── internal
```

Damit hast du **eine AI-Architektur und viele Interfaces**, anstatt viele AI-Architekturen zu pflegen.

## Jev ist für deine „Vorarbeit“-Idee sehr gut geeignet

Hier ist der wahrscheinlich interessanteste Teil deiner Frage.

TypeSafe positioniert Jev genau als Maschine für Entscheidungen und nicht für Textgenerierung. Die offizielle Dokumentation beschreibt den Vertrag als `state + typed questions → structured results`; verfügbar sind `Choice`, `Score` und `Noul`, jeweils mit strukturierten Wahrscheinlichkeiten beziehungsweise Confidence. Die Dokumentation empfiehlt außerdem kleine, atomare Entscheidungen und das anschließende Kombinieren der Ergebnisse in normalem Programmcode. citeturn4view2

Die TypeSafe-Homepage formuliert dasselbe als „Decisions, not strings“ und beschreibt Jev als Modell für Automation mit typisierten Entscheidungen und Confidence. citeturn4view0

Genau deshalb würde ich Jev **nicht als Ersatz für dein Antwortmodell** verwenden.

Und ich würde Jev ebenfalls **nicht als Ersatz für eine Vector DB beziehungsweise einen klassischen Retriever** betrachten.

Die ideale Position ist dazwischen.

### Jev als Query Router

Beispielzustand:

```text
QUERY:
"Warum haben wir im Twitch Bot Promptfoo rausgenommen?"

CALLER:
internal

AVAILABLE SCOPES:
deadlock.public
brain.internal
github.code
architecture
```

In einer Jev-Anfrage könnten mehrere Entscheidungen gemeinsam getroffen werden:

```text
Choice:
"What is the primary query domain?"
- game_knowledge
- internal_architecture
- source_code
- infrastructure
- streaming
- mixed

Noul:
"Does this query require internal project knowledge?"

Noul:
"Does this query require current source-code evidence?"

Noul:
"Is freshness important for answering this query?"

Score:
"How broad should retrieval be?"
```

TypeSafe dokumentiert ausdrücklich, dass verschiedene Fragetypen gemeinsam in einer Anfrage gestellt und gegen denselben State ausgewertet werden können. citeturn4view2

Der resultierende Code könnte dann entscheiden:

```python
if internal_probability > 0.75:
    corpora.add("brain.internal")

if code_probability > 0.70:
    corpora.add("github.code")

if freshness_probability > 0.80:
    retrieval.prefer_recent = True

retrieval.top_k = map_breadth_score(...)
```

Das spart einen LLM-Router-Call vollständig.

### Jev als Relevanzfilter nach dem Retrieval

Das ist wahrscheinlich **der größte Hebel für deine Tokenkosten**.

Nicht:

```text
Frage
  ↓
30 Chunks suchen
  ↓
30 vollständige Chunks an Claude/GPT/Minimax
  ↓
Modell soll selbst herausfinden,
welche 5 wichtig waren
```

sondern:

```text
Frage
  ↓
30 Chunks billig finden
  ↓
Jev bewertet Relevanz
  ↓
5–8 gute Chunks
  ↓
nur diese an finales LLM
```

Jev eignet sich dazu konzeptionell, weil `Noul` eine Aussage mit einer Wahrscheinlichkeit bewerten kann. citeturn4view2

Für beispielsweise 20 Kandidaten:

```text
STATE:

User question:
"Was wurde am Twitch AI Backend im Februar geändert?"

Candidate C01:
...

Candidate C02:
...

...

Candidate C20:
...
```

Dazu atomare Fragen:

```text
Noul C01:
"C01 contains evidence needed to answer the user's question."

Noul C02:
"C02 contains evidence needed to answer the user's question."

...

Noul C20:
"C20 contains evidence needed to answer the user's question."
```

Danach:

```python
selected = [
    candidate
    for candidate in candidates
    if candidate.jev_relevance >= threshold
]

selected = sorted(
    selected,
    key=lambda x: x.jev_relevance,
    reverse=True,
)[:MAX_EVIDENCE]
```

Die Schwelle würde ich **nicht blind auf 0,7 festschreiben**. Sie muss anhand echter Deadlock-Fragen kalibriert werden.

### Jev sollte nicht die eigentliche Suche ersetzen

Das ist eine wichtige Abgrenzung.

Jev ist laut TypeSafe ein Entscheidungsmodell, das Fragen über einen gegebenen State beantwortet. citeturn4view2turn4view0

Es ist deshalb nicht der richtige Ersatz für:

```text
embedding generation
ANN/vector search
BM25
keyword search
metadata filtering
document indexing
```

Der beste Stack ist:

```text
             Recall
               │
               ▼
     BM25 + Vector Retrieval
          top 20–50
               │
               ▼
           Precision
               │
               ▼
       Jev Relevance Gate
           top 5–8
               │
               ▼
           Synthesis
               │
               ▼
          großes LLM
```

Das beantwortet auch deine Idee mit der „Stellenfindung“ präziser:

**Jev sollte nicht Millionen Dokumentstellen selbst durchsuchen. Der Retriever findet zunächst günstig Kandidaten; Jev entscheidet anschließend, welche davon wirklich zur Frage gehören.**

### Noch besser: kleinere semantische Chunks

Wenn du wirklich möglichst genaue „Stellen“ statt ganzer Seiten möchtest, sollte die Ingestion kleinere semantische Einheiten erzeugen:

```text
Dokument
  ↓
Heading Sections
  ↓
Absätze / semantische Chunks
  ↓
150–400 Token pro Chunk
```

Dann muss Jev keine Textspanne erzeugen. Es muss lediglich entscheiden:

```text
Chunk relevant?
0.94

Chunk relevant?
0.08

Chunk relevant?
0.81
```

Das passt deutlich besser zu Jevs Modellcharakter als „extrahiere mir aus diesem 5.000-Token-Dokument die richtigen Sätze“.

### Jev als Answerability Gate

Vor dem finalen LLM würde ich noch eine zweite Entscheidung einführen:

```text
STATE:
query + selected evidence

Noul:
"The available evidence is sufficient
to answer the user's factual question
without guessing."
```

Dann:

```text
hohe Wahrscheinlichkeit
        │
        ▼
Final LLM

niedrige Wahrscheinlichkeit
        │
        ├── zweiter Retrieval-Pass
        ├── breiterer Scope
        └── ansonsten "nicht genug Evidenz"
```

Damit bekommt dein teures Antwortmodell nicht mehr automatisch jede Frage.

### Jev als Model Router

Auch das ist interessant.

Nicht jede Anfrage benötigt dasselbe LLM:

```text
"Was kostet Extra Charge?"
       ↓
kleines/schnelles Modell

"Vergleiche zwei komplexe Builds
für diese konkrete Spielsituation"
       ↓
stärkeres Modell

"Warum wurde unsere Architektur
im Februar umgestellt?"
       ↓
starkes Modell + internes RAG
```

Jev könnte beispielsweise Entscheidungen liefern zu:

```text
complexity
requires_reasoning
requires_code_analysis
requires_long_answer
requires_current_data
```

Normale Anwendungscode-Regeln wählen daraus das Modell.

Das ist günstiger und kontrollierbarer als ein vorgeschaltetes generatives LLM, das lediglich `"complex"` zurückschreiben soll.

### Jev auch nach der Antwort verwenden

Optional gibt es noch einen sinnvollen Post-Answer-Schritt.

```text
answer
+
evidence
   │
   ▼
Jev
   │
   ├── "Is the answer supported?"
   ├── "Does the answer introduce unsupported claims?"
   └── "Does the evidence actually answer the question?"
```

Auch hier soll Jev **nicht die Antwort umschreiben**. Es ist lediglich ein Gate.

Das passt zur TypeSafe-Empfehlung, Jev für fokussierte, strukturierte Entscheidungen einzusetzen und komplexere Abläufe im Anwendungscode zu komponieren. citeturn4view2

### Was ich Jev ausdrücklich nicht geben würde

Jev sollte nicht zuständig sein für:

| Aufgabe | Besser |
|---|---|
| finale natürliche Antwort | LLM |
| längere Zusammenfassung | LLM |
| Embeddings | Embedding-Modell |
| Erstsuche über Millionen Chunks | Search/Vector DB |
| exakte Berechnungen | Code |
| Permissions | Code |
| Authentifizierung | Code |
| Source-of-Truth-Regeln | Code/DB |
| langfristiges Knowledge Memory | Knowledge Store |
| komplexe mehrstufige Planung | Reasoning-Modell/Code |

TypeSafe selbst grenzt System-One-Aufgaben als fokussierte Entscheidungen ab und empfiehlt, komplexe Entscheidungen in kleinere atomare Fragen zu zerlegen und die Resultate anschließend in Code zusammenzuführen. citeturn4view2

### Der Kostenvorteil könnte sehr groß sein

TypeSafe nennt aktuell einen Listenpreis von **42 US-Dollar pro Milliarde Input-Tokens**, also **0,042 US-Dollar pro Million Input-Tokens**. Die Homepage veröffentlicht außerdem erhebliche Geschwindigkeits- und Kostenvorteile gegenüber LLM-basierten Workflows; diese Performancewerte würde ich allerdings ausdrücklich als **Herstellerbenchmark und nicht als für Deadlock Brain nachgewiesenes Ergebnis** behandeln. citeturn4view0

Eine reine Beispielrechnung zeigt dennoch, warum der Ansatz interessant ist.

Angenommen, dein Retriever produziert:

```text
30 Kandidaten
× 350 Token
= 10.500 Kontext-Tokens
```

Wenn alle davon im finalen LLM landen, bezahlst du beim teuren Modell für sämtliche 10.500 Tokens.

Mit Jev könntest du zunächst nur kompakte Kandidatenrepräsentationen verwenden, beispielsweise:

```text
30 Kandidaten
× 120 Token
≈ 3.600 Token

+ Query und Decisions
≈ 4.000–4.500 Jev Input-Tokens
```

Bei der derzeit von TypeSafe veröffentlichten Input-Preisgröße wäre dieser Jev-Schritt in der Größenordnung von lediglich rund **0,0002 US-Dollar je Query**. citeturn4view0

Wenn Jev anschließend sechs relevante Voll-Chunks auswählt:

```text
6 × 350
= 2.100 Token
```

würde der finale LLM-Kontext von:

```text
10.500
   ↓
 2.100
```

um rund **80 %** schrumpfen.

Das ist **keine Messung deines aktuellen Systems**, sondern eine Beispielrechnung. Genau diese Kennzahl würde ich später in einem A/B-Test messen.

Die wirkliche wirtschaftliche Idee lautet also:

> **Nicht Jev verwenden, weil Jev billig ist. Jev verwenden, um dem teuren Modell viel weniger irrelevanten Kontext schicken zu müssen.**

## Empfohlene Struktur des einzigen Deadlock-Brain-Repositories

Ich würde die bestehenden Repositories **nicht einfach als vier Root-Verzeichnisse importieren**:

```text
deadlock-brain/
  old-brain/
  old-docs/
  old-second-brain/
  old-twitch-bot/
```

Dann hättest du ein Repo, aber immer noch vier Architekturen.

Stattdessen würde ich nach Verantwortung schneiden:

```text
Deadlock-Brain/
│
├── apps/
│   ├── api/
│   │   └── zentraler HTTP/API Entry Point
│   │
│   ├── twitch-bot/
│   │   └── Twitch Events + Commands, keine AI-Logik
│   │
│   ├── mcp/
│   │   └── MCP Adapter
│   │
│   ├── docs-site/
│   │   └── nur falls Deadlock-Docs eine eigene App benötigt
│   │
│   └── worker/
│       └── ingestion, feeder, scheduled syncs
│
├── packages/
│   ├── answer-kernel/
│   │   ├── orchestrator
│   │   ├── answerability
│   │   ├── evidence
│   │   └── citations
│   │
│   ├── retrieval/
│   │   ├── lexical
│   │   ├── dense
│   │   ├── hybrid
│   │   ├── filters
│   │   └── ranking
│   │
│   ├── jev/
│   │   ├── client
│   │   ├── query_router
│   │   ├── relevance_gate
│   │   ├── answerability_gate
│   │   └── model_router
│   │
│   ├── ingestion/
│   │   ├── chunking
│   │   ├── normalization
│   │   ├── embeddings
│   │   └── provenance
│   │
│   ├── providers/
│   │   ├── llm
│   │   ├── embedding
│   │   └── search
│   │
│   ├── contracts/
│   │   ├── Query
│   │   ├── Document
│   │   ├── Chunk
│   │   ├── Evidence
│   │   └── Answer
│   │
│   └── deadlock-domain/
│       ├── builds
│       ├── entities
│       ├── analytics
│       ├── coaching
│       ├── meta
│       └── lineage
│
├── knowledge/
│   ├── public/
│   │   └── deadlock/
│   │
│   ├── internal/
│   │   ├── architecture/
│   │   ├── projects/
│   │   ├── decisions/
│   │   └── systems/
│   │
│   └── generated/
│       ├── daily/
│       ├── github/
│       └── sessions/
│
├── connectors/
│   ├── github/
│   ├── deadlock-data/
│   ├── twitch/
│   └── openclaw/
│
├── evals/
│   ├── datasets/
│   ├── retrieval/
│   ├── answers/
│   └── jev/
│
├── infra/
│   ├── docker/
│   ├── migrations/
│   ├── monitoring/
│   └── deployment/
│
└── architecture/
    ├── ADRs/
    ├── diagrams/
    └── runbooks/
```

### Wohin die vorhandenen Dinge wandern

| Heutiges Element | Neues Ziel |
|---|---|
| `Deadlock-Brain/src/deadlock_brain/*` | `packages/deadlock-domain/` bzw. passende Core-Packages |
| Brain API/Service | `apps/api/` |
| bestehendes MCP | `apps/mcp/` |
| Twitch Bot | `apps/twitch-bot/` |
| Twitch AI-/RAG-Code | **entfernen**, durch Brain API Client ersetzen |
| Deadlock Docs Inhalte | `knowledge/public/deadlock/` |
| Docs-Website | `apps/docs-site/` |
| Brain Feeder | `apps/worker/` + `packages/ingestion/` |
| Second-Brain Markdown | `knowledge/internal/` |
| automatisch generierte Second-Brain-Inhalte | `knowledge/generated/` |
| alte „Deadlock-Rag“-Logik | `packages/retrieval/` |
| OpenRouter/Modelllogik | `packages/providers/llm/` |
| neues Jev | `packages/jev/` |

Das Entscheidende daran: **Die Ordnerstruktur bildet die Architektur ab.**

### Keine Vector-DB-Artefakte in Git

Das Knowledge-Markdown darf in Git liegen.

Embeddings, Suchindizes, Cache und Laufzeitdaten würde ich dagegen als erzeugbare Artefakte behandeln:

```text
Git:
raw/curated knowledge
metadata
schemas
configs
code

Runtime storage:
chunks
embeddings
lexical index
cache
query traces
usage metrics
```

Ein sauberer Rebuild sollte immer möglich sein:

```text
git checkout
   ↓
ingest
   ↓
normalize
   ↓
chunk
   ↓
embed/index
   ↓
ready
```

### Der Twitch Bot wird bewusst langweilig

Im Zielzustand sollte die AI-Seite des Bots ungefähr nur noch das hier tun:

```javascript
const result = await brain.answer({
  query: message,
  channel: "twitch",
  scopes: ["deadlock.public"],
  responseProfile: "twitch-short",
  conversationId
});

await twitch.reply(result.answer);
```

Kein:

```text
OpenRouter Prompt X
RAG Query Y
Embedding Provider Z
Citation Validation
Model Routing
Retrieval Prompt
```

im Bot.

Dann kannst du morgen Discord, Website oder einen anderen Client ergänzen und bekommst **dieselben Antworten aus derselben Knowledge-Pipeline**.

## Migration und konkrete Anpassungen

Ich würde die Umstellung nicht als „Repo Merge zuerst“ machen. Erst die Architekturgrenzen definieren, dann verschieben.

### Zuerst den zentralen Contract einführen

Noch bevor Inhalte migriert werden, sollte `Deadlock-Brain` einen stabilen internen Vertrag bekommen:

```text
answer(query_context) -> Answer
```

Beispiel:

```python
class QueryContext:
    query: str
    channel: str
    scopes: list[str]
    locale: str
    actor_id: str | None
    conversation_id: str | None
    response_profile: str
```

und:

```python
class Answer:
    text: str
    citations: list[Citation]
    evidence: list[Evidence]
    confidence: float | None
    trace_id: str
    usage: Usage
```

Ab dann müssen alle neuen Interfaces diesen Kernel verwenden.

### Danach Twitch auf Deadlock Brain umstellen

Das wäre für mich die **erste reale Cutover-Migration**, weil du dadurch sofort einen parallelen AI-Pfad eliminierst.

Vorher:

```text
Twitch
 └── eigene AI Integration
      └── eigenes RAG
```

Nachher:

```text
Twitch
 └── Brain Client
      └── gemeinsamer Kernel
```

Erst wenn die Antworten funktional gleich oder besser sind, entfernst du den alten Code.

### Danach Brain Feeder aus Docs herausziehen

Der bestehende Gedanke des Brain Feeders ist gut. Seine Heimat ist nur falsch.

Neu:

```text
apps/worker/
    feeder/
        github.py
        openclaw.py
        docs.py
        sessions.py

packages/ingestion/
    normalize.py
    classify.py
    deduplicate.py
    chunk.py
    persist.py
```

Hier ist Jev übrigens ebenfalls interessant.

Der historische Feeder verwendet laut deinem Second-Brain-Archiv Claude Haiku zur Relevanzentscheidung.

Das ist ein **perfekter Jev-Kandidat**.

Statt:

```text
GitHub Event
    ↓
Claude Haiku
"ist das wichtig fürs Brain?"
    ↓
yes/no + ggf. prompt parsing
```

könnte es heißen:

```text
GitHub Event
    ↓
Jev
    │
    ├── relevant_to_long_term_memory?
    ├── category?
    ├── confidence?
    ├── contains_architectural_decision?
    └── contains_sensitive_information?
```

TypeSafe hat Jev ausdrücklich für solche strukturierten Softwareentscheidungen konzipiert. citeturn4view2turn4view0

Für das **eigentliche Verfassen einer neuen Markdown-Zusammenfassung** würdest du dann weiterhin ein generatives Modell verwenden.

Damit wird:

```text
100 Events
  ↓
Jev
  ↓
vielleicht 12 relevante
  ↓
LLM nur für diese 12
```

statt:

```text
100 Events
  ↓
LLM 100x
```

Das könnte neben Retrieval sogar einer der wertvollsten Jev-Anwendungsfälle in deinem ganzen System sein.

### Danach Second Brain importieren

Die bestehenden Markdown-Dateien würde ich nicht neu schreiben.

Sie sind wertvolle Historie.

Zum Beispiel:

```text
Deadlock-2nd-Brain/systeme/*
    → knowledge/internal/systems/

Deadlock-2nd-Brain/projekte/*
    → knowledge/internal/projects/

Deadlock-2nd-Brain/entscheidungen/*
    → knowledge/internal/decisions/

Deadlock-2nd-Brain/daily/*
    → knowledge/generated/daily/
```

Dabei würde ich möglichst die Git-Historie erhalten, statt nur den letzten Stand zu kopieren.

### Danach Deadlock-Docs auflösen

Hier muss zwischen **Content** und **Software** unterschieden werden:

```text
Content
  → knowledge/public/deadlock/

Frontend/Website
  → apps/docs-site/

Ingestion/Feeder
  → apps/worker/

AI/RAG-Komponenten
  → packages/*
```

Anschließend sollte `Deadlock-Docs` nur noch archiviert beziehungsweise read-only sein.

### Erst danach Jev aktiv in den Hauptpfad setzen

Nicht unmittelbar Jev in Produktion schalten.

Zuerst Shadow Mode:

```text
bestehende Retrieval-Pipeline
             │
             ├──────────────► aktuelle Antwort
             │
             └──► Jev Entscheidungen
                    │
                    └── nur loggen
```

Dann kannst du messen:

```text
Welchen Scope hätte Jev gewählt?
Welche Chunks hätte Jev entfernt?
Hätte es relevante Chunks verworfen?
Welches Modell hätte es ausgewählt?
Wie zuverlässig ist die Confidence?
```

Erst danach beeinflusst Jev echte Antworten.

Das ist insbesondere bei einem Relevanzfilter wichtig: Ein False Positive kostet etwas zusätzlichen Kontext. Ein False Negative kann genau die Passage entfernen, die das LLM gebraucht hätte.

**Beim Retrieval würde ich Jev anfangs deshalb auf Recall optimieren, nicht auf maximale Aggressivität.**

### Dann die alten Repositories einfrieren

Erst wenn:

```text
Twitch → Brain
Docs → Brain
Feeder → Brain
Second Brain → Brain
RAG → Brain
```

wirklich abgeschlossen ist:

```text
Deadlock-Docs        archived
Deadlock-2nd-Brain   archived
Deadlock-Twitch-Bot  archived
```

README dort jeweils nur noch sinngemäß:

```text
This repository is archived.
Development moved to Deadlock-Brain.
```

Ab diesem Moment gibt es tatsächlich nur noch eine Architekturquelle.

## Was ich messen würde und wo noch Unsicherheit besteht

Die Konsolidierung ist nur erfolgreich, wenn du danach nicht lediglich „weniger Repositories“ hast, sondern bessere Antworten für weniger Aufwand.

Ich würde deshalb ein festes Eval-Set im Repository aufbauen.

Nicht nur generische QA-Fragen, sondern vier Klassen:

| Klasse | Beispiel |
|---|---|
| Public Deadlock | „Was macht Extra Charge?“ |
| komplexe Deadlock-Frage | „Welches Build passt gegen diese Composition?“ |
| internes Wissen | „Warum wurde Backend X entfernt?“ |
| Code/Operations | „Wo wird derzeit Provider Y konfiguriert?“ |

Für jede Frage speicherst du:

```yaml
question: "..."
allowed_scopes:
  - brain.internal

expected_sources:
  - "knowledge/internal/systems/twitch-bot.md"

must_contain:
  - "..."

must_not_use:
  - "..."

answerable: true
```

Dann werden nicht nur die Antworten bewertet, sondern die gesamte Pipeline:

```text
Retrieval Recall@K
        ↓
Jev Recall / Precision
        ↓
Evidence Size
        ↓
Final Answer Quality
        ↓
Citation Correctness
        ↓
Input Tokens
        ↓
Cost
        ↓
p50 / p95 Latency
```

Für Jev ist besonders diese Matrix relevant:

```text
                    tatsächlich relevant
                  ja                  nein

Jev ja        true positive      false positive
             gut                 nur etwas teurer

Jev nein      false negative     true negative
             GEFÄHRLICH          ideal
```

Beim Tuning sollte die Reihenfolge daher sein:

```text
1. möglichst keine relevanten Stellen verlieren
2. anschließend Kontext stärker reduzieren
3. erst danach maximale Kostenoptimierung
```

### Die wichtigsten Observability-Daten

Jeder Request sollte einen `trace_id` bekommen und ungefähr dieses Trace erzeugen:

```text
query
├── auth scopes
├── Jev routing
│   ├── choices
│   └── confidence
├── retrieval
│   ├── candidates: 32
│   └── retrieval scores
├── Jev filtering
│   ├── kept: 7
│   └── relevance probabilities
├── final model
│   ├── model
│   ├── input tokens
│   ├── output tokens
│   └── latency
└── result
    ├── citations
    └── total cost
```

Dann kannst du nach einem Monat wirklich beantworten:

> „Bringt Jev etwas?“

statt nur das Gefühl zu haben, dass es schneller sein müsste.

### Mein Prioritätenranking für Jev

Für dein konkretes System würde ich Jev in dieser Reihenfolge einführen:

**Sehr hoher Nutzen:** Relevanzfilter für bereits gefundene RAG-Chunks.

**Sehr hoher Nutzen:** Classification/Gating im Brain Feeder, damit generative Modelle nur relevante Repository-/Session-Ereignisse zusammenfassen.

**Hoher Nutzen:** Query-Routing zwischen `deadlock.public`, `brain.internal`, Code, Infrastruktur und anderen Corpora.

**Hoher Nutzen:** Answerability Gate vor dem finalen LLM.

**Mittlerer Nutzen:** Routing zwischen kleinem und großem Antwortmodell.

**Später interessant:** Post-Answer-Support-/Citation-Gate.

**Nicht sinnvoll:** Jev als Antwortmodell, Embedding-Modell oder alleinige Suchmaschine.

Diese Gewichtung folgt unmittelbar aus Jevs dokumentiertem Design als Modell für typisierte, fokussierte Entscheidungen anstelle freier Textgenerierung. citeturn4view2turn4view0

### Was ich aus dem jetzigen Stand nicht seriös behaupten würde

Bei der Untersuchung gibt es eine relevante Grenze: Der aktuelle `Deadlock-Brain`-Stand ließ sich auf Repository- und Paketebene einsehen, und das bereitgestellte `Deadlock-2nd-Brain`-Archiv liefert sehr gute Architekturdokumentation. Der komplette aktuelle Laufzeitpfad aller vier Repositories konnte in dieser Untersuchung jedoch **nicht bis zu jedem einzelnen Provider-Aufruf und Endpoint auf Codezeilenebene verifiziert werden**.

Insbesondere würde ich deshalb momentan nicht behaupten:

```text
"Deadlock-Rag ist definitiv noch ein eigener laufender Service"
```

oder:

```text
"der aktuelle Twitch Bot ruft heute garantiert noch exakt
denselben OpenRouter-Pfad auf wie in der Second-Brain-Dokumentation"
```

Die Second-Brain-Dateien dokumentieren diesen Zustand beziehungsweise die Architekturhistorie; sie können gegenüber dem heutigen Runtime-Code veraltet sein.

Auch den genauen aktuellen Retriever von `Deadlock-Brain` — beispielsweise welches Dense-Embedding-Modell, welche Vector-DB, welche konkrete Hybrid-Fusion und welches heutige `top_k` — würde ich auf Basis des hier verifizierten Materials nicht erfinden.

Diese Unsicherheit ändert allerdings **nicht** die zentrale Architekturentscheidung. Unabhängig davon, ob der heutige Retriever bereits sehr gut oder nur mittelmäßig ist, sollte die Sollstruktur gleich aussehen:

```text
                         DEADLOCK BRAIN

Consumers
Twitch │ Web │ MCP │ Agent │ CLI │ Internal
                  │
                  ▼
         ┌─────────────────┐
         │ Unified Query API│
         └────────┬────────┘
                  │
          Auth + hard scopes
                  │
                  ▼
         ┌─────────────────┐
         │       Jev       │
         │ route / classify│
         └────────┬────────┘
                  │
                  ▼
      ┌────────────────────────┐
      │ Retrieval / Deadlock RAG│
      │ lexical + dense + meta │
      └───────────┬────────────┘
                  │
            candidates
                  │
                  ▼
         ┌─────────────────┐
         │       Jev       │
         │ relevance filter│
         └────────┬────────┘
                  │
             evidence
                  │
           ┌──────▼──────┐
           │ Jev suffices?│
           └──────┬──────┘
                  │
                  ▼
         Generatives Modell
                  │
                  ▼
        citations + response
                  │
          format by channel
```

**Das wäre für mich das eigentliche „Deadlock Brain 2.0“: nicht nur ein größeres Repository, sondern ein einziges Nervensystem.**

`Deadlock-Docs`, `Deadlock-2nd-Brain` und `Deadlock-Twitch-Bot` verlieren damit ihre Rolle als eigenständige AI-Systeme. Ihre nützlichen Bestandteile verschwinden nicht; sie werden sauber in `Deadlock-Brain` integriert. Das bestehende Domain-Know-how des Brain bleibt der Kern, das Second Brain wird sein internes Langzeitwissen, Docs werden Knowledge Sources, Twitch wird ein Adapter, RAG wird eine interne Retrieval-Capability, und Jev sitzt genau dort, wo heute wahrscheinlich unnötig viele Tokens verbrannt werden: **zwischen einer eingehenden Frage beziehungsweise großen Kandidatenmenge und dem teuren Modell, das am Ende tatsächlich formulieren und denken muss.**