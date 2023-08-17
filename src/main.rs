use std::sync::Arc;

use anyhow::Result;
use axum::Router;
use interface::{ItemModule, Module, UserModule};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{named_params, Connection};

pub struct Modules(Pool<SqliteConnectionManager>);

impl Module for Modules {
    // I just stuck the pool in here for simplicity. You shouldn't use the pool inside the impls otherwise you might end up using different database connections for operations that require being made on the same connection.
    fn pool(&self) -> &r2d2::Pool<r2d2_sqlite::SqliteConnectionManager> {
        &self.0
    }
}

impl UserModule for Modules {
    fn load_users(
        &self,
        db: &mut Connection,
        item: &impl ItemModule,
    ) -> Result<Vec<interface::User>> {
        users::data::load_users(db, item)
    }
}

impl ItemModule for Modules {
    fn load_items_user_id(
        &self,
        db: &mut Connection,
        user_id: i64,
    ) -> Result<Vec<interface::Item>> {
        item::data::load_item_user_id(db, user_id)
    }
}

#[tokio::main]
async fn main() {
    let manager = SqliteConnectionManager::file("db.sqlite3")
        .with_init(|conn| conn.pragma_update(None, "foreign_keys", "ON"));
    let pool = Pool::new(manager).unwrap();

    let mut conn = pool.get().unwrap();

    users::data::migrate(&mut conn).unwrap();
    item::data::migrate(&mut conn).unwrap();

    // Check if we should add test data.
    if conn
        .query_row("SELECT COUNT(id) FROM User", (), |row| row.get::<_, i64>(0))
        .unwrap()
        <= 0
    {
        create_test_data(&mut conn).unwrap();
    }

    let app = Router::new()
        .nest("/user", users::routes())
        .nest("/item", item::routes())
        .with_state(Arc::new(Modules(pool)));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn create_test_data(conn: &mut Connection) -> Result<()> {
    {
        let mut stmt = conn.prepare("INSERT INTO User (name) VALUES (:name)")?;

        for name in ["First User", "Second User", "Foo Bar", "Whatever"] {
            stmt.execute(named_params! {":name": name}).unwrap();
        }
    }

    {
        let mut stmt = conn.prepare("INSERT INTO Item (name, user_id) VALUES (:name, :user_id)")?;

        let items = [
            vec!["Thing 1", "Thing 2"],
            vec!["Bowl", "Fork"],
            vec!["Nothing"],
        ];

        for (idx, items) in items.into_iter().enumerate() {
            for item in items {
                stmt.execute(named_params! {":name": item, ":user_id": idx + 1})?;
            }
        }
    }

    Ok(())
}
