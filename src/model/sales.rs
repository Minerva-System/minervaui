use rust_decimal::Decimal;
use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaleItem {
    pub id: u64,
    pub product: super::products::Product,
    pub amount: Decimal,
}
