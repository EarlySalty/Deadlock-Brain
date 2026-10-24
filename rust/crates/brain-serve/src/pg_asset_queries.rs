use super::{bounded_card_result, bounded_comparison_result, invalid};
use brain_contracts::{PortError, ToolRequest, ToolSubrequest};
use serde_json::{json, Value};
use sqlx::PgConnection;

pub(super) const COMPARE: &str = include_str!("pg_asset_compare.sql");

const CARD_FACTS: &str = r#"
SELECT jsonb_build_object(
    'facts',COALESCE(jsonb_agg(r.fact ORDER BY r.ord) FILTER(WHERE r.kept),'[]'::jsonb),
    'truncated',COALESCE(max(r.total),0)>count(*) FILTER(WHERE r.kept),
    'omitted_count',COALESCE(max(r.total),0)-count(*) FILTER(WHERE r.kept)
)
FROM (
    SELECT q.fact,q.ord,count(*) OVER() AS total,
        q.ord<=32 AND sum(q.result_bytes) OVER(ORDER BY q.ord)<=10240 AS kept
    FROM (
        SELECT row_number() OVER(
                ORDER BY CASE
                    WHEN c.stat_key='item_cost' THEN 0
                    WHEN c.stat_key LIKE '%\_description\_%' ESCAPE '\'
                        AND c.fact->>'value'<>'' THEN 1
                    WHEN c.stat_key LIKE 'identity\_%' ESCAPE '\' THEN 5
                    WHEN NULLIF(c.fact->>'label','') IS NOT NULL
                        AND c.fact->'value' NOT IN('0'::jsonb,'-1'::jsonb,'""'::jsonb,'false'::jsonb,'null'::jsonb) THEN
                        CASE WHEN c.fact->'condition'->>'usage_flag_0'='ConditionallyApplied'
                            OR c.fact->'condition'->>'scale_class' LIKE '%damage%' THEN 2 ELSE 3 END
                    ELSE 4 END,c.stat_key,c.fact::text
            ) AS ord,
            c.fact,octet_length(regexp_replace(c.fact::text,'<[^>]*>','','g'))+187-octet_length(c.fact->>'citation')
                +COALESCE((SELECT sum(187-octet_length(q->>'citation'))
                    FROM jsonb_array_elements(c.fact->'qualifiers') q),0) AS result_bytes
        FROM serve.p2_fact c WHERE c.entity_id=$1
    ) q
) r
"#;

const CARD_LINKS: &str = r#"
WITH selected AS (SELECT $1::bigint AS entity_id,$2::text AS key,$3::text AS kind)
SELECT jsonb_build_object(
    'links',COALESCE(jsonb_agg(r.link ORDER BY r.ord) FILTER(WHERE r.kept),'[]'::jsonb),
    'truncated',COALESCE(max(r.total),0)>count(*) FILTER(WHERE r.kept),
    'omitted_count',COALESCE(max(r.total),0)-count(*) FILTER(WHERE r.kept)
)
FROM (
    SELECT q.link,q.ord,count(*) OVER() AS total,
        q.ord<=16 AND sum(octet_length(q.link::text)) OVER(ORDER BY q.ord)<=2560 AS kept
    FROM (
        SELECT row_number() OVER(ORDER BY x.grp,length(x.sort),x.sort,x.receipt_id) AS ord,
            jsonb_build_object(
                'entity_id',x.entity_id,'kind',x.kind,
                'from_entity_id',x.from_id,'to_entity_id',x.to_id,
                'citation',x.receipt_id::text,'source_revision',x.source_revision,
                'patch',NULL,'identity',x.identity
            ) AS link
        FROM selected e
        CROSS JOIN LATERAL (
            SELECT 0 AS grp,c.stat_key AS sort,'component_of' AS kind,
                t.entity_id AS from_id,c.entity_id AS to_id,t.entity_id,
                c.receipt_id,c.source_revision,t.identity
            FROM serve.p2_component c
            JOIN serve.current_alias a ON a.alias_norm=c.value_text AND a.lang='und'
            CROSS JOIN LATERAL (
                SELECT entity_id,identity FROM serve.entity_identity
                WHERE entity_id=a.entity_id AND key=c.value_text AND kind='item' LIMIT 1
            ) t
            WHERE c.entity_id=e.entity_id
                AND c.stat_key LIKE 'component\_item\_%' ESCAPE '\'
            UNION ALL
            SELECT 0,c.stat_key,'upgrades_to',e.entity_id,t.entity_id,t.entity_id,
                c.receipt_id,c.source_revision,t.identity
            FROM serve.p2_component c
            CROSS JOIN LATERAL (
                SELECT entity_id,identity FROM serve.entity_identity
                WHERE entity_id=c.entity_id AND kind='item' LIMIT 1
            ) t
            WHERE c.value_text=e.key AND e.kind='item'
                AND c.stat_key LIKE 'component\_item\_%' ESCAPE '\'
            UNION ALL
            SELECT 1,'',r.kind,r.from_entity,r.to_entity,t.entity_id,
                r.receipt_id,r.source_revision,t.identity
            FROM serve.current_relation r
            CROSS JOIN LATERAL (
                SELECT i.entity_id,i.identity FROM serve.entity_identity i
                WHERE i.entity_id=CASE WHEN r.from_entity=e.entity_id
                    THEN r.to_entity ELSE r.from_entity END
                    AND (i.kind<>'hero' OR serve.hero_is_playable(i.entity_id)) LIMIT 1
            ) t
            WHERE r.from_entity=e.entity_id OR r.to_entity=e.entity_id
        ) x
    ) q
) r
"#;

