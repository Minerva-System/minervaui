use chrono::{DateTime, Utc};
use uuid::Uuid;

use cursive_table_view::TableViewItem;

#[derive(Clone, Debug, Default)]
pub struct User {
    pub id: Uuid,
    pub login: String,
    pub name: String,
    pub email: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: DateTime<Utc>,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum UserListColumn {
    ID,
    Login,
    Name,
    Email,
}

impl UserListColumn {
    fn as_str(&self) -> &str {
        match *self {
            UserListColumn::ID => "ID",
            UserListColumn::Login => "Login",
            UserListColumn::Name => "Name",
            UserListColumn::Email => "E-mail",
        }
    }
}

impl TableViewItem<UserListColumn> for User {
    fn to_column(&self, column: UserListColumn) -> String {
        match column {
            UserListColumn::ID => self.id.to_string(),
            UserListColumn::Login => self.login.clone(),
            UserListColumn::Name => self.name.clone(),
            UserListColumn::Email => self.email.clone().unwrap_or(String::new()),
        }
    }

    fn cmp(&self, other: &Self, column: UserListColumn) -> std::cmp::Ordering
    where
        Self: Sized,
    {
        match column {
            UserListColumn::ID => self.id.cmp(&other.id),
            UserListColumn::Login => self.login.cmp(&other.login),
            UserListColumn::Name => self.name.cmp(&other.name),
            UserListColumn::Email => self.email.cmp(&other.email),
        }
    }
}
