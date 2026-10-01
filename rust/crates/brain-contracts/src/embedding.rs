//! Versioned embedding boundary; equal dimensions do not imply compatible models.
use crate::{AuthorizedContext, PortError, Usage};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EmbeddingIdentity {
    pub provider: String,
    pub model: String,
    pub revision: String,
    pub dimension: usize,
    pub pooling: String,
    pub query_prefix: String,
    pub document_prefix: String,
    pub normalized: bool,
}
impl EmbeddingIdentity {
    pub fn validate(&self) -> Result<(), PortError> {
        if [&self.provider, &self.model, &self.revision, &self.pooling]
            .iter()
            .any(|s| s.trim().is_empty() || s.len() > 512)
            || !(1..=16384).contains(&self.dimension)
            || self.query_prefix.len() > 4096
            || self.document_prefix.len() > 4096
        {
            return Err(PortError::InvalidResponse(
                "invalid embedding identity".into(),
            ));
        }
        Ok(())
    }
    pub fn validate_vector(&self, vector: &[f64]) -> Result<(), PortError> {
        self.validate()?;
        let norm = vector.iter().map(|x| x * x).sum::<f64>().sqrt();
        if vector.len() != self.dimension
            || vector.iter().any(|x| !x.is_finite())
            || !norm.is_finite()
            || norm <= f64::EPSILON
            || (self.normalized && (norm - 1.0).abs() > 0.00001)
        {
            return Err(PortError::InvalidResponse(
                "invalid embedding vector".into(),
            ));
        }
        Ok(())
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EmbeddingOutput {
    pub identity: EmbeddingIdentity,
    pub vectors: Vec<Vec<f64>>,
    pub usage: Usage,
}
pub trait EmbeddingProviderPort: Send + Sync {
    fn embed(
        &self,
        inputs: &[String],
        identity: &EmbeddingIdentity,
        context: &AuthorizedContext,
    ) -> Result<EmbeddingOutput, PortError>;
}

#[cfg(test)]
mod tests {
    use super::*;
    fn identity() -> EmbeddingIdentity {
        EmbeddingIdentity {
            provider: "fixture".into(),
            model: "fixture".into(),
            revision: "r1".into(),
            dimension: 2,
            pooling: "mean".into(),
            query_prefix: "query: ".into(),
            document_prefix: "passage: ".into(),
            normalized: true,
        }
    }
    #[test]
    fn equal_dimension_different_model_is_incompatible() {
        let a = identity();
        let mut b = a.clone();
        b.revision = "r2".into();
        assert_ne!(a, b);
    }
    #[test]
    fn rejects_missing_nonfinite_zero_and_unnormalized_vectors() {
        let model = identity();
        assert!(model.validate_vector(&[1.0, 0.0]).is_ok());
        for vector in [
            vec![],
            vec![1.0],
            vec![0.0, 0.0],
            vec![f64::NAN, 0.0],
            vec![1.0, 1.0],
        ] {
            assert!(model.validate_vector(&vector).is_err());
        }
    }
}
