use brain_contracts::{
    answer_contract::AnswerViolation, AnswerProviderPort, AuthorizedContext, Principal, Query,
    RetrievalPort,
};
use serde_json::json;

#[test]
#[ignore = "explicit synthetic public-source model compliance probe using Infisical-exported variables"]
fn live_structured_answer_compliance() {
    let config_path = std::env::var("K2_SERVE_CONFIG").expect("K2_SERVE_CONFIG required");
    let mut config = crate::Config::parse(&std::fs::read(config_path).expect("config unavailable"))
        .expect("config invalid");
    config
        .credentials
        .retain(|grant| grant.actor_id == "dl-bot" && grant.channel == "discord");
    for grant in &mut config.credentials {
        grant.entity_profile_model_context = false;
    }
    config.discord_live = None;
    config.deadlock_api = None;
    config.task_providers = None;
    config.analytics = None;
    config.internal_operator = None;
    config.entity_profile_maintenance_config = None;
    config.trusted_discord_consumers.clear();
    config.answer_artifact_consumers.clear();
    config.bot_task_consumers.clear();
    let grant = config
        .credentials
        .first()
        .expect("public bot grant missing")
        .clone();
    let secrets = crate::Secrets::load(&config, |name| std::env::var(name).ok())
        .expect("Infisical-exported variables required");
    let prepared = crate::Prepared::new(config, secrets).expect("runtime composition failed");
    let (reader, provider) = prepared.probe_components();
    let retrieval =
        dbrain_retrieval::ReleaseRetriever::new(reader, prepared.config.retrieval.limit);
    let mut answered = 0;
    let mut social_answered = 0;
    let mut format_silence = 0;
    let mut value_silence = 0;
    let mut evidence_silence = 0;
    let mut provider_errors = 0;
    let cases = [
        ("social", "wie gehts dir"),
        ("thanks", "Danke für die Hilfe!"),
        ("greeting", "Hallo!"),
        ("goodbye", "Bis später!"),
        ("health", "Wie viele Lebenspunkte hat Abrams?"),
        ("speed", "Wie schnell läuft Warden?"),
        ("stamina", "Wie viel Stamina hat Haze?"),
        ("damage", "Wie viel Bullet Damage hat Infernus?"),
        ("comparison", "Vergleich Haze und Infernus bei Lebenspunkten."),
        ("cost", "Wie viele Souls kostet Monster Rounds?"),
        ("cooldown", "Wie lang ist der Cooldown von Warden Alchemical Flask?"),
        ("timer", "Wann erscheint Mid Boss?"),
        ("patch_date", "Von wann sind die neuesten Patchnotes?"),
        ("patch_changes", "Was hat sich für Warden im letzten Patch geändert?"),
        ("coaching", "Ich sterbe oft in der Lane, wie spiele ich sicherer?"),
        ("positioning", "Wie nutze ich Deckung besser?"),
        ("aim", "Wie kann ich mein Aim verbessern?"),
        ("teamplay", "Wie kommuniziere ich Rotationen mit meinem Team?"),
        ("uncertainty", "Hast du einen Tipp gegen einen stärkeren Gegner, auch wenn du dir nicht ganz sicher bist?"),
        ("community", "Wie finde ich jemanden zum gemeinsamen Spielen?"),
    ];
    for (index, (category, text)) in cases.iter().enumerate() {
        let query: Query = serde_json::from_value(json!({
            "request_id":format!("synthetic-k2-compliance-{index}"),
            "conversation_id":"synthetic-k2-compliance",
            "text":text,"profile":if (14..=18).contains(&index) { "coaching" } else { "explain" },
            "requested_scopes":["bot.public"]
        }))
        .expect("query invalid");
        let context = AuthorizedContext {
            principal: Principal {
                actor_id: grant.actor_id.clone(),
                channel: grant.channel.clone(),
                scopes: grant.scopes.clone(),
                provider_egress: grant.provider_egress.clone(),
            },
            conversation_id: query.conversation_id.clone(),
            knowledge_release: grant
                .release
                .as_ref()
                .unwrap_or(&prepared.config.release)
                .id
                .clone(),
            deadline_ms: prepared.config.timeouts.request_ms,
            budget: (&prepared.config.budgets).into(),
            discord: None,
            request_deadline: None,
        }
        .with_request_deadline()
        .into_owned();
        let evidence: Vec<_> = retrieval
            .retrieve(&query, &context)
            .expect("public retrieval failed")
            .into_iter()
            .filter(|item| {
                item.visibility == brain_contracts::SourceVisibility::Public
                    && (brain_contracts::answer_contract::is_game_source(item)
                        || item.source_id.starts_with("maintenance-docs:"))
            })
            .collect();
        assert!(
            brain_policy::provider_egress_allowed(&context.principal, "public"),
            "public provider egress not granted"
        );
        if !evidence.is_empty() {
            retrieval
                .validate_evidence(&query, &context, &evidence, true)
                .expect("public provider egress denied");
        }
        let result = provider.answer_accounted(&query, &context, &evidence);
        let social = result
            .as_ref()
            .is_ok_and(|accounted| accounted.value.is_social());
        let mut violation = None;
        let classification = match result {
            Ok(mut accounted) => match accounted.value.render(&evidence) {
                Err(reason @ (AnswerViolation::Format | AnswerViolation::Citation)) => {
                    violation = Some(reason.code());
                    format_silence += 1;
                    "format_silence"
                }
                Err(AnswerViolation::Value) => {
                    violation = Some(AnswerViolation::Value.code());
                    value_silence += 1;
                    "value_silence"
                }
                Ok(()) if accounted.value.text.is_empty() => {
                    evidence_silence += 1;
                    "evidence_silence"
                }
                Ok(()) => {
                    answered += 1;
                    if index < 4 && social {
                        social_answered += 1;
                    }
                    "answered"
                }
            },
            Err(failure) => match AnswerViolation::from_error(&failure.error) {
                Some(reason @ (AnswerViolation::Format | AnswerViolation::Citation)) => {
                    violation = Some(reason.code());
                    format_silence += 1;
                    "format_silence"
                }
                Some(AnswerViolation::Value) => {
                    violation = Some(AnswerViolation::Value.code());
                    value_silence += 1;
                    "value_silence"
                }
                None => {
                    provider_errors += 1;
                    "provider_error"
                }
            },
        };
        println!(
            "{}",
            json!({"case":index,"type":category,"outcome":classification,"evidence":evidence.len(),"social":social,"violation":violation})
        );
    }
    println!(
        "{}",
        json!({"cases":cases.len(),"answered":answered,"social_answered":social_answered,"format_silence":format_silence,
        "format_silence_percent":format_silence * 100 / cases.len(), "value_silence":value_silence,
        "evidence_silence":evidence_silence,"provider_errors":provider_errors,
        "source_release":grant.release.as_ref().unwrap_or(&prepared.config.release).id,"candidate_contract":true,"discord_reads":false})
    );
    assert_eq!(
        provider_errors, 0,
        "compliance cannot be measured with provider errors"
    );
    assert_eq!(
        social_answered, 4,
        "social categories must answer with the social signal"
    );
    assert!(
        format_silence <= 2,
        "format silence exceeds ten percent; approval required before merge"
    );
}
