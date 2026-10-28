use crate::sources;
use sqlx::PgPool;
use std::path::Path;

pub const SOURCE: &str = "deadlock-data";
const INGEST_HISTORY: usize = 64;
const BACKFILL_HISTORY: usize = 5000;
const SUBJECT_CHARS: usize = 1000;

#[derive(Debug, PartialEq, Eq)]
pub struct Commit {
    pub revision: String,
    pub committed_at: i64,
    pub subject: String,
    pub client_build: Option<i32>,
}

pub fn client_build(subject: &str) -> Option<i32> {
    if !subject.starts_with("Deadbot ") {
        return None;
    }
    subject
        .split(" | ")
        .find_map(|part| part.strip_prefix("Client "))
        .and_then(|rest| rest.split_whitespace().next())
        .and_then(|number| number.parse().ok())
        .filter(|number| *number > 0)
}

fn valid_revision(revision: &str) -> bool {
    [40, 64].contains(&revision.len())
        && revision
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

pub fn parse_log(log: &str) -> anyhow::Result<Vec<Commit>> {
    log.split('\x1e')
        .map(str::trim)
        .filter(|record| !record.is_empty())
        .map(|record| {
            let mut fields = record.splitn(3, '\x1f');
            let revision = fields.next().unwrap_or_default().to_owned();
            let committed_at = fields.next().unwrap_or_default().parse()?;
            let subject: String = fields
                .next()
                .unwrap_or_default()
                .chars()
                .take(SUBJECT_CHARS)
                .collect();
            anyhow::ensure!(valid_revision(&revision), "Ungültige Git-Revision");
            Ok(Commit {
                client_build: client_build(&subject),
                revision,
                committed_at,
                subject,
            })
        })
        .collect()
}

pub fn commits(repository: &Path, revision: &str, limit: usize) -> anyhow::Result<Vec<Commit>> {
    anyhow::ensure!(valid_revision(revision), "Ungültige Git-Revision");
    let log = sources::git(
        repository,
        &[
            "log",
            &format!("--max-count={limit}"),
            "--format=%H%x1f%ct%x1f%s%x1e",
            revision,
            "--",
        ],
    )?;
    parse_log(&String::from_utf8(log)?)
}

async fn record_commits(pool: &PgPool, commits: &[Commit]) -> anyhow::Result<u64> {
    let mut inserted = 0;
    for commit in commits {
        inserted += sqlx::query(
            "INSERT INTO ops.upstream_commit (source,revision,committed_at,subject,client_build)
             VALUES ($1,$2,to_timestamp($3::bigint),$4,$5) ON CONFLICT DO NOTHING",
        )
        .bind(SOURCE)
        .bind(&commit.revision)
        .bind(commit.committed_at)
        .bind(&commit.subject)
        .bind(commit.client_build)
        .execute(pool)
        .await?
        .rows_affected();
    }
    Ok(inserted)
}

pub async fn record_ingest(
    pool: &PgPool,
    run_id: i64,
    repository: &Path,
    revision: &str,
) -> anyhow::Result<()> {
    record_commits(pool, &commits(repository, revision, INGEST_HISTORY)?).await?;
    sqlx::query(
        "INSERT INTO ops.ingest_run_revision (run_id,source,revision) VALUES ($1,$2,$3)
         ON CONFLICT DO NOTHING",
    )
    .bind(run_id)
    .bind(SOURCE)
    .bind(revision)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn backfill(pool: &PgPool, repository: &Path) -> anyhow::Result<(u64, u64)> {
    let head = String::from_utf8(sources::git(
        repository,
        &["rev-parse", "--verify", "HEAD^{commit}"],
    )?)?;
    let commits =
        record_commits(pool, &commits(repository, head.trim(), BACKFILL_HISTORY)?).await?;
    let runs = sqlx::query(
        "INSERT INTO ops.ingest_run_revision (run_id,source,revision,inferred)
         SELECT r.id,$1,coalesce(o.revision,h.revision),o.revision IS NULL
         FROM ops.ingest_run r
         LEFT JOIN LATERAL (
             SELECT min(x.revision) AS revision FROM raw.object x
             WHERE x.ingest_run_id=r.id AND x.source=$1
             HAVING count(DISTINCT x.revision)=1
         ) o ON true
         LEFT JOIN LATERAL (
             SELECT c.revision FROM ops.upstream_commit c
             WHERE c.source=$1 AND c.committed_at<=r.started
             ORDER BY c.committed_at DESC,c.revision
             LIMIT 1
         ) h ON true
         WHERE r.source=$1 AND r.status='succeeded'
             AND EXISTS (
                 SELECT 1 FROM ops.upstream_commit c
                 WHERE c.source=$1 AND c.revision=coalesce(o.revision,h.revision)
             )
         ON CONFLICT (run_id) DO NOTHING",
    )
    .bind(SOURCE)
    .execute(pool)
    .await?
    .rows_affected();
    Ok((commits, runs))
}

type Row = (
    Option<i64>,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<i32>,
);

type Summary = (Option<String>, Option<i64>, Option<String>, Option<String>);

const PATCH_REPORT: &str = "SELECT patch_id,
        to_char(published_at AT TIME ZONE 'UTC','YYYY-MM-DD HH24:MI'),
        round((extract(epoch FROM detect_lag)/60)::numeric,1)::text,
        round((extract(epoch FROM upstream_lag)/60)::numeric,1)::text,
        round((extract(epoch FROM ingest_lag)/60)::numeric,1)::text,
        round((extract(epoch FROM total_lag)/60)::numeric,1)::text,
        left(upstream_revision,10)||CASE WHEN ingest_inferred THEN '*' ELSE '' END,
        label,client_build
    FROM ops.patch_freshness WHERE published_at>=$1::timestamptz ORDER BY published_at,patch_id";

const SUMMARY: &str = "WITH stage(name,lag) AS (
        SELECT 'detect',detect_lag FROM ops.patch_freshness WHERE published_at>=$1::timestamptz
        UNION ALL SELECT 'upstream',upstream_lag FROM ops.patch_freshness WHERE published_at>=$1::timestamptz
        UNION ALL SELECT 'ingest',ingest_lag FROM ops.patch_freshness WHERE published_at>=$1::timestamptz
        UNION ALL SELECT 'total',total_lag FROM ops.patch_freshness WHERE published_at>=$1::timestamptz
        UNION ALL SELECT 'silent_ingest',ingest_lag FROM ops.game_data_freshness
            WHERE silent AND committed_at>=$1::timestamptz
    )
    SELECT name,count(lag),
        round((percentile_cont(0.5) WITHIN GROUP (ORDER BY extract(epoch FROM lag))/60)::numeric,1)::text,
        round((max(extract(epoch FROM lag))/60)::numeric,1)::text
    FROM stage GROUP BY name ORDER BY min(CASE name WHEN 'detect' THEN 1 WHEN 'upstream' THEN 2
        WHEN 'ingest' THEN 3 WHEN 'total' THEN 4 ELSE 5 END)";

fn cell(value: &Option<String>) -> &str {
    value.as_deref().unwrap_or("-")
}

pub async fn report(pool: &PgPool, since: &str) -> anyhow::Result<()> {
    let rows: Vec<Row> = sqlx::query_as(PATCH_REPORT)
        .bind(since)
        .fetch_all(pool)
        .await?;
    println!(
        "{:>6}  {:<16}  {:>9}  {:>9}  {:>9}  {:>9}  {:<11}  {:>6}  label",
        "patch",
        "published_utc",
        "detect_m",
        "upstream_m",
        "ingest_m",
        "total_m",
        "revision",
        "client"
    );
    for (patch, published, detect, upstream, ingest, total, revision, label, client) in &rows {
        println!(
            "{:>6}  {:<16}  {:>9}  {:>10}  {:>9}  {:>9}  {:<11}  {:>6}  {}",
            patch.map(|p| p.to_string()).unwrap_or_default(),
            cell(published),
            cell(detect),
            cell(upstream),
            cell(ingest),
            cell(total),
            cell(revision),
            client.map(|c| c.to_string()).unwrap_or_else(|| "-".into()),
            cell(label)
        );
    }
    let summary: Vec<Summary> = sqlx::query_as(SUMMARY).bind(since).fetch_all(pool).await?;
    println!();
    println!(
        "{:<14}  {:>5}  {:>9}  {:>9}",
        "stage", "n", "median_m", "max_m"
    );
    for (name, count, median, max) in &summary {
        println!(
            "{:<14}  {:>5}  {:>9}  {:>9}",
            cell(name),
            count.unwrap_or_default(),
            cell(median),
            cell(max)
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn client_build_comes_only_from_deadbot_subjects() {
        assert_eq!(
            client_build("Deadbot v1.18.7 | Client 6821 - Oct 26 2026 (deadbot@daeaa56)"),
            Some(6821)
        );
        assert_eq!(
            client_build("Merge pull request #250 from deadlock-wiki/chore/remove-stale-images"),
            None
        );
    }

    #[test]
    fn log_records_keep_separators_out_of_subjects() {
        let log = format!(
            "{}\x1f1761509354\x1fDeadbot v1.18.7 | Client 6821 - Oct 26 2026\x1e\n{}\x1f1761301615\x1fa\x1fb\x1e\n",
            "f".repeat(40),
            "0".repeat(40)
        );
        let parsed = parse_log(&log).unwrap_or_default();
        assert_eq!(parsed.len(), 2);
        assert_eq!(parsed[0].client_build, Some(6821));
        assert_eq!(parsed[1].subject, "a\x1fb");
        assert!(parse_log("zz\x1f1\x1fx\x1e").is_err());
    }
}
