//! Private Textdaten werden an den tatsächlich ausgewählten zentralen Provider gebunden.
use brain_contracts::{PortError, Principal};

#[derive(Clone, Copy, Debug)]
pub enum TextProviderTarget<'a> {
    Api { endpoint: &'a str, model: &'a str },
    CodexSubscription { model: &'a str },
}
/// Modellfreigabe und belegte lokale Verarbeitung sind getrennte Voraussetzungen.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PrivateProcessingDecision {
    GrantedApi,
    CliIsolationRequired,
}
pub fn approved_private_api(endpoint: &str, model: &str) -> bool {
    reqwest::Url::parse(endpoint).is_ok_and(|url| {
        url.scheme() == "https"
            && url.host_str() == Some("api.fireworks.ai")
            && url.port_or_known_default() == Some(443)
            && url.username().is_empty()
            && url.password().is_none()
            && url.query().is_none()
            && url.fragment().is_none()
            && model.starts_with("accounts/fireworks/models/deepseek-")
            && model.ends_with("-flash")
    })
}
pub fn private_processing_requirements(
    principal: &Principal,
    target: TextProviderTarget<'_>,
) -> Result<PrivateProcessingDecision, PortError> {
    if !principal.provider_egress.contains("private_dm") {
        return Err(PortError::PermissionDenied(
            "Private Weitergabe ist nicht freigegeben".into(),
        ));
    }
    match target {
        TextProviderTarget::Api { endpoint, model } if approved_private_api(endpoint, model) => {
            Ok(PrivateProcessingDecision::GrantedApi)
        }
        TextProviderTarget::CodexSubscription {
            model: "gpt-6-luna",
        } => {
            // Die ausdrückliche Modellfreigabe reicht nicht als Beleg für sichere CLI-Caches.
            // Der gemeinsame Runner muss diese Entscheidung vor seinem Start weiter prüfen.
            Ok(PrivateProcessingDecision::CliIsolationRequired)
        }
        _ => Err(PortError::PermissionDenied(
            "Dieser private Antwortpfad ist nicht freigegeben".into(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;
    fn principal(allow: bool) -> Principal {
        Principal {
            actor_id: "synthetic-guide".into(),
            channel: "discord".into(),
            scopes: BTreeSet::from(["guide".into()]),
            provider_egress: if allow {
                BTreeSet::from(["public".into(), "private_dm".into()])
            } else {
                BTreeSet::from(["public".into()])
            },
        }
    }
    #[test]
    fn ausdrueckliche_luna_freigabe_ueberspringt_keine_isolation() {
        assert_eq!(
            private_processing_requirements(
                &principal(true),
                TextProviderTarget::CodexSubscription {
                    model: "gpt-6-luna"
                }
            )
            .unwrap(),
            PrivateProcessingDecision::CliIsolationRequired
        );
        assert!(private_processing_requirements(
            &principal(false),
            TextProviderTarget::CodexSubscription {
                model: "gpt-6-luna"
            }
        )
        .is_err());
        assert!(private_processing_requirements(
            &principal(true),
            TextProviderTarget::CodexSubscription {
                model: "unapproved"
            }
        )
        .is_err());
    }
    #[test]
    fn api_freigabe_ist_an_tatsaechlichen_anbieter_und_flash_gebunden() {
        let model = "accounts/fireworks/models/deepseek-v4p1-flash";
        assert_eq!(
            private_processing_requirements(
                &principal(true),
                TextProviderTarget::Api {
                    endpoint: "https://api.fireworks.ai/inference/v1",
                    model
                }
            )
            .unwrap(),
            PrivateProcessingDecision::GrantedApi
        );
        for endpoint in [
            "http://api.fireworks.ai/v1",
            "https://api.fireworks.ai:444/v1",
            "https://api.fireworks.ai.evil.example/v1",
            "https://other.example/v1",
            "https://user@api.fireworks.ai/v1",
        ] {
            assert!(private_processing_requirements(
                &principal(true),
                TextProviderTarget::Api { endpoint, model }
            )
            .is_err());
        }
        assert!(private_processing_requirements(
            &principal(true),
            TextProviderTarget::Api {
                endpoint: "https://api.fireworks.ai/inference/v1",
                model: "accounts/fireworks/models/deepseek-v4-pro"
            }
        )
        .is_err());
    }
}
