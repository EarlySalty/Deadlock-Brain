#[tokio::main]
async fn main() {
    let path = std::env::args_os()
        .nth(1)
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| "/etc/deadlock-brain/infisical.json".into());
    let result = async {
        let pool = dbrain_sources::core::pg::pg_pool_from_config(&path, true).await.map_err(|_| "Vorhandener Infisical-/FD-Zugang ist nicht verfügbar")?;
        let mut tx = pool.begin().await.map_err(|_| "Lesetransaktion fehlt")?;
        sqlx::raw_sql("SET TRANSACTION READ ONLY; SET LOCAL statement_timeout='5000ms'").execute(&mut *tx).await.map_err(|_| "Leseschutz fehlt")?;
        let rows: Vec<serde_json::Value> = sqlx::query_scalar("SELECT jsonb_build_object('relation', 'patch_changes', 'rows', count(*), 'patches', count(DISTINCT patch_date), 'latest', max(patch_date)) FROM brain.patch_changes").fetch_all(&mut *tx).await.map_err(|_| "Patch-Bestandsabfrage fehlgeschlagen")?;
        let entities: Vec<serde_json::Value> = sqlx::query_scalar("SELECT jsonb_build_object('kind',entity_type,'rows',count(*)) FROM brain.entities WHERE entity_type IN ('hero','ability','item') GROUP BY entity_type ORDER BY entity_type").fetch_all(&mut *tx).await.map_err(|_| "Entitätsbestandsabfrage fehlgeschlagen")?;
        tx.commit().await.map_err(|_| "Lesetransaktion konnte nicht abgeschlossen werden")?;
        println!("{}", serde_json::json!({"patch_history":rows,"entities":entities}));
        Ok::<_, &str>(())
    }.await;
    if let Err(error) = result {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
