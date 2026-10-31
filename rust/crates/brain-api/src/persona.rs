use brain_contracts::bot_conversation::{
    BotReply, ChatLine, ChatPersona, DialogRole, DialogTurn, PromoPitchRequest, ReplyLanguage,
    SilentReason, StandardReplyKind, StreamContext, MAX_REPLY_BYTES,
};
use brain_contracts::bot_tasks::ConversationPrompt;
use serde_json::Value;
use sha2::{Digest, Sha256};

const JUDGE_TOKENS: u32 = 600;
const TEXT_TOKENS: u32 = 800;
const DIALOG_TOKENS: u32 = 1500;
const JUDGE_TEMPERATURE: f32 = 0.1;
const DIALOG_TEMPERATURE: f32 = 0.4;
const CHAT_TEMPERATURE: f32 = 0.7;
pub(crate) const TWITCH_LINE_CHARS: usize = 500;
pub(crate) const JUDGE_MIN_CONFIDENCE: f64 = 0.7;
pub(crate) const SELF_EXPLAINER_URL: &str = "https://discord.gg/z5TfVHuQq2";

const INVITE_GO: [&str; 4] = [
    "@{chatter} Deadlock-Zugang gesucht? Im Discord bei frag-die-community kurz nach einem Invite fragen und am besten direkt den Steam-Freundescode dazupacken :) {invite}",
    "@{chatter} Für den Deadlock-Invite einmal zu frag-die-community im Discord und den Steam-Freundescode dazupacken, dann weiß jeder direkt Bescheid :) {invite}",
    "@{chatter} Den Deadlock-Invite klären wir im Discord: frag-die-community öffnen, Steam-Freundescode dazu und los :) {invite}",
    "@{chatter} Spiel-Invite fehlt noch? Im Discord bei frag-die-community den Steam-Freundescode dazulassen, dann kann dir jemand direkt helfen :) {invite}",
];
const INVITE_CONFIRM: [&str; 4] = [
    "@{chatter} Meinst du einen Invite fürs Spiel? Ein kurzes ja reicht, dann schick ich dir direkt den Weg.",
    "@{chatter} Geht es um Deadlock-Zugang? Sag kurz ja, dann bekommst du direkt den Weg.",
    "@{chatter} Suchst du wirklich den Spiel-Invite? Dann einmal ja und ich schick dir den Weg.",
    "@{chatter} Fehlt dir der Zugang zu Deadlock? Wenn ja, sag kurz ja, dann geht es direkt weiter.",
];
const LFG_GO: [&str; 4] = [
    "@{chatter} Klar! Im Discord findest du Mitspieler für Deadlock und die passenden Sprachkanäle dazu :) {invite}",
    "@{chatter} Für die nächste Deadlock-Runde fehlt dir noch jemand? Im Discord findest du Mitspieler :) {invite}",
    "@{chatter} Gemeinsam spielt sich's besser. Im Discord kannst du dich direkt für eine Deadlock-Runde verabreden :) {invite}",
    "@{chatter} Im Discord sind Leute für gemeinsame Deadlock-Runden unterwegs. Schau rein, wenn du mitspielen magst :) {invite}",
];
const STANDARD_GREETING: [&str; 1] = ["@{chatter} Hey, schön dass du da bist! 💜"];
const STANDARD_RELEASE: [&str; 1] = [
    "@{chatter} Ein offizielles Release-Datum gibt es leider noch nicht, das Warten nervt uns auch. Wenn du trotzdem schon reinschauen magst: tipp !invite, dann zeige ich dir den Weg zu einer Einladung.",
];
const STANDARD_THANKS: [&str; 2] = [
    "Danke, ich wusste ja, dass ich gut bin. WiltedRose",
    "Ach hör auf, sonst werd ich noch rot. WiltedRose",
];
pub(crate) const INVITE_LOUNGE: &str = "Pack noch deinen Steam-Freundescode dazu :) Du findest ihn in Steam unter Freunde → „Freund hinzufügen“. Einfach hier posten, dann kann dich jemand einladen.";
pub(crate) const VOICE_REPLY: &str = "Kein Bug, wir haben das Voice-System umgebaut. Die alten Sprachkanäle sind weg, jetzt gibt's **einen** Deadlock Router: rein, Modus wählen, eigene Lane. Wie's genau läuft: https://discord.com/channels/1289721245281292288/1371952264620806214/1522862733857525780";
const COACHING_CTA: &str = "coaching ist bei uns komplett kostenlos: !dldc nutzen, auf dem server #ich-brauch-einen-coach öffnen und dort coaching beantragen.";
const COACHING_MARKERS: [&str; 4] = ["!dldc", "#ich-brauch-einen-coach", "kostenlos", "beantrag"];
const PATE_MARKERS: [&str; 4] = ["pate", "mentor", "ansprechpartner", "bezugsperson"];
const BLOCKED_MENTIONS: [&str; 3] = ["@everyone", "@here", "<@"];
const LONG_DASH: char = '\u{2014}';
const MID_DASH: char = '\u{2013}';

