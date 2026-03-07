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
