use crate::model;
use cursive_table_view::TableViewItem;
use std::cmp::Ordering;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum CommonColumn {
    ID,
    Login,
    Name,
    Email,
    Description,
    Price,
    Unit,
    CreatedAt,
    UpdatedAt,
    Amount,
    Total,
}

const DATE_FORMAT: &str = "%d/%m/%Y %H:%M:%S";

impl TableViewItem<CommonColumn> for model::user::User {
    fn to_column(&self, column: CommonColumn) -> String {
        match column {
            CommonColumn::ID => self.id.to_string(),
            CommonColumn::Login => self.login.clone(),
            CommonColumn::Name => self.name.clone(),
            CommonColumn::Email => self.email.clone().unwrap_or_default(),
            CommonColumn::CreatedAt => self.created_at.format(DATE_FORMAT).to_string(),
            CommonColumn::UpdatedAt => self.updated_at.format(DATE_FORMAT).to_string(),
            _ => String::new(),
        }
    }

    fn cmp(&self, other: &Self, column: CommonColumn) -> std::cmp::Ordering
    where
        Self: Sized,
    {
        match column {
            CommonColumn::ID => self.id.cmp(&other.id),
            CommonColumn::Login => self.login.cmp(&other.login),
            CommonColumn::Name => self.name.cmp(&other.name),
            CommonColumn::Email => self.email.cmp(&other.email),
            CommonColumn::CreatedAt => self.created_at.cmp(&other.created_at),
            CommonColumn::UpdatedAt => self.created_at.cmp(&other.updated_at),
            _ => Ordering::Equal,
        }
    }
}

impl TableViewItem<CommonColumn> for model::products::Product {
    fn to_column(&self, column: CommonColumn) -> String {
        match column {
            CommonColumn::ID => self.id.to_string(),
            CommonColumn::Description => self.description.clone(),
            CommonColumn::Price => format!("{:.2}", self.price),
            CommonColumn::Unit => self.unit.clone(),
            CommonColumn::CreatedAt => self.created_at.format(DATE_FORMAT).to_string(),
            CommonColumn::UpdatedAt => self.updated_at.format(DATE_FORMAT).to_string(),
            _ => String::new(),
        }
    }

    fn cmp(&self, other: &Self, column: CommonColumn) -> std::cmp::Ordering
    where
        Self: Sized,
    {
        match column {
            CommonColumn::ID => self.id.cmp(&other.id),
            CommonColumn::Description => self.description.cmp(&other.description),
            CommonColumn::Price => self.price.cmp(&other.price),
            CommonColumn::Unit => self.unit.cmp(&other.unit),
            CommonColumn::CreatedAt => self.created_at.cmp(&other.created_at),
            CommonColumn::UpdatedAt => self.created_at.cmp(&other.updated_at),
            _ => Ordering::Equal,
        }
    }
}

impl TableViewItem<CommonColumn> for model::sales::SaleItem {
    fn to_column(&self, column: CommonColumn) -> String {
        match column {
            CommonColumn::ID => format!("{}", self.id),
            CommonColumn::Description => self.product.description.clone(),
            CommonColumn::Unit => self.product.unit.clone(),
            CommonColumn::Amount => format!("{:.3}", self.amount),
            CommonColumn::Total => format!("{:.2}", self.product.price * self.amount),
            _ => String::new(),
        }
    }

    fn cmp(&self, other: &Self, column: CommonColumn) -> std::cmp::Ordering
    where
        Self: Sized,
    {
        match column {
            CommonColumn::ID => self.id.cmp(&other.id),
            CommonColumn::Description => self.product.description.cmp(&other.product.description),
            CommonColumn::Unit => self.product.unit.cmp(&other.product.unit),
            CommonColumn::Amount => self.amount.cmp(&other.amount),
            _ => Ordering::Equal,
        }
    }
}
