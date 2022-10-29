use std::{path::PathBuf, ffi::OsStr, collections::hash_map::DefaultHasher};
use scan_dir::ScanDir;
use tauri::async_runtime::block_on;
use std::hash::{Hash, Hasher};
use crate::Metadata::metadata::{MovieMetadata, MovieRecord};
use chrono;


pub fn scan_dir(libary_dir: PathBuf) -> Vec<PathBuf>
{
    let mut movie_paths: Vec<PathBuf> = Vec::new();

    ScanDir::dirs()
        .read(libary_dir, |iter| {
            for (entry, _) in iter
            {
                let path = entry.path();
                if path.is_file()
                    && match (match path.extension()
                    {
                        Some(x) => x,
                        None => OsStr::new("ERR"),
                    })
                    .to_str()
                    {
                        Some("mp4") | Some("mkv") | Some("mov") => true,
                        _ => false,
                    }
                {
                    movie_paths.push(path);
                }
            }
        })
        .unwrap();

    return movie_paths;
}

pub fn scan_and_enter(record: &mut MovieRecord)
{
    for lib in record.movie_libary_vector.clone()
    {
        let paths = scan_dir(lib);

        for path in paths
        {
            let title = String::from(path.file_stem().unwrap().to_string_lossy());

            let data = match block_on(MovieMetadata::from_query(title.clone(), path.clone()))
            {
                Ok(x) => x,
                Err(_) =>
                {
                    let mut s = DefaultHasher::new();
                    title.clone().hash(&mut s);
                    let hash = s.finish() % chrono::offset::Utc::now().timestamp() as u64;
                    MovieMetadata {
                        id: hash,
                        title,
                        rating: 0.0,
                        release_year: 1970,
                        certification: "R".to_string(),
                        poster_link: "#".to_string(),
                        video: path,
                    }
                }
            };
            record.add(data.save());
        }
    }
}