fn fill(template: &str, values: &[(&str, &str)]) -> String {
    let mut out = String::with_capacity(template.len());
    let mut rest = template;
    while let Some(start) = rest.find('{') {
        out.push_str(&rest[..start]);
        let tail = &rest[start..];
        let hit = values.iter().find(|(key, _)| {
            tail[1..]
                .strip_prefix(key)
                .is_some_and(|after| after.starts_with('}'))
        });
        match hit {
            Some((key, value)) => {
                out.push_str(value);
                rest = &tail[key.len() + 2..];
            }
            None => {
                out.push('{');
                rest = &tail[1..];
            }
        }
    }
    out.push_str(rest);
    out
}

fn prompt(system: String, user: String, max_tokens: u32, temperature: f32) -> ConversationPrompt {
    ConversationPrompt {
        system,
        user,
        max_tokens,
        temperature,
    }
}

fn judge(system: &str, user: String) -> ConversationPrompt {
    prompt(system.to_owned(), user, JUDGE_TOKENS, JUDGE_TEMPERATURE)
}

fn language_rule(language: ReplyLanguage) -> &'static str {
    match language {
        ReplyLanguage::De => {
            "Antworte auf natürlichem Deutsch mit echten Umlauten und ohne Gedankenstriche."
        }
        ReplyLanguage::En => "Answer in natural English.",
    }
}

