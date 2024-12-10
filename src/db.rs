use rusqlite::{Connection, Result};

pub fn connection_db() -> Result<Connection> {
    Connection::open("todo_list.db")
}

pub fn initialize_database() {
   
    tokio::task::block_in_place(|| {
        let conn = connection_db().unwrap();

        let query = "
            CREATE TABLE IF NOT EXISTS todoList(
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                description TEXT NOT NULL,
                status VARCHAR(8) NOT NULL,
                created_at DATETIME NOT NULL,
                updated_at DATETIME NOT NULL
            );
        ";


        conn.execute(query, []).unwrap();
    });
}