const ABILITY_LINKS: &str = r#"
WITH current_links AS (
    SELECT r.from_entity,r.to_entity,r.receipt_id
    FROM serve.current_relation r
    CROSS JOIN LATERAL (
        SELECT peer.entity_id FROM serve.entity_identity peer
        WHERE peer.entity_id=CASE WHEN r.from_entity=$1 THEN r.to_entity ELSE r.from_entity END
            AND (peer.kind<>'hero' OR serve.hero_is_playable(peer.entity_id))
        LIMIT 1
    ) peer
    WHERE (r.from_entity=$1 OR r.to_entity=$1)
        AND (r.kind<>'has_ability' OR r.source<>'assets' OR EXISTS (
            SELECT 1 FROM serve.p2_hero_slot slot
            WHERE slot.from_entity_id=r.from_entity AND slot.to_entity_id=r.to_entity
                AND slot.link->>'citation'=r.receipt_id::text
        ))
)
SELECT jsonb_build_object(
    'links',COALESCE(shown.links,'[]'::jsonb),
    'truncated',total.n>shown.n,'omitted_count',total.n-shown.n
)
FROM (
    SELECT jsonb_agg(entry.link ORDER BY entry.position) AS links,count(*) AS n
    FROM jsonb_array_elements($2::jsonb) WITH ORDINALITY entry(link,position)
    WHERE EXISTS (
        SELECT 1 FROM current_links valid
        WHERE valid.from_entity::text=entry.link->>'from_entity_id'
            AND valid.to_entity::text=entry.link->>'to_entity_id'
            AND valid.receipt_id::text=entry.link->>'citation'
    )
) shown
CROSS JOIN (SELECT count(*) AS n FROM current_links) total
"#;

fn database_error(_: sqlx::Error) -> PortError {
    PortError::Unavailable("Spielwerte können derzeit nicht sicher gelesen werden.".into())
}

async fn read_lineage(
    connection: &mut PgConnection,
    id: i64,
    history_limit: i32,
) -> Result<Value, PortError> {
    sqlx::query_scalar::<_, Value>("SELECT serve.item_lineage($1,$2)")
        .bind(id)
        .bind(history_limit)
        .fetch_optional(&mut *connection)
        .await
        .map_err(database_error)?
        .filter(Value::is_object)
        .ok_or_else(invalid)
}

fn successor_id(lineage: &Value) -> Option<i64> {
    if lineage["status"] != "removed" {
        return None;
    }
    lineage["removed"]["successor"]["entity_id"]
        .as_i64()
        .filter(|id| *id > 0)
}

