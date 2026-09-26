use super::{hardening, OpenAiCompatibleProvider, ProviderError};
use brain_contracts::{
    AuthorizedContext, EmbeddingIdentity, EmbeddingOutput, EmbeddingProviderPort, PortError,
};
use serde::Deserialize;
#[derive(Deserialize)]
struct Response {
    model: String,
    data: Vec<Vector>,
    usage: EmbeddingUsage,
}
#[derive(Deserialize)]
struct Vector {
    index: usize,
    embedding: Vec<f64>,
}
#[derive(Deserialize)]
struct EmbeddingUsage {
    prompt_tokens: u64,
}
impl EmbeddingProviderPort for OpenAiCompatibleProvider {
    fn embed(
        &self,
        inputs: &[String],
        identity: &EmbeddingIdentity,
        context: &AuthorizedContext,
    ) -> Result<EmbeddingOutput, PortError> {
        identity.validate()?;
        if identity.model != self.config.model
            || inputs.is_empty()
            || inputs.len() > 128
            || inputs
                .iter()
                .any(|s| s.trim().is_empty() || s.len() > 32768)
            || !context.principal.provider_egress.contains("public")
        {
            return Err(PortError::InvalidResponse(
                "embedding identity, input or egress denied".into(),
            ));
        }
        let result=self.with_circuit(|| {
            hardening::validate_endpoint(&self.config)?;
            let mut payload=serde_json::json!({"model":identity.model,"input":inputs,"dimensions":identity.dimension,"encoding_format":"float"});
            let (bytes,charge)=self.transport_json("embeddings",&mut payload,context,false)?;
            let response:Response=serde_json::from_slice(&bytes).map_err(|_|ProviderError::InvalidResponse("invalid embedding response schema".into()))?;
            if response.model!=identity.model || response.data.len()!=inputs.len() {return Err(ProviderError::InvalidResponse("embedding model or count mismatch".into()));}
            let mut ordered=vec![None;inputs.len()];
            for item in response.data {
                if item.index>=ordered.len() || ordered[item.index].is_some() {return Err(ProviderError::InvalidResponse("embedding index missing or duplicated".into()));}
                identity.validate_vector(&item.embedding).map_err(|_|ProviderError::InvalidResponse("embedding vector invalid".into()))?;
                ordered[item.index]=Some(item.embedding);
            }
            let vectors=ordered.into_iter().collect::<Option<Vec<_>>>().ok_or_else(||ProviderError::InvalidResponse("embedding index missing".into()))?;
            let usage=self.charged_usage(context,charge,response.usage.prompt_tokens,0)?;
            Ok(EmbeddingOutput {identity:identity.clone(),vectors,usage})
        });
        result.map_err(|error| match error {
            ProviderError::BudgetExceeded => PortError::BudgetExceeded,
            ProviderError::InvalidResponse(message) => PortError::InvalidResponse(message),
            _ => PortError::Unavailable("embedding transport failed".into()),
        })
    }
}
