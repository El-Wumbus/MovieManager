pub mod omdb_key
{
    use directories::UserDirs;
    use std::fs;
    pub enum OmdbKey
    {
        Key(String),
    }

    impl OmdbKey
    {
        pub fn get() -> Result<OmdbKey, String>
        {
            let filename = UserDirs::new()
                .unwrap()
                .document_dir()
                .expect("Couldn't get user's document directory")
                .join("MovieManager")
                .join("omdb.key");

            if !filename.exists()
            {
                return Err(format!("No API key file found at {}", filename.display()));
            }

            let key = match fs::read_to_string(filename.clone())
            {
                Ok(x) => String::from(x.trim()),
                Err(_) => return Err(format!("Couldn't read API key from {}", filename.display())),
            };

            Ok(OmdbKey::Key(key))
        }
    }
}
