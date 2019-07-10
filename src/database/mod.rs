mod articles;

use crate::database::articles::ArticleQuerier;
use crate::models::Article;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::*;
use std::sync::Mutex;

pub struct Database(Mutex<Pool<ConnectionManager<PgConnection>>>);

impl Database {
    pub fn new() -> Database {
        dotenv::dotenv().ok();

        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
        let manager = ConnectionManager::new(database_url);
        let pool = r2d2::Pool::builder().build(manager).unwrap();
        Database(Mutex::new(pool))
    }

    pub fn articles(
        &self,
        conn: PooledConnection<ConnectionManager<PgConnection>>,
    ) -> impl Querier<Article> {
        ArticleQuerier::new(conn)
    }

    pub fn get_conn(&self) -> PooledConnection<ConnectionManager<PgConnection>> {
        self.0.lock().unwrap().get().unwrap()
    }
}

pub trait Querier<T> {
    fn find_all(&self) -> diesel::result::QueryResult<Vec<T>>;
    fn get(&self, id: i64) -> diesel::result::QueryResult<T>;
}
