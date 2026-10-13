use super::{bounded_card_result, bounded_comparison_result, invalid};
use brain_contracts::{PortError, ToolRequest, ToolSubrequest};
use serde_json::Value;
use sqlx::PgConnection;

pub(super) const COMPARE: &str = include_str!("pg_asset_compare.sql");

fn database_error(_: sqlx::Error) -> PortError {
    PortError::Unavailable("Spielwerte können derzeit nicht sicher gelesen werden.".into())
}

pub(crate) async fn read(
    connection: &mut PgConnection,
    call_id: &str,
    request: &ToolRequest,
) -> Result<Option<Value>, PortError> {
    let result = match request.subrequest() {
        ToolSubrequest::GetItem(item) => {
            let card = sqlx::query_scalar::<_, Value>(
                "SELECT card FROM serve.item_card WHERE entity_id=$1 LIMIT 1",
            )
            .bind(item.item as i64)
            .fetch_optional(&mut *connection)
            .await
            .map_err(database_error)?;
            let Some(card) = card else {
                return Ok(None);
            };
            bounded_card_result(call_id, request, card)?
        }
        ToolSubrequest::GetAbility(ability) => {
            let selection = if let (Some(hero), Some(slot)) = (ability.hero, ability.slot) {
                let profile = sqlx::query_scalar::<_,Value>(
                    "SELECT jsonb_build_object('identity',identity,'abilities',abilities) FROM serve.hero_profile WHERE entity_id=$1 LIMIT 1")
                    .bind(hero as i64).fetch_optional(&mut *connection).await.map_err(database_error)?;
                let Some(profile) = profile else {
                    return Ok(None);
                };
                if profile["abilities"]["available"] != true
                    || profile["abilities"]["truncated"] != false
                    || profile["abilities"]["omitted_count"].as_u64() != Some(0)
                {
                    return Ok(None);
                }
                let Some(links) = profile["abilities"]["links"].as_array() else {
                    return Ok(None);
                };
                let matching: Vec<_> = links
                    .iter()
                    .filter(|row| {
                        row["kind"] == "has_ability"
                            && row["from_entity_id"].as_u64() == Some(hero)
                            && row["slot"].as_u64() == Some(u64::from(slot))
                    })
                    .collect();
                if matching.len() != 1 {
                    return Ok(None);
                }
                let mut link = (*matching[0]).clone();
                link["entity_id"] = serde_json::json!(hero);
                link["identity"] = profile["identity"].clone();
                Some(link)
            } else {
                None
            };
            let id = ability
                .ability
                .and_then(|id| i64::try_from(id).ok())
                .or_else(|| {
                    selection
                        .as_ref()
                        .and_then(|link| link["to_entity_id"].as_i64())
                        .filter(|id| *id > 0)
                });
            let Some(id) = id else {
                return Ok(None);
            };
            let card = sqlx::query_scalar::<_, Value>(
                "SELECT card FROM serve.ability_card WHERE entity_id=$1 LIMIT 1",
            )
            .bind(id)
            .fetch_optional(&mut *connection)
            .await
            .map_err(database_error)?;
            let Some(mut card) = card else {
                return Ok(None);
            };
            if let Some(link) = selection {
                card["abilities"] =
                    serde_json::json!({"links":[link],"truncated":false,"omitted_count":0});
                card["hero_selection"] = serde_json::json!({"available":true});
            }
            bounded_card_result(call_id, request, card)?
        }
        ToolSubrequest::Compare(compare) => {
            let entities: Vec<i64> = compare.entities.iter().map(|id| *id as i64).collect();
            let value = sqlx::query_scalar::<_, Value>(COMPARE)
                .bind(entities)
                .bind(&compare.stats)
                .fetch_one(&mut *connection)
                .await
                .map_err(database_error)?;
            bounded_comparison_result(call_id, request, value)?
        }
        _ => return Err(invalid()),
    };
    Ok(result.map(|result| result.result))
}