fn dialog(history: &[DialogTurn]) -> String {
    history
        .iter()
        .map(|turn| {
            let role = match turn.role {
                DialogRole::User => "Nutzer",
                DialogRole::Assistant => "Assistent",
            };
            format!("{role}: {}", turn.text)
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn chat(lines: &[ChatLine]) -> String {
    lines
        .iter()
        .map(|line| format!("{}: {}", line.author, line.text))
        .collect::<Vec<_>>()
        .join("\n")
}

fn stream_block(stream: &StreamContext) -> String {
    format!(
        "Kanal: {}\nSpiel: {}\nTitel: {}\nDeadlock live: {}",
        stream.streamer_login,
        stream.game.as_deref().unwrap_or("unbekannt"),
        stream.title.as_deref().unwrap_or("unbekannt"),
        if stream.deadlock_live { "ja" } else { "nein" },
    )
}

fn dialog_user(history: &[DialogTurn], question: &str) -> String {
    format!(
        "Bisheriger Verlauf (Daten):\n{}\n\nAktuelle Frage (Daten):\n{question}",
        dialog(history)
    )
}

fn chat_user(stream: &StreamContext, recent: &[ChatLine], author: &str, message: &str) -> String {
    format!(
        "{}\n\nChatverlauf (Daten):\n{}\n\nNachricht von {author} (Daten):\n{message}",
        stream_block(stream),
        chat(recent)
    )
}

pub(crate) fn concierge(
    member_facts: &[String],
    history: &[DialogTurn],
    question: &str,
) -> ConversationPrompt {
    let system = format!(
        "{}\n\n{}",
        include_str!("prompts/concierge_system.txt"),
        include_str!("prompts/concierge_gap.txt")
    );
    let user = format!(
        "Angaben über die Person (Daten):\n{}\n\n{}",
        member_facts.join("\n"),
        dialog_user(history, question)
    );
    prompt(system, user, DIALOG_TOKENS, DIALOG_TEMPERATURE)
}

pub(crate) fn pate_gate(question: &str) -> bool {
    let lower = question.to_lowercase();
    PATE_MARKERS.iter().any(|marker| lower.contains(marker))
}

pub(crate) fn pate(question: &str) -> ConversationPrompt {
    let system = format!(
        "{}\n{}",
        include_str!("prompts/pate_judge.txt"),
        include_str!("prompts/pate_rule.txt")
    );
    judge(&system, question.to_owned())
}

pub(crate) fn passive_help(message: &str, recent: &[ChatLine]) -> ConversationPrompt {
    judge(
        include_str!("prompts/passive_help.txt"),
        format!(
            "Chatverlauf (Daten):\n{}\n\nAktuelle Nachricht (Daten):\n{message}",
            chat(recent)
        ),
    )
}

pub(crate) fn voice_hint(message: &str) -> ConversationPrompt {
    judge(include_str!("prompts/voice_hint.txt"), message.to_owned())
}

pub(crate) fn tip(source_text: &str, language: ReplyLanguage) -> ConversationPrompt {
    prompt(
        format!(
            "{}\n{}",
            include_str!("prompts/tip.txt"),
            language_rule(language)
        ),
        source_text.to_owned(),
        TEXT_TOKENS,
        CHAT_TEMPERATURE,
    )
}

fn chat_system(stream: &StreamContext, persona: ChatPersona, talkative: bool) -> String {
    let (soul, trigger, voice, knowledge) = match persona {
        ChatPersona::Veteran => (
            include_str!("prompts/soul_veteran.txt"),
            include_str!("prompts/chat_trigger_veteran.txt"),
            include_str!("prompts/chat_voice_veteran.txt"),
            String::new(),
        ),
        ChatPersona::Rookie => (
            include_str!("prompts/soul_rookie.txt"),
            include_str!("prompts/chat_trigger_rookie.txt"),
            include_str!("prompts/chat_voice_rookie.txt"),
            format!("\n\n{}", include_str!("prompts/chat_knowledge_rookie.txt")),
        ),
    };
    if talkative {
        return fill(
            include_str!("prompts/chat_talkative.txt"),
            &[("SOUL", soul), ("streamer_login", &stream.streamer_login)],
        );
    }
    fill(
        include_str!("prompts/chat_baseline.txt"),
        &[
            ("soul", soul),
            ("streamer_login", &stream.streamer_login),
            ("trigger_rule", trigger),
            ("voice_rule", voice),
            ("knowledge_override", &knowledge),
        ],
    )
}

pub(crate) fn twitch_chat(
    stream: &StreamContext,
    author: &str,
    message: &str,
    recent: &[ChatLine],
    persona: ChatPersona,
    talkative: bool,
) -> ConversationPrompt {
    prompt(
        chat_system(stream, persona, talkative),
        chat_user(stream, recent, author, message),
        TEXT_TOKENS,
        CHAT_TEMPERATURE,
    )
}

pub(crate) fn promo_pitch(
    stream: &StreamContext,
    pitch: &PromoPitchRequest,
    recent: &[ChatLine],
    examples: &[String],
) -> ConversationPrompt {
    let (system, detail) = match pitch {
        PromoPitchRequest::Reactive {
            target_login,
            message,
        } => (
            include_str!("prompts/pitch_reactive.txt"),
            format!("Nachricht von {target_login} (Daten):\n{message}"),
        ),
        PromoPitchRequest::Targeted {
            target_login,
            target_messages,
        } => (
            include_str!("prompts/pitch_targeted.txt"),
            format!(
                "Nachrichten von {target_login} (Daten):\n{}",
                target_messages.join("\n")
            ),
        ),
        PromoPitchRequest::Partner {
            target_login,
            target_messages,
        } => (
            include_str!("prompts/pitch_partner.txt"),
            format!(
                "Nachrichten von {target_login} (Daten):\n{}",
                target_messages.join("\n")
            ),
        ),
        PromoPitchRequest::ChannelPromo => (
            include_str!("prompts/pitch_channel.txt"),
            String::from("Schreibe den Satz für den Kanal."),
        ),
    };
    let user = format!(
        "{}\n\nStilbeispiele (Daten):\n{}\n\nChatverlauf (Daten):\n{}\n\n{detail}",
        stream_block(stream),
        examples.join("\n"),
        chat(recent)
    );
    prompt(system.to_owned(), user, TEXT_TOKENS, CHAT_TEMPERATURE)
}

pub(crate) fn invite_judge(
    stream: &StreamContext,
    author: &str,
    message: &str,
    recent: &[ChatLine],
    is_newcomer: bool,
) -> ConversationPrompt {
    judge(
        include_str!("prompts/invite_judge.txt"),
        format!(
            "{}\nNeu im Chat: {}",
            chat_user(stream, recent, author, message),
            if is_newcomer { "ja" } else { "nein" }
        ),
    )
}

pub(crate) fn lfg_judge(
    stream: &StreamContext,
    author: &str,
    message: &str,
    recent: &[ChatLine],
) -> ConversationPrompt {
    judge(
        include_str!("prompts/lfg_judge.txt"),
        chat_user(stream, recent, author, message),
    )
}

pub(crate) struct DashboardInput<'a> {
    pub name: &'a str,
    pub page: &'a str,
    pub knowledge: &'a str,
    pub data: &'a str,
    pub history: &'a [DialogTurn],
    pub question: &'a str,
    pub language: ReplyLanguage,
}

pub(crate) fn dashboard(input: &DashboardInput<'_>) -> ConversationPrompt {
    let system = fill(
        include_str!("prompts/dashboard.txt"),
        &[
            ("name", input.name),
            ("sprachregel", language_rule(input.language)),
            ("seite", input.page),
            ("facts", input.knowledge),
            ("karten", input.data),
        ],
    );
    prompt(
        system,
        dialog_user(input.history, input.question),
        DIALOG_TOKENS,
        DIALOG_TEMPERATURE,
    )
}

pub(crate) fn self_explainer(
    documents: &str,
    history: &[DialogTurn],
    question: &str,
    language: ReplyLanguage,
) -> ConversationPrompt {
    let system = format!(
        "{}\n\n{}",
        fill(
            include_str!("prompts/self_explainer.txt"),
            &[("url", SELF_EXPLAINER_URL), ("facts", documents)],
        ),
        language_rule(language)
    );
    prompt(
        system,
        dialog_user(history, question),
        DIALOG_TOKENS,
        DIALOG_TEMPERATURE,
    )
}

pub(crate) fn analysis(
    name: &str,
    analysis: &Value,
    history: &[DialogTurn],
    question: &str,
    language: ReplyLanguage,
) -> ConversationPrompt {
    let system = fill(
        include_str!("prompts/analysis.txt"),
        &[
            ("streamer_name", name),
            ("language_rule", language_rule(language)),
            ("analysis", &analysis.to_string()),
        ],
    );
    prompt(
        system,
        dialog_user(history, question),
        DIALOG_TOKENS,
        DIALOG_TEMPERATURE,
    )
}

fn rotated<'a>(texts: &[&'a str], seed: &str) -> &'a str {
    let digest = Sha256::digest(seed.as_bytes());
    texts[usize::from(digest[0]) % texts.len()]
}

