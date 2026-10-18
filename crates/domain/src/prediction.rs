use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{error::DomainError, order::OrderId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityPrediction {
    pub order_id: OrderId,
    pub defect_probability: f64,
    pub model_version: String,
    pub features_hash: String,
    pub created_at: DateTime<Utc>,
}

impl QualityPrediction {
    pub fn new(
        order_id: OrderId,
        defect_probability: f64,
        model_version: String,
        features_hash: String,
    ) -> Result<Self, DomainError> {
        if !(0.0..=1.0).contains(&defect_probability) {
            return Err(DomainError::InvalidProbability(defect_probability));
        }
        Ok(Self {
            order_id,
            defect_probability,
            model_version,
            features_hash,
            created_at: Utc::now(),
        })
    }

    pub fn is_at_risk(&self, threshold: f64) -> bool {
        self.defect_probability >= threshold
    }
}