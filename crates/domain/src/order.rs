use std::collections::BTreeMap;
use std::str::FromStr;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::DomainError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OrderId(pub Uuid);

impl OrderId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for OrderId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for OrderId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderStatus {
    Planned,
    Released,
    InProgress,
    Completed,
    Cancelled,
}

impl OrderStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Planned => "planned",
            Self::Released => "released",
            Self::InProgress => "in_progress",
            Self::Completed => "completed",
            Self::Cancelled => "cancelled",
        }
    }

    pub fn can_transition_to(self, next: Self) -> bool {
        use OrderStatus::*;
        matches!(
            (self, next),
            (Planned, Released)
                | (Planned, Cancelled)
                | (Released, InProgress)
                | (Released, Cancelled)
                | (InProgress, Completed)
                | (InProgress, Cancelled)
        )
    }
}

impl FromStr for OrderStatus {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "planned" => Ok(Self::Planned),
            "released" => Ok(Self::Released),
            "in_progress" => Ok(Self::InProgress),
            "completed" => Ok(Self::Completed),
            "cancelled" => Ok(Self::Cancelled),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone)]
pub struct NewProductionOrder {
    pub external_id: String,
    pub source_system: String,
    pub site_id: String,
    pub work_center: String,
    pub material_code: String,
    pub quantity: f64,
    pub uom: String,
    pub status: OrderStatus,
    pub planned_start: Option<DateTime<Utc>>,
    pub planned_end: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionOrder {
    pub id: OrderId,
    pub external_id: String,
    pub source_system: String,
    pub site_id: String,
    pub work_center: String,
    pub material_code: String,
    pub quantity: f64,
    pub uom: String,
    pub status: OrderStatus,
    pub planned_start: Option<DateTime<Utc>>,
    pub planned_end: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ProductionOrder {
    pub fn create(input: NewProductionOrder) -> Result<Self, DomainError> {
        if !input.quantity.is_finite() || input.quantity <= 0.0 {
            return Err(DomainError::InvalidQuantity(input.quantity));
        }
        for (field, value) in [
            ("external_id", input.external_id.as_str()),
            ("source_system", input.source_system.as_str()),
            ("site_id", input.site_id.as_str()),
            ("work_center", input.work_center.as_str()),
            ("material_code", input.material_code.as_str()),
            ("uom", input.uom.as_str()),
        ] {
            if value.trim().is_empty() {
                return Err(DomainError::EmptyField { field });
            }
        }
        if let (Some(start), Some(end)) = (input.planned_start, input.planned_end) {
            if end < start {
                return Err(DomainError::InvalidSchedule);
            }
        }

        let now = Utc::now();
        Ok(Self {
            id: OrderId::new(),
            external_id: input.external_id,
            source_system: input.source_system,
            site_id: input.site_id,
            work_center: input.work_center,
            material_code: input.material_code,
            quantity: input.quantity,
            uom: input.uom,
            status: input.status,
            planned_start: input.planned_start,
            planned_end: input.planned_end,
            created_at: now,
            updated_at: now,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardSummary {
    pub total_orders: u64,
    pub by_status: BTreeMap<String, u64>,
    pub at_risk: u64,
    pub average_predicted_defect_rate: Option<f64>,
}