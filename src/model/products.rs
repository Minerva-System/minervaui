use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Product {
    pub id: uuid::Uuid,
    pub description: String,
    #[serde(deserialize_with = "rust_decimal::serde::arbitrary_precision::deserialize")]
    pub price: Decimal,
    pub unit: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewProduct {
    pub description: String,
    pub price: Decimal,
    pub unit: String,
}
