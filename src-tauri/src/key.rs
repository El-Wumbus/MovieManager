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
        pub fn get() -> Option<OmdbKey>
        {
            let filename = UserDirs::new()
                .unwrap()
                .document_dir()
                .expect("Couldn't get user's document directory")
                .join("MovieManager")
                .join("omdb.key");

            if !filename.exists()
            {
                return None;
            }

            let key = match fs::read_to_string(filename)
            {
                Ok(x) => String::from(x.trim()),
                Err(_) => return None,
            };

            Some(OmdbKey::Key(key))
        }
    }
}
