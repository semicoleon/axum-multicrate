use anyhow::Result;
use interface::{ItemModule, User};
use rusqlite::Connection;

/// Load all users
pub fn load_users(db: &mut Connection, item: &impl ItemModule) -> Result<Vec<User>> {
    let mut users: Vec<User> = {
        let mut stmt = db.prepare("SELECT * FROM User")?;

        let iter = stmt.query_map((), |u| {
            Ok(User {
                id: u.get("id")?,
                name: u.get("name")?,
                items: Vec::new(),
            })
        })?;

        iter.collect::<Result<_, _>>()?
    };

    for user in &mut users {
        user.items = item.load_items_user_id(db, user.id)?;
    }

    Ok(users)
}

/// Perform db migrations for this crate's data.
pub fn migrate(connection: &mut Connection) -> Result<()> {
    connection.execute(
        "\
CREATE TABLE IF NOT EXISTS User (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL
)",
        (),
    )?;
    Ok(())
}
