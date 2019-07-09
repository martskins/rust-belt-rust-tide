use std::env;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

pub struct Database {
    pub conn: Pool<ConnectionManager<PgConnection>>,
}

impl Database {
    pub fn establish_connection() -> Database {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
        Database {
            conn: Pool::new(ConnectionManager::new(&database_url))
                .expect(&format!("Error connecting to {}", database_url)),
        }
    }
}
