use directories::UserDirs;
use std::{fs, path::PathBuf};
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
    pub fn save<'a>(&self) -> PathBuf
    {
        let savedir: PathBuf = UserDirs::new()
            .unwrap()
            .document_dir()
            .expect("Couldn't get user's document directory")
            .join("MovieManager")
            .join(format!("{}", self.id.clone()));
        let savefile = savedir.join("metadata.save");
        if !savedir.exists()
        {
            fs::create_dir_all(savedir.clone())
                .expect(&format!("Unable to create '{}'", savedir.display()));
        }

        save_file(savefile.clone(), 0, self).expect(&format!(
            "Unable to save metadata to '{}'",
            savefile.display()
        ));
        savefile
    }

    pub fn load(file: PathBuf) -> MovieMetadata { load_file(file, 0).unwrap() }
}

#[derive(Savefile)]
pub struct MovieRecord
{
    pub movie_libary_vector: Vec<PathBuf>,
    pub record_location: PathBuf,
    pub file_vector: Vec<PathBuf>,
}

impl MovieRecord
{
    pub fn exits() -> bool
    {
        let filename = UserDirs::new()
        .unwrap()
        .document_dir()
        .expect("Couldn't get user's document directory")
        .join("MovieManager")
        .join("movie_record.save");
        return filename.exists() && filename.is_file();
        }

    pub fn add(&mut self, path: PathBuf)
    {
        self.file_vector.push(path);
        save_file(self.record_location.clone(), 0, self).expect(&format!(
            "Unable to save metadata to '{}'",
            self.record_location.display()
        ));
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
            movie_libary_vector: Vec::new(),
            record_location: filename,
            file_vector: Vec::new(),
        };

        let filedir = UserDirs::new()
        .unwrap()
        .document_dir()
        .expect("Couldn't get user's document directory")
        .join("MovieManager");

        if !filedir.exists()
        {
            fs::create_dir_all(filedir);
        }
        save_file(record.record_location.clone(), 0, &record).expect(&format!(
            "Unable to save metadata to '{}'",
            record.record_location.display()
        ));
        record
    }
}
