use crate::model;
use cursive_table_view::TableViewItem;
use std::cmp::Ordering;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum CommonColumn {
    ID,
    Login,
    Name,
    Email,
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
