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
}

impl TableViewItem<CommonColumn> for model::user::User {
    fn to_column(&self, column: CommonColumn) -> String {
        match column {
            CommonColumn::ID => self.id.to_string(),
            CommonColumn::Login => self.login.clone(),
            CommonColumn::Name => self.name.clone(),
            CommonColumn::Email => self.email.clone().unwrap_or_default(),
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
            _ => Ordering::Equal,
        }
    }
}
