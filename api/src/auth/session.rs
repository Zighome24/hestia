use tower_sessions::Session;
use uuid::Uuid;

const USER_ID_KEY: &str = "user_id";

pub async fn set_user_id(session: &Session, user_id: Uuid) -> Result<(), tower_sessions::session::Error> {
    session.insert(USER_ID_KEY, user_id.to_string()).await
}

pub async fn get_user_id(session: &Session) -> Result<Option<Uuid>, tower_sessions::session::Error> {
    let value: Option<String> = session.get(USER_ID_KEY).await?;
    Ok(value.and_then(|s| Uuid::parse_str(&s).ok()))
}

pub async fn clear_session(session: &Session) -> Result<(), tower_sessions::session::Error> {
    session.flush().await
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_user_id_key_is_defined() {
        assert_eq!(super::USER_ID_KEY, "user_id");
    }
}
