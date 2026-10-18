use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::DomainError;

#[derive(Debug, Clone)]
pub struct NewProductionPerformance {
    pub source_system: String,
    pub order_external_id: String,
    pub good_units: u64,
    pub scrap_units: u64,
    pub run_minutes: f64,
    pub recorded_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionPerformance {
    pub id: Uuid,
    pub source_system: String,
    pub order_external_id: String,
    pub good_units: u64,
    pub scrap_units: u64,
    pub run_minutes: f64,
    pub recorded_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl ProductionPerformance {
    pub fn create(input: NewProductionPerformance) -> Result<Self, DomainError> {
        if input.good_units + input.scrap_units == 0 {
            return Err(DomainError::NoProducedUnits);
        }
        if !input.run_minutes.is_finite() || input.run_minutes < 0.0 {
            return Err(DomainError::InvalidRunMinutes);
        }

        Ok(Self {
            id: Uuid::new_v4(),
            source_system: input.source_system,
            order_external_id: input.order_external_id,
            good_units: input.good_units,
            scrap_units: input.scrap_units,
            run_minutes: input.run_minutes,
            recorded_at: input.recorded_at,
            created_at: Utc::now(),
        })
    }

    pub fn yield_pct(&self) -> f64 {
        let total = self.good_units + self.scrap_units;
        if total == 0 {
            0.0
        } else {
            self.good_units as f64 / total as f64
        }
    }
}