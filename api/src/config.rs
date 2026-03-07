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
        dotenvy::dotenv().ok();

        let database_url = std::env::var("DATABASE_URL")
            .map_err(|_| anyhow::anyhow!("DATABASE_URL must be set"))?;

        let port = std::env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
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
    use std::env;

    #[test]
    fn test_from_env_with_all_vars() {
        env::set_var("DATABASE_URL", "postgres://test:test@localhost/test");
        env::set_var("SESSION_SECRET", "test-secret-key");
        env::set_var("PORT", "3000");
        env::set_var("STORAGE_PATH", "/tmp/uploads");

        let config = Config::from_env().unwrap();
        assert_eq!(config.database_url, "postgres://test:test@localhost/test");
        assert_eq!(config.session_secret, "test-secret-key");
        assert_eq!(config.port, 3000);
        assert_eq!(config.storage_path, "/tmp/uploads");

        // Cleanup
        env::remove_var("PORT");
        env::remove_var("STORAGE_PATH");
    }

    #[test]
    fn test_from_env_defaults() {
        env::set_var("DATABASE_URL", "postgres://test:test@localhost/test");
        env::set_var("SESSION_SECRET", "test-secret");
        env::remove_var("PORT");
        env::remove_var("STORAGE_PATH");

        let config = Config::from_env().unwrap();
        assert_eq!(config.port, 8080);
        assert_eq!(config.storage_path, "./uploads");
    }

    #[test]
    fn test_from_env_missing_database_url() {
        env::remove_var("DATABASE_URL");
        env::set_var("SESSION_SECRET", "test-secret");

        let result = Config::from_env();
        assert!(result.is_err());
    }

    #[test]
    fn test_from_env_missing_session_secret() {
        env::set_var("DATABASE_URL", "postgres://test:test@localhost/test");
        env::remove_var("SESSION_SECRET");

        let result = Config::from_env();
        assert!(result.is_err());
    }

    #[test]
    fn test_from_env_invalid_port() {
        env::set_var("DATABASE_URL", "postgres://test:test@localhost/test");
        env::set_var("SESSION_SECRET", "test-secret");
        env::set_var("PORT", "not_a_number");

        let result = Config::from_env();
        assert!(result.is_err());

        env::remove_var("PORT");
    }
}
