use std::path::PathBuf;

use crate::Metadata::metadata::MovieRecord;

pub fn add(path: PathBuf) { let mut record = MovieRecord::load(); record.movie_libary_vector.push(path); }

pub fn list() -> String
{
    let mut record = MovieRecord::load();
    let mut list: Vec<String> = Vec::new();
    for item in record.movie_libary_vector.clone()
    {
        list.push(String::from(item.to_str().unwrap()));
    }
    list.join("\n")
}

pub fn remove(path: PathBuf)
{
    let mut record = MovieRecord::load();
    let location = record.file_vector.binary_search(&path).unwrap();
    record.movie_libary_vector.remove(location);
}
