use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Receipt {
    pub id: Uuid,
    pub user_id: Uuid,
    pub card_id: Option<Uuid>,
    pub total_amount: i64,
    pub photo_path: Option<String>,
    pub notes: Option<String>,
    pub purchased_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateReceiptRequest {
    pub card_id: Option<Uuid>,
    pub total_amount: i64,
    pub notes: Option<String>,
    pub purchased_at: String, // ISO 8601 datetime string
    pub category_ids: Option<Vec<Uuid>>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateReceiptRequest {
    pub card_id: Option<Option<Uuid>>,  // None = don't change, Some(None) = clear, Some(Some(id)) = set
    pub total_amount: Option<i64>,
    pub notes: Option<Option<String>>,
    pub purchased_at: Option<String>,
    pub category_ids: Option<Vec<Uuid>>,
}

#[derive(Debug, Serialize)]
pub struct ReceiptResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub card_id: Option<Uuid>,
    pub total_amount: i64,
    pub photo_path: Option<String>,
    pub notes: Option<String>,
    pub purchased_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub categories: Vec<super::category::CategoryResponse>,
}

impl Receipt {
    pub async fn create(
        pool: &PgPool,
        user_id: Uuid,
        card_id: Option<Uuid>,
        total_amount: i64,
        photo_path: Option<&str>,
        notes: Option<&str>,
        purchased_at: DateTime<Utc>,
    ) -> Result<Receipt, sqlx::Error> {
        sqlx::query_as::<_, Receipt>(
            "INSERT INTO receipts (user_id, card_id, total_amount, photo_path, notes, purchased_at) \
             VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
        )
        .bind(user_id)
        .bind(card_id)
        .bind(total_amount)
        .bind(photo_path)
        .bind(notes)
        .bind(purchased_at)
        .fetch_one(pool)
        .await
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Receipt>, sqlx::Error> {
        sqlx::query_as::<_, Receipt>("SELECT * FROM receipts WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    pub async fn find_all(pool: &PgPool) -> Result<Vec<Receipt>, sqlx::Error> {
        sqlx::query_as::<_, Receipt>("SELECT * FROM receipts ORDER BY purchased_at DESC")
            .fetch_all(pool)
            .await
    }

    pub async fn update(
        pool: &PgPool,
        id: Uuid,
        card_id: Option<Option<Uuid>>,
        total_amount: Option<i64>,
        notes: Option<Option<&str>>,
        purchased_at: Option<DateTime<Utc>>,
    ) -> Result<Option<Receipt>, sqlx::Error> {
        // Build dynamic update - use COALESCE pattern for optional fields
        sqlx::query_as::<_, Receipt>(
            "UPDATE receipts SET \
             card_id = CASE WHEN $1 THEN $2 ELSE card_id END, \
             total_amount = COALESCE($3, total_amount), \
             notes = CASE WHEN $4 THEN $5 ELSE notes END, \
             purchased_at = COALESCE($6, purchased_at) \
             WHERE id = $7 RETURNING *",
        )
        .bind(card_id.is_some()) // $1: should update card_id?
        .bind(card_id.flatten()) // $2: new card_id value (may be null)
        .bind(total_amount) // $3
        .bind(notes.is_some()) // $4: should update notes?
        .bind(notes.flatten()) // $5: new notes value
        .bind(purchased_at) // $6
        .bind(id) // $7
        .fetch_optional(pool)
        .await
    }

    pub async fn set_photo_path(
        pool: &PgPool,
        id: Uuid,
        photo_path: &str,
    ) -> Result<Option<Receipt>, sqlx::Error> {
        sqlx::query_as::<_, Receipt>(
            "UPDATE receipts SET photo_path = $1 WHERE id = $2 RETURNING *",
        )
        .bind(photo_path)
        .bind(id)
        .fetch_optional(pool)
        .await
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM receipts WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    pub async fn set_categories(
        pool: &PgPool,
        receipt_id: Uuid,
        category_ids: &[Uuid],
    ) -> Result<(), sqlx::Error> {
        // Delete existing associations
        sqlx::query("DELETE FROM receipt_categories WHERE receipt_id = $1")
            .bind(receipt_id)
            .execute(pool)
            .await?;

        // Insert new associations
        for cat_id in category_ids {
            sqlx::query(
                "INSERT INTO receipt_categories (receipt_id, category_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
            )
            .bind(receipt_id)
            .bind(cat_id)
            .execute(pool)
            .await?;
        }

        Ok(())
    }

    pub async fn get_categories(
        pool: &PgPool,
        receipt_id: Uuid,
    ) -> Result<Vec<super::category::Category>, sqlx::Error> {
        sqlx::query_as::<_, super::category::Category>(
            "SELECT c.* FROM categories c \
             JOIN receipt_categories rc ON rc.category_id = c.id \
             WHERE rc.receipt_id = $1 \
             ORDER BY c.name",
        )
        .bind(receipt_id)
        .fetch_all(pool)
        .await
    }
}
