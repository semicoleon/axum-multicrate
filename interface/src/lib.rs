//! This crate contains the definitions for types and traits that need to be shared between multiple service crates in a way that would cause cyclic dependency issues if the service crates contained the definitions.

use anyhow::Result;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::Connection;

#[derive(Debug, Clone)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub items: Vec<Item>,
}

#[derive(Debug, Clone)]
pub struct Item {
    pub id: i64,
    pub name: String,
    pub user_id: i64,
}

/// The base module trait, which allows access to a connection pool.
pub trait Module: Send + Sync + 'static {
    fn pool(&self) -> &Pool<SqliteConnectionManager>;
}

/// The user service crate's trait interface.
pub trait UserModule: Module {
    fn load_users(&self, db: &mut Connection, item: &impl ItemModule) -> Result<Vec<User>>;
}

/// The item service crate's trait interface.
pub trait ItemModule: Module {
    fn load_items_user_id(&self, db: &mut Connection, user_id: i64) -> Result<Vec<Item>>;
}