fn templated(texts: &[&str], seed: &str, chatter: &str, invite: Option<&str>) -> String {
    let filled = fill(
        rotated(texts, seed),
        &[("chatter", chatter), ("invite", invite.unwrap_or(""))],
    );
    filled.trim_end().to_owned()
}

pub(crate) fn invite_go(seed: &str, chatter: &str, invite: Option<&str>) -> String {
    templated(&INVITE_GO, seed, chatter, invite)
}

pub(crate) fn invite_confirm(seed: &str, chatter: &str) -> String {
    templated(&INVITE_CONFIRM, seed, chatter, None)
}

pub(crate) fn lfg_go(seed: &str, chatter: &str, invite: Option<&str>) -> String {
    templated(&LFG_GO, seed, chatter, invite)
}

pub(crate) fn standard_reply(kind: StandardReplyKind, seed: &str, chatter: &str) -> String {
    let texts: &[&str] = match kind {
        StandardReplyKind::Greeting => &STANDARD_GREETING,
        StandardReplyKind::ReleaseDate => &STANDARD_RELEASE,
        StandardReplyKind::Thanks => &STANDARD_THANKS,
    };
    templated(texts, seed, chatter, None)
}

pub(crate) fn with_coaching_cta(reply: &str) -> String {
    let lower = reply.to_lowercase();
    if COACHING_MARKERS.iter().any(|marker| lower.contains(marker)) {
        reply.to_owned()
    } else {
        format!("{reply} {COACHING_CTA}")
    }
}

