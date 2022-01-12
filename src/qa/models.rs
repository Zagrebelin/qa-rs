use jelly::accounts::{OneTimeUseTokenGenerator, User};
use jelly::chrono::{DateTime, Utc};
use jelly::djangohashers::{check_password, make_password};
use jelly::error::Error;
use jelly::serde::{Deserialize, Serialize};
use jelly::sqlx::{self, postgres::PgPool, types::Json, FromRow, Row};
use jelly::sqlx::{};
use jelly::sqlx::postgres::PgRow;

#[derive(Serialize, Deserialize)]
pub struct Question {
    pub id: i32,
    pub header: String,
    pub body: String,
    pub created: DateTime<Utc>,
    pub updated: Option<DateTime<Utc>>,
}


impl  Question {
    pub async fn find_all(pool: &PgPool) -> Result<Vec<Question>, Error> {
        Ok(sqlx::query(
            "SELECT id, header, body, created, updated FROM questions ORDER BY created"
        )
            .fetch_all(pool)
            .await?
            .into_iter()
            .map(|rec:PgRow| Question {
                id: rec.get("id"),
                header: rec.get("header"),
                body: rec.get("body"),
                created: rec.get("created"),
                updated: rec.get("updated")
            })
            .collect()
        )
    }

    pub async fn get_by_id(id: i32, pool: &PgPool) -> Result<Question, Error> {
        let q = sqlx::query_as_unchecked!(
            Question,
            "SELECT id, header, body, created, updated FROM questions WHERE id=$1",
            id
        ).fetch_one(pool)
            .await?;

        Ok(q)
    }

}
