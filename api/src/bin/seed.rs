//! Seed script to create initial user accounts.
//!
//! Usage: cargo run --bin seed -- --username admin --display-name "Admin User" --password "changeme123"

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use sqlx::postgres::PgPoolOptions;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let args: Vec<String> = env::args().collect();

    let mut username = None;
    let mut display_name = None;
    let mut password = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--username" => {
                i += 1;
                username = Some(args[i].clone());
            }
            "--display-name" => {
                i += 1;
                display_name = Some(args[i].clone());
            }
            "--password" => {
                i += 1;
                password = Some(args[i].clone());
            }
            _ => {
                eprintln!("Unknown argument: {}", args[i]);
                eprintln!(
                    "Usage: seed --username <NAME> --display-name <DISPLAY> --password <PASS>"
                );
                std::process::exit(1);
            }
        }
        i += 1;
    }

    let username = username.unwrap_or_else(|| {
        eprintln!("--username is required");
        std::process::exit(1);
    });
    let display_name = display_name.unwrap_or_else(|| {
        eprintln!("--display-name is required");
        std::process::exit(1);
    });
    let password = password.unwrap_or_else(|| {
        eprintln!("--password is required");
        std::process::exit(1);
    });

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&database_url)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| anyhow::anyhow!("Failed to hash password: {}", e))?
        .to_string();

    let row: (sqlx::types::Uuid,) = sqlx::query_as(
        "INSERT INTO users (username, display_name, password_hash) VALUES ($1, $2, $3) RETURNING id",
    )
    .bind(&username)
    .bind(&display_name)
    .bind(&password_hash)
    .fetch_one(&pool)
    .await?;

    println!("Created user: {} ({})", username, row.0);

    Ok(())
}
