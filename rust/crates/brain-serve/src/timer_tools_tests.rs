use super::*;
use brain_contracts::{tools::TimerKind, ToolCall, ToolSubrequest};

#[test]
fn timer_contract_is_closed_and_uses_canonical_keys() {
    let definition = definition();
    definition.validate().unwrap();
    for timer in [
        TimerKind::Urn,
        TimerKind::SoulBox,
        TimerKind::MidBoss,
        TimerKind::Rejuvenator,
    ] {
        let call = ToolCall {
            id: "timer-contract".into(),
            name: ToolName::GetTimer,
            arguments: json!({"timer":timer.as_str(),"patch":"current"}),
        };
        let request = call.validate(std::slice::from_ref(&definition)).unwrap();
        assert_eq!(
            request.subrequest(),
            &ToolSubrequest::GetTimer(GetTimerRequest {
                timer,
                patch: Some("current".into())
            })
        );
        assert_eq!(
            serde_json::to_value(request.subrequest()).unwrap()["name"],
            "get_timer"
        );
    }
    for arguments in [
        json!({}),
        json!({"timer":"unknown"}),
        json!({"timer":"Urne"}),
        json!({"timer":7}),
        json!({"timer":"urn","patch":" "}),
        json!({"timer":"urn","patch":"x".repeat(129)}),
        json!({"timer":"urn","patch":"ä".repeat(65)}),
        json!({"timer":"urn","patch":"current\n"}),
        json!({"timer":"urn","sql":"SELECT 1"}),
        json!({"timer":"urn","scopes":["internal"]}),
    ] {
        assert!(ToolCall {
            id: "invalid-timer".into(),
            name: ToolName::GetTimer,
            arguments
        }
        .validate(std::slice::from_ref(&definition))
        .is_err());
    }
}

#[test]
fn timer_rows_trim_without_losing_receipt_condition_or_value() {
    use super::super::{citations, fits, trim_row};
    use brain_contracts::ToolResult;
    use std::collections::BTreeSet;

    let row = json!({"timer":"urn","stat_key":"timer.first_spawn_seconds","value":300,
        "unit":"seconds","condition":{"clock":"match"},"citation":"7",
        "patch":null,"source_revision":"commit-a"});
    let mut result = ToolResult {
        call_id: "fixture".into(),
        name: ToolName::GetTimer,
        result: json!({"timer":"urn","facts":vec![row.clone();200],"truncated":false,"omitted_count":0}),
        evidence_ids: vec!["7".into()],
        is_error: false,
    };
    while !fits(&result, 1).unwrap() {
        assert!(trim_row(&mut result.result));
    }
    assert!(serde_json::to_vec(&result).unwrap().len() <= 16_384);
    let rows = result.result["facts"].as_array().unwrap();
    assert!(rows.iter().all(|value| value == &row));
    assert_eq!(result.result["omitted_count"], 200 - rows.len());
    let mut ids = BTreeSet::new();
    citations(&result.result, &mut ids).unwrap();
    assert_eq!(ids, BTreeSet::from([7]));
}
