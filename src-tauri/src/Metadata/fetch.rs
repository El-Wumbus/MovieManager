use std::path::PathBuf;
use tauri::async_runtime::block_on;

use super::super::omdb_key;
use super::metadata::{self, MovieMetadata};

pub enum Search
{
    ImdbId(String),
    Query(String),
}

impl metadata::MovieMetadata
{
    pub async fn from_query(query: String, video_path: PathBuf) -> Result<MovieMetadata, String>
    {
        let apikey: String = match omdb_key::OmdbKey::get()
        {
            Ok(x) => match x
            {
                omdb_key::OmdbKey::Key(x) => x,
            },
            Err(x) => return Err(x),
        };

        let movies = omdb::search(query.clone())
            .apikey(apikey.clone())
            .get()
            .await
            .unwrap();
        let movie = match movies.results.get(0)
        {
            None =>
            {
                return Err(format!(
                    "Couldn't get first movie result with query '{}'",
                    query
                ))
            }
            Some(x) => x,
        };

        let movie_info = omdb::imdb_id(movie.imdb_id.clone())
            .apikey(apikey)
            .get()
            .await
            .unwrap();
        let metadata = MovieMetadata {
            id: movie_info.imdb_id.parse().unwrap(),
            title: movie_info.title,
            rating: movie_info.imdb_rating.parse().unwrap(),
            release_year: movie_info.year.parse().unwrap(),
            certification: movie_info.rated,
            poster_link: movie_info.poster,
            video: video_path,
        };
        Ok(metadata)
    }

    pub async fn from_imdb_id(id: String, video_path: PathBuf) -> Result<MovieMetadata, String>
    {
        let apikey: String = match omdb_key::OmdbKey::get()
        {
            Ok(x) => match x
            {
                omdb_key::OmdbKey::Key(x) => x,
            },
            Err(x) => return Err(x),
        };
        let movie_info = omdb::imdb_id(id).apikey(apikey).get().await.unwrap();

        Ok(MovieMetadata {
            id: movie_info.imdb_id.parse().unwrap(),
            title: movie_info.title,
            rating: movie_info.imdb_rating.parse().unwrap(),
            release_year: movie_info.year.parse().unwrap(),
            certification: movie_info.rated,
            poster_link: movie_info.poster,
            video: video_path,
        })
    }
}

impl From<Search> for metadata::MovieMetadata
{
    fn from(item: Search) -> Self
    {
        match item
        {
            Search::ImdbId(x) => return block_on(metadata::MovieMetadata::from_imdb_id(x, PathBuf::new())).unwrap(),
            Search::Query(x) => return block_on(metadata::MovieMetadata::from_query(x, PathBuf::new())).unwrap(),
        }
    }
}