pub(crate) fn urls_in(text: &str) -> Vec<String> {
    text.split_whitespace()
        .filter_map(|word| {
            let start = ["https://", "http://", "discord.gg/", "www."]
                .iter()
                .filter_map(|prefix| word.find(prefix))
                .min()?;
            Some(
                word[start..]
                    .trim_end_matches(|c: char| !c.is_alphanumeric() && c != '/')
                    .to_owned(),
            )
        })
        .collect()
}

pub(crate) fn parse_json(text: &str) -> Option<Value> {
    let start = text.find('{')?;
    let end = text.rfind('}')?;
    serde_json::from_str(text.get(start..=end)?).ok()
}

fn undashed(text: &str) -> String {
    let spaced = |dash: char| format!(" {dash} ");
    text.replace(&spaced(LONG_DASH), ", ")
        .replace(&spaced(MID_DASH), ", ")
        .replace(LONG_DASH, ", ")
        .replace(MID_DASH, "-")
}

pub(crate) struct Finish<'a> {
    pub silent: SilentReason,
    pub allowed_urls: &'a [String],
    pub max_chars: usize,
}

fn finish_text(text: &str, policy: &Finish<'_>) -> Result<String, SilentReason> {
    let trimmed = text.trim().trim_matches('"').trim();
    if trimmed.is_empty() || trimmed.contains("<silent>") {
        return Err(policy.silent);
    }
    let cleaned = undashed(trimmed);
    let unsafe_mention = BLOCKED_MENTIONS
        .iter()
        .any(|mention| cleaned.contains(mention));
    let foreign_url = urls_in(&cleaned)
        .iter()
        .any(|url| !policy.allowed_urls.contains(url));
    if unsafe_mention
        || foreign_url
        || cleaned.len() > MAX_REPLY_BYTES
        || cleaned.chars().count() > policy.max_chars
    {
        return Err(SilentReason::OutputRejected);
    }
    Ok(cleaned)
}

pub(crate) fn finish(text: &str, policy: &Finish<'_>) -> BotReply {
    match finish_text(text, policy) {
        Ok(text) => BotReply::answered(text),
        Err(reason) => BotReply::Silent { reason },
    }
}