pub(crate) async fn attach_successor(
    connection: &mut PgConnection,
    target: &mut Value,
    history_limit: i32,
) -> Result<(), PortError> {
    let Some(successor) = successor_id(&target["lineage"]) else {
        return Ok(());
    };
    if let Some(mut card) = base_card(connection, successor, "item").await? {
        card["lineage"] = read_lineage(connection, successor, history_limit).await?;
        target["successor_item"] = card;
    }
    Ok(())
}

async fn read_card(
    connection: &mut PgConnection,
    id: i64,
    kind: &str,
) -> Result<Option<Value>, PortError> {
    let Some(mut card) = base_card(connection, id, kind).await? else {
        return Ok(None);
    };
    if kind == "item" {
        card["lineage"] = read_lineage(connection, id, 8).await?;
        attach_successor(connection, &mut card, 3).await?;
    }
    if kind == "ability" {
        card["hero_selection"] = json!({"available":false,"reason":"hero_slot_not_published"});
    }
    Ok(Some(card))
}

async fn base_card(
    connection: &mut PgConnection,
    id: i64,
    kind: &str,
) -> Result<Option<Value>, PortError> {
    let identity = sqlx::query_scalar::<_, Value>(
        "SELECT identity FROM serve.entity_identity WHERE entity_id=$1 AND kind=$2 LIMIT 1",
    )
    .bind(id)
    .bind(kind)
    .fetch_optional(&mut *connection)
    .await
    .map_err(database_error)?;
    let Some(identity) = identity else {
        return Ok(None);
    };
    let key = identity["key"].as_str().ok_or_else(invalid)?;
    let facts = sqlx::query_scalar::<_, Value>(CARD_FACTS)
        .bind(id)
        .fetch_one(&mut *connection)
        .await
        .map_err(database_error)?;
    let mut links = sqlx::query_scalar::<_, Value>(CARD_LINKS)
        .bind(id)
        .bind(key)
        .bind(kind)
        .fetch_one(&mut *connection)
        .await
        .map_err(database_error)?;
    if kind == "ability" {
        links = sqlx::query_scalar::<_, Value>(ABILITY_LINKS)
            .bind(id)
            .bind(&links["links"])
            .fetch_one(&mut *connection)
            .await
            .map_err(database_error)?;
    }
    let mut card = json!({"identity":identity,"facts":facts,"links":links});
    crate::game_text::present(&mut card["facts"]);
    Ok(Some(card))
}

pub(crate) async fn read(
    connection: &mut PgConnection,
    call_id: &str,
    request: &ToolRequest,
) -> Result<Option<Value>, PortError> {
    let result = match request.subrequest() {
        ToolSubrequest::GetItem(item) => {
            let Some(card) = read_card(connection, item.item as i64, "item").await? else {
                return Ok(None);
            };
            bounded_card_result(call_id, request, card)?
        }
        ToolSubrequest::GetAbility(ability) => {
            let selection = if let (Some(hero), Some(slot)) = (ability.hero, ability.slot) {
                let links = sqlx::query_scalar::<_, Value>(
                    "SELECT link FROM serve.p2_hero_slot WHERE from_entity_id=$1 AND slot=$2 AND serve.hero_is_playable($1) LIMIT 2",
                )
                .bind(hero as i64)
                .bind(i32::from(slot))
                .fetch_all(&mut *connection)
                .await
                .map_err(database_error)?;
                if links.len() != 1 {
                    return Ok(None);
                }
                links.into_iter().next()
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
            let Some(mut card) = read_card(connection, id, "ability").await? else {
                return Ok(None);
            };
            if let Some(link) = selection {
                card["abilities"] = json!({"links":[link],"truncated":false,"omitted_count":0});
                card["hero_selection"] = json!({"available":true});
            }
            bounded_card_result(call_id, request, card)?
        }
        ToolSubrequest::Compare(compare) => {
            let entities: Vec<i64> = compare.entities.iter().map(|id| *id as i64).collect();
            let mut value = sqlx::query_scalar::<_, Value>(COMPARE)
                .bind(entities)
                .bind(&compare.stats)
                .fetch_one(&mut *connection)
                .await
                .map_err(database_error)?;
            crate::game_text::present(&mut value["stats"]);
            bounded_comparison_result(call_id, request, value)?
        }
        _ => return Err(invalid()),
    };
    Ok(result.map(|result| result.result))
}
