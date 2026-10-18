use thiserror::Error;

use crate::order::OrderStatus;

#[derive(Debug, Error, Clone, PartialEq)]
pub enum DomainError {
    #[error("quantity must be a positive finite number, got {0}")]
    InvalidQuantity(f64),

    #[error("{field} must not be empty")]
    EmptyField { field: &'static str },

    #[error("planned_end must not be before planned_start")]
    InvalidSchedule,

    #[error("invalid status transition from {from} to {to}")]
    InvalidTransition { from: OrderStatus, to: OrderStatus },

    #[error("good_units + scrap_units must be greater than zero")]
    NoProducedUnits,

    #[error("run_minutes must be non-negative")]
    InvalidRunMinutes,

    #[error("defect probability must be between 0 and 1, got {0}")]
    InvalidProbability(f64),
}