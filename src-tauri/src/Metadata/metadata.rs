use directories::UserDirs;
use std::{
    fs,
    path::{PathBuf},
};
use savefile::prelude::*;

#[derive(Savefile)]
pub struct MovieMetadata
{
    pub id: u64,
    pub title: String,
    pub rating: f64,
    pub release_year: u16,
    pub certification: String,
    pub poster_link: String,
    pub video: PathBuf,
}

impl MovieMetadata
{
    pub fn save(&self) -> PathBuf
    {
        let savedir = UserDirs::new()
            .unwrap()
            .document_dir()
            .expect("Couldn't get user's document directory")
            .join("MovieManager")
            .join(format!("{}",self.id.clone()));
        let savefile = savedir.join("metadata.save");
        if !savedir.exists()
        {
            fs::create_dir_all(savedir);
        }

        save_file(savefile.clone(), 0, self);
        savefile
    }

    pub fn load(file: PathBuf) -> MovieMetadata { load_file(file, 0).unwrap()}
}

#[derive(Savefile)]
pub struct MovieRecord
{
    pub record_location: PathBuf,
    pub file_vector: Vec<PathBuf>,
}

impl MovieRecord
{
    pub fn add(&mut self, path: PathBuf)
    {
        self.file_vector.push(path);
        save_file(self.record_location.clone(), 0, self);
    }

    pub fn remove(&mut self, path: PathBuf)
    {
        let location = self.file_vector.binary_search(&path).unwrap();
        self.file_vector.remove(location);
    }

    pub fn load() -> MovieRecord
    {
        let filename = UserDirs::new()
            .unwrap()
            .document_dir()
            .expect("Couldn't get user's document directory")
            .join("MovieManager")
            .join("movie_record.save");

        if !filename.exists()
        {
            return MovieRecord::new();
        }

        load_file(filename, 0).unwrap()
    }

    pub fn new() -> MovieRecord
    {
        let filename = UserDirs::new()
            .unwrap()
            .document_dir()
            .expect("Couldn't get user's document directory")
            .join("MovieManager")
            .join("movie_record.save");

        let record = MovieRecord {
            record_location: filename,
            file_vector: Vec::new(),
        };
        save_file(record.record_location.clone(), 0, &record);
        record
    }
}
