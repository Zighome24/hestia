use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Card {
    pub id: Uuid,
    pub user_id: Uuid,
    pub nickname: String,
    pub last_four: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCardRequest {
    pub nickname: String,
    pub last_four: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCardRequest {
    pub nickname: Option<String>,
    pub last_four: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CardResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub nickname: String,
    pub last_four: String,
    pub created_at: DateTime<Utc>,
}

impl From<Card> for CardResponse {
    fn from(card: Card) -> Self {
        CardResponse {
            id: card.id,
            user_id: card.user_id,
            nickname: card.nickname,
            last_four: card.last_four,
            created_at: card.created_at,
        }
    }
}

impl Card {
    pub fn validate_last_four(last_four: &str) -> Result<(), String> {
        if last_four.len() != 4 {
            return Err("last_four must be exactly 4 characters".to_string());
        }
        if !last_four.chars().all(|c| c.is_ascii_digit()) {
            return Err("last_four must contain only digits".to_string());
        }
        Ok(())
    }

    pub fn validate_nickname(nickname: &str) -> Result<(), String> {
        if nickname.trim().is_empty() {
            return Err("nickname is required".to_string());
        }
        Ok(())
    }

    pub async fn create(
        pool: &PgPool,
        user_id: Uuid,
        nickname: &str,
        last_four: &str,
    ) -> Result<Card, sqlx::Error> {
        sqlx::query_as::<_, Card>(
            "INSERT INTO cards (user_id, nickname, last_four) VALUES ($1, $2, $3) RETURNING *",
        )
        .bind(user_id)
        .bind(nickname)
        .bind(last_four)
        .fetch_one(pool)
        .await
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Card>, sqlx::Error> {
        sqlx::query_as::<_, Card>("SELECT * FROM cards WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    pub async fn find_by_user(pool: &PgPool, user_id: Uuid) -> Result<Vec<Card>, sqlx::Error> {
        sqlx::query_as::<_, Card>(
            "SELECT * FROM cards WHERE user_id = $1 ORDER BY created_at DESC",
        )
        .bind(user_id)
        .fetch_all(pool)
        .await
    }

    pub async fn update(
        pool: &PgPool,
        id: Uuid,
        user_id: Uuid,
        nickname: Option<&str>,
        last_four: Option<&str>,
    ) -> Result<Option<Card>, sqlx::Error> {
        sqlx::query_as::<_, Card>(
            "UPDATE cards SET nickname = COALESCE($1, nickname), last_four = COALESCE($2, last_four) WHERE id = $3 AND user_id = $4 RETURNING *",
        )
        .bind(nickname)
        .bind(last_four)
        .bind(id)
        .bind(user_id)
        .fetch_optional(pool)
        .await
    }

    pub async fn delete(pool: &PgPool, id: Uuid, user_id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM cards WHERE id = $1 AND user_id = $2")
            .bind(id)
            .bind(user_id)
            .execute(pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_last_four_valid() {
        assert!(Card::validate_last_four("1234").is_ok());
        assert!(Card::validate_last_four("0000").is_ok());
        assert!(Card::validate_last_four("9999").is_ok());
    }

    #[test]
    fn test_validate_last_four_wrong_length() {
        assert!(Card::validate_last_four("123").is_err());
        assert!(Card::validate_last_four("12345").is_err());
        assert!(Card::validate_last_four("").is_err());
    }

    #[test]
    fn test_validate_last_four_non_digits() {
        assert!(Card::validate_last_four("abcd").is_err());
        assert!(Card::validate_last_four("12ab").is_err());
        assert!(Card::validate_last_four("12 4").is_err());
    }

    #[test]
    fn test_validate_nickname_valid() {
        assert!(Card::validate_nickname("Chase Sapphire").is_ok());
        assert!(Card::validate_nickname("a").is_ok());
    }

    #[test]
    fn test_validate_nickname_empty() {
        assert!(Card::validate_nickname("").is_err());
        assert!(Card::validate_nickname("   ").is_err());
    }
}
