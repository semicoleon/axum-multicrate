use anyhow::Result;
use interface::Item;
use rusqlite::{named_params, Connection};

/// Load items for a given user ID
pub fn load_item_user_id(db: &mut Connection, user_id: i64) -> Result<Vec<Item>> {
    let mut stmt = db.prepare("SELECT * FROM Item WHERE user_id = :user_id")?;

    let mapped = stmt.query_map(named_params! {":user_id": user_id}, |i| {
        Ok(Item {
            id: i.get("id")?,
            name: i.get("name")?,
            user_id: i.get("user_id")?,
        })
    })?;

    Ok(mapped.collect::<Result<_, _>>()?)
}

/// Perform db migrations for this crate's data.
pub fn migrate(connection: &mut Connection) -> Result<()> {
    connection.execute(
        "\
CREATE TABLE IF NOT EXISTS Item (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    user_id INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES User(id)
)",
        (),
    )?;
    Ok(())
}
