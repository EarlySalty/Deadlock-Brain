use super::*;
const FILES: [&str; 6] = [
    "public-discord-core.json",
    "public-discord-tools.json",
    "public-integration.json",
    "public-patchnotes-turniere.json",
    "public-steam-website.json",
    "public-twitch.json",
];

pub fn inventory(root: &Path) -> Result<Value, String> {
    let root = root
        .canonicalize()
        .map_err(|_| "Docs checkout unavailable")?;
    let revision = git_text(&root, &["rev-parse", "HEAD"])?;
    if !valid_hash(&revision, 40) {
        return Err("Docs revision invalid".into());
    }
    if !git_text(&root, &["status", "--porcelain", "--", "evals"])?.is_empty() {
        return Err("Docs eval files are dirty; pin them before inventory".into());
    }
    let mut packages = vec![];
    let mut total = 0;
    let mut unanswerable = 0;
    for name in FILES {
        let path = root
            .join("evals")
            .join(name)
            .canonicalize()
            .map_err(|_| "required Docs eval file unavailable")?;
        if !path.starts_with(&root) {
            return Err("Docs eval path escapes checkout".into());
        }
        let bytes = read_bytes(&path)?;
        let cases: Vec<Value> = parse(&bytes)?;
        if cases.is_empty() {
            return Err("empty Docs eval package".into());
        }
        let mut abstentions = 0;
        for case in &cases {
            if !case["question"]
                .as_str()
                .is_some_and(|s| !s.trim().is_empty())
                || !case["answerable"].is_boolean()
            {
                return Err("invalid Docs support case".into());
            }
            for key in [
                "expected_sources",
                "context_terms",
                "answer_terms",
                "forbidden_terms",
            ] {
                if !case[key].as_array().is_some_and(|items| {
                    items
                        .iter()
                        .all(|item| item.as_str().is_some_and(|s| !s.trim().is_empty()))
                }) {
                    return Err("invalid Docs independent labels".into());
                }
            }
            if case["answerable"] == true
                && case["expected_sources"]
                    .as_array()
                    .is_some_and(Vec::is_empty)
            {
                return Err("answerable Docs case has no reference source".into());
            }
            if case["answerable"] == false {
                abstentions += 1;
            }
        }
        total += cases.len();
        unanswerable += abstentions;
        packages.push(json!({"path":format!("evals/{name}"),"sha256":digest(&bytes),"case_count":cases.len(),"unanswerable_count":abstentions,"labels":"expected_sources/context_terms/answer_terms/forbidden_terms","executed":false}));
    }
    if git_text(&root, &["rev-parse", "HEAD"])? != revision
        || !git_text(&root, &["status", "--porcelain", "--", "evals"])?.is_empty()
    {
        return Err("Docs checkout changed during inventory".into());
    }
    // Only counts and fingerprints leave this tool, never questions, answers or private docs.
    Ok(
        json!({"schema_version":1,"kind":"support_eval_inventory","source_repository":"Deadlock-Docs","source_commit":revision,"files":packages,"case_count":total,"unanswerable_count":unanswerable,"executed":false,"release_approved":false}),
    )
}
