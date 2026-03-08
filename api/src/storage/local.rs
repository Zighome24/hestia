use std::path::PathBuf;
use tokio::fs;
use uuid::Uuid;

const MAX_FILE_SIZE: usize = 10 * 1024 * 1024; // 10 MB

pub struct LocalStorage {
    base_path: PathBuf,
}

impl LocalStorage {
    pub fn new(base_path: &str) -> Self {
        Self {
            base_path: PathBuf::from(base_path),
        }
    }

    pub async fn init(&self) -> std::io::Result<()> {
        fs::create_dir_all(&self.base_path).await
    }

    pub fn validate_content_type(content_type: &str) -> Result<&'static str, String> {
        match content_type {
            "image/jpeg" => Ok("jpg"),
            "image/png" => Ok("png"),
            _ => Err(format!(
                "unsupported file type: {}. Only JPEG and PNG are accepted",
                content_type
            )),
        }
    }

    pub fn validate_file_size(size: usize) -> Result<(), String> {
        if size > MAX_FILE_SIZE {
            return Err(format!(
                "file too large: {} bytes (max {} bytes)",
                size, MAX_FILE_SIZE
            ));
        }
        Ok(())
    }

    pub fn generate_filename(extension: &str) -> String {
        format!("{}.{}", Uuid::new_v4(), extension)
    }

    pub async fn save(&self, filename: &str, data: &[u8]) -> std::io::Result<String> {
        let path = self.base_path.join(filename);
        fs::write(&path, data).await?;
        Ok(filename.to_string())
    }

    pub fn file_path(&self, filename: &str) -> PathBuf {
        self.base_path.join(filename)
    }

    pub async fn delete(&self, filename: &str) -> std::io::Result<()> {
        let path = self.base_path.join(filename);
        if path.exists() {
            fs::remove_file(path).await?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_content_type_jpeg() {
        assert_eq!(
            LocalStorage::validate_content_type("image/jpeg").unwrap(),
            "jpg"
        );
    }

    #[test]
    fn test_validate_content_type_png() {
        assert_eq!(
            LocalStorage::validate_content_type("image/png").unwrap(),
            "png"
        );
    }

    #[test]
    fn test_validate_content_type_invalid() {
        assert!(LocalStorage::validate_content_type("image/gif").is_err());
        assert!(LocalStorage::validate_content_type("application/pdf").is_err());
        assert!(LocalStorage::validate_content_type("text/plain").is_err());
    }

    #[test]
    fn test_validate_file_size_valid() {
        assert!(LocalStorage::validate_file_size(1024).is_ok());
        assert!(LocalStorage::validate_file_size(MAX_FILE_SIZE).is_ok());
    }

    #[test]
    fn test_validate_file_size_too_large() {
        assert!(LocalStorage::validate_file_size(MAX_FILE_SIZE + 1).is_err());
    }

    #[test]
    fn test_generate_filename_has_extension() {
        let name = LocalStorage::generate_filename("jpg");
        assert!(name.ends_with(".jpg"));
        assert!(name.len() > 4); // UUID + .jpg
    }

    #[test]
    fn test_generate_filename_unique() {
        let name1 = LocalStorage::generate_filename("png");
        let name2 = LocalStorage::generate_filename("png");
        assert_ne!(name1, name2);
    }
}
