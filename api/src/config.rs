use anyhow::Result;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Config {
    pub database_url: String,
    pub port: u16,
    pub storage_path: String,
    pub session_secret: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        #[cfg(not(test))]
        dotenvy::dotenv().ok();

        let database_url = std::env::var("DATABASE_URL")
            .map_err(|_| anyhow::anyhow!("DATABASE_URL must be set"))?;

        let port = std::env::var("PORT")
            .unwrap_or_else(|_| "9069".to_string())
            .parse::<u16>()
            .map_err(|_| anyhow::anyhow!("PORT must be a valid port number"))?;

        let storage_path = std::env::var("STORAGE_PATH")
            .unwrap_or_else(|_| "./uploads".to_string());

        let session_secret = std::env::var("SESSION_SECRET")
            .map_err(|_| anyhow::anyhow!("SESSION_SECRET must be set"))?;

        Ok(Config {
            database_url,
            port,
            storage_path,
            session_secret,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_env_with_all_vars() {
        temp_env::with_vars(
            [
                ("DATABASE_URL", Some("postgres://test:test@localhost/test")),
                ("SESSION_SECRET", Some("test-secret-key")),
                ("PORT", Some("3000")),
                ("STORAGE_PATH", Some("/tmp/uploads")),
            ],
            || {
                let config = Config::from_env().unwrap();
                assert_eq!(config.database_url, "postgres://test:test@localhost/test");
                assert_eq!(config.session_secret, "test-secret-key");
                assert_eq!(config.port, 3000);
                assert_eq!(config.storage_path, "/tmp/uploads");
            },
        );
    }

    #[test]
    fn test_from_env_defaults() {
        temp_env::with_vars(
            [
                ("DATABASE_URL", Some("postgres://test:test@localhost/test")),
                ("SESSION_SECRET", Some("test-secret")),
                ("PORT", None),
                ("STORAGE_PATH", None),
            ],
            || {
                let config = Config::from_env().unwrap();
                assert_eq!(config.port, 9069);
                assert_eq!(config.storage_path, "./uploads");
            },
        );
    }

    #[test]
    fn test_from_env_missing_database_url() {
        temp_env::with_vars(
            [
                ("DATABASE_URL", None),
                ("SESSION_SECRET", Some("test-secret")),
            ],
            || {
                let result = Config::from_env();
                assert!(result.is_err());
            },
        );
    }

    #[test]
    fn test_from_env_missing_session_secret() {
        temp_env::with_vars(
            [
                ("DATABASE_URL", Some("postgres://test:test@localhost/test")),
                ("SESSION_SECRET", None),
            ],
            || {
                let result = Config::from_env();
                assert!(result.is_err());
            },
        );
    }

    #[test]
    fn test_from_env_invalid_port() {
        temp_env::with_vars(
            [
                ("DATABASE_URL", Some("postgres://test:test@localhost/test")),
                ("SESSION_SECRET", Some("test-secret")),
                ("PORT", Some("not_a_number")),
            ],
            || {
                let result = Config::from_env();
                assert!(result.is_err());
            },
        );
    }
}
