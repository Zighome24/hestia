use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

/// Hash a password using Argon2id
pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2.hash_password(password.as_bytes(), &salt)?;
    Ok(hash.to_string())
}

/// Verify a password against a stored hash
pub fn verify_password(password: &str, hash: &str) -> Result<bool, argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(hash)?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_password_produces_valid_hash() {
        let hash = hash_password("my_secure_password").unwrap();
        assert!(hash.starts_with("$argon2"));
    }

    #[test]
    fn test_verify_correct_password() {
        let hash = hash_password("test_password_123").unwrap();
        assert!(verify_password("test_password_123", &hash).unwrap());
    }

    #[test]
    fn test_verify_incorrect_password() {
        let hash = hash_password("correct_password").unwrap();
        assert!(!verify_password("wrong_password", &hash).unwrap());
    }

    #[test]
    fn test_different_passwords_produce_different_hashes() {
        let hash1 = hash_password("password1").unwrap();
        let hash2 = hash_password("password2").unwrap();
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_same_password_produces_different_hashes() {
        // Due to random salt, same password should produce different hashes
        let hash1 = hash_password("same_password").unwrap();
        let hash2 = hash_password("same_password").unwrap();
        assert_ne!(hash1, hash2);
        // But both should verify
        assert!(verify_password("same_password", &hash1).unwrap());
        assert!(verify_password("same_password", &hash2).unwrap());
    }
}
