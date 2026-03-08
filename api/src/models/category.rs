use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Category {
    pub id: Uuid,
    pub name: String,
    pub color: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCategoryRequest {
    pub name: String,
    pub color: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCategoryRequest {
    pub name: Option<String>,
    pub color: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct CategoryResponse {
    pub id: Uuid,
    pub name: String,
    pub color: String,
    pub created_at: DateTime<Utc>,
}

impl From<Category> for CategoryResponse {
    fn from(cat: Category) -> Self {
        CategoryResponse {
            id: cat.id,
            name: cat.name,
            color: cat.color,
            created_at: cat.created_at,
        }
    }
}

impl Category {
    pub fn validate_name(name: &str) -> Result<(), String> {
        if name.trim().is_empty() {
            return Err("name is required".to_string());
        }
        Ok(())
    }

    pub fn validate_color(color: &str) -> Result<(), String> {
        if !color.starts_with('#') || color.len() != 7 {
            return Err("color must be a hex color code (e.g. #ff0000)".to_string());
        }
        if !color[1..].chars().all(|c| c.is_ascii_hexdigit()) {
            return Err("color must be a valid hex color code".to_string());
        }
        Ok(())
    }

    pub async fn create(pool: &PgPool, name: &str, color: &str) -> Result<Category, sqlx::Error> {
        sqlx::query_as::<_, Category>(
            "INSERT INTO categories (name, color) VALUES ($1, $2) RETURNING *",
        )
        .bind(name)
        .bind(color)
        .fetch_one(pool)
        .await
    }

    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Category>, sqlx::Error> {
        sqlx::query_as::<_, Category>("SELECT * FROM categories WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    pub async fn find_all(pool: &PgPool) -> Result<Vec<Category>, sqlx::Error> {
        sqlx::query_as::<_, Category>("SELECT * FROM categories ORDER BY name")
            .fetch_all(pool)
            .await
    }

    pub async fn update(
        pool: &PgPool,
        id: Uuid,
        name: Option<&str>,
        color: Option<&str>,
    ) -> Result<Option<Category>, sqlx::Error> {
        sqlx::query_as::<_, Category>(
            "UPDATE categories SET name = COALESCE($1, name), color = COALESCE($2, color) WHERE id = $3 RETURNING *",
        )
        .bind(name)
        .bind(color)
        .bind(id)
        .fetch_optional(pool)
        .await
    }

    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM categories WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_name_valid() {
        assert!(Category::validate_name("Groceries").is_ok());
    }

    #[test]
    fn test_validate_name_empty() {
        assert!(Category::validate_name("").is_err());
        assert!(Category::validate_name("   ").is_err());
    }

    #[test]
    fn test_validate_color_valid() {
        assert!(Category::validate_color("#ff0000").is_ok());
        assert!(Category::validate_color("#6b7280").is_ok());
        assert!(Category::validate_color("#AABBCC").is_ok());
    }

    #[test]
    fn test_validate_color_invalid() {
        assert!(Category::validate_color("ff0000").is_err()); // missing #
        assert!(Category::validate_color("#fff").is_err()); // too short
        assert!(Category::validate_color("#gggggg").is_err()); // invalid hex
        assert!(Category::validate_color("").is_err());
    }
}
