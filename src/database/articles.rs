use crate::database::Querier;
use crate::models::Article;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::*;

pub struct ArticleQuerier(PooledConnection<ConnectionManager<PgConnection>>);

impl ArticleQuerier {
    pub fn new(conn: PooledConnection<ConnectionManager<PgConnection>>) -> ArticleQuerier {
        ArticleQuerier(conn)
    }
}

use crate::schema::article::dsl;
use crate::schema::article::dsl::article as all_articles;
impl Querier<Article> for ArticleQuerier {
    fn find_all(&self) -> diesel::result::QueryResult<Vec<Article>> {
        let conn = &self.0;
        all_articles.load(conn)
    }

    fn get(&self, id: i64) -> diesel::result::QueryResult<Article> {
        let conn = &self.0;
        all_articles.filter(dsl::id.eq(id)).first(conn)
    }
}
