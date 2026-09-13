use anyhow::{anyhow, Result};
use nix::fcntl::{fcntl, FcntlArg, FdFlag};
use serde::Deserialize;
use std::{
    path::{Path, PathBuf},
    time::Duration,
};
use tokio::io::AsyncReadExt;
use zeroize::{Zeroize, Zeroizing};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Config {
    project_id: String,
    environment: String,
    secret_path: String,
    credential_fd: i32,
    socket_path: PathBuf,
    database_secret: String,
}

#[derive(Deserialize)]
struct Reply {
    #[serde(default)]
    secrets: Vec<Entry>,
    #[serde(default)]
    imports: Vec<Import>,
}

#[derive(Deserialize)]
struct Import {
    #[serde(default)]
    secrets: Vec<Entry>,
}

#[derive(Deserialize)]
struct Entry {
    #[serde(rename = "secretKey")]
    name: String,
    #[serde(rename = "secretValue")]
    value: String,
}

impl Drop for Entry {
    fn drop(&mut self) {
        self.value.zeroize();
    }
}

pub(super) async fn database_dsn(path: &Path) -> Result<Zeroizing<String>> {
    let data =
        std::fs::read(path).map_err(|_| anyhow!("Infisical-Konfiguration ist nicht lesbar."))?;
    let config: Config = serde_json::from_slice(&data)
        .map_err(|_| anyhow!("Infisical-Konfiguration ist ungültig."))?;
    let fd = config.credential_fd;
    if fd < 3 {
        return Err(anyhow!(
            "Infisical benötigt einen ausdrücklich übergebenen Credential-FD."
        ));
    }
    let flags = fcntl(fd, FcntlArg::F_GETFD)
        .map_err(|_| anyhow!("Infisical-Credential-FD ist nicht verfügbar."))?;
    fcntl(
        fd,
        FcntlArg::F_SETFD(FdFlag::from_bits_retain(flags) | FdFlag::FD_CLOEXEC),
    )
    .map_err(|_| anyhow!("Infisical-Credential-FD konnte nicht geschützt werden."))?;
    let descriptor = filedescriptor::FileDescriptor::dup(&fd)
        .map_err(|_| anyhow!("Infisical-Credential-FD ist nicht verfügbar."))?;
    let file = descriptor
        .as_file()
        .map_err(|_| anyhow!("Infisical-Credential-FD ist nicht lesbar."))?;
    if !file
        .metadata()
        .map_err(|_| anyhow!("Infisical-Credential-FD konnte nicht geprüft werden."))?
        .is_file()
    {
        return Err(anyhow!(
            "Infisical benötigt einen regulären Credential-Dateideskriptor."
        ));
    }
    let mut token = Zeroizing::new(Vec::new());
    tokio::time::timeout(
        Duration::from_secs(5),
        tokio::fs::File::from_std(file)
            .take(8193)
            .read_to_end(&mut token),
    )
    .await
    .map_err(|_| anyhow!("Infisical-Credential wurde nicht rechtzeitig geliefert."))?
    .map_err(|_| anyhow!("Infisical-Credential ist nicht lesbar."))?;
    if token.is_empty() || token.len() > 8192 {
        return Err(anyhow!("Infisical-Credential hat eine ungültige Größe."));
    }
    let token_text = std::str::from_utf8(&token)
        .map_err(|_| anyhow!("Infisical-Credential ist ungültig."))?
        .trim();
    let client = uplink_infisical_transport::client_builder(&config.socket_path, 0)
        .map_err(|message| anyhow!(message))?
        .timeout(Duration::from_secs(15))
        .build()
        .map_err(|_| anyhow!("Infisical-Client konnte nicht gestartet werden."))?;
    let mut response = client
        .get(format!(
            "{}/api/v4/secrets/",
            uplink_infisical_transport::BASE_URL
        ))
        .query(&[
            ("projectId", config.project_id.as_str()),
            ("environment", config.environment.as_str()),
            ("secretPath", config.secret_path.as_str()),
            ("viewSecretValue", "true"),
            ("includeImports", "true"),
            ("recursive", "false"),
        ])
        .bearer_auth(token_text)
        .send()
        .await
        .map_err(|_| anyhow!("Infisical ist nicht erreichbar."))?;
    if !response.status().is_success() {
        return Err(anyhow!(
            "Infisical-Zugriff fehlgeschlagen (HTTP {}).",
            response.status().as_u16()
        ));
    }
    let mut body = Zeroizing::new(Vec::new());
    while let Some(chunk) = response
        .chunk()
        .await
        .map_err(|_| anyhow!("Infisical-Antwort wurde unterbrochen."))?
    {
        if body.len().saturating_add(chunk.len()) > 2 * 1024 * 1024 {
            return Err(anyhow!("Infisical-Antwort ist zu groß."));
        }
        body.extend_from_slice(&chunk);
    }
    let reply: Reply =
        serde_json::from_slice(&body).map_err(|_| anyhow!("Infisical-Antwort ist ungültig."))?;
    let mut value = None;
    for mut entry in reply
        .secrets
        .into_iter()
        .chain(reply.imports.into_iter().flat_map(|i| i.secrets))
    {
        if entry.name == config.database_secret {
            if value.is_some() || entry.value.trim().is_empty() {
                return Err(anyhow!(
                    "Datenbankzugang fehlt oder ist mehrfach definiert."
                ));
            }
            value = Some(Zeroizing::new(std::mem::take(&mut entry.value)));
        }
    }
    value.ok_or_else(|| anyhow!("Datenbankzugang fehlt in Infisical."))
}
