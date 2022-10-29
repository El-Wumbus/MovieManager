use std::path::PathBuf;
use super::super::omdb_key;
use super::metadata::{self, MovieMetadata};

impl metadata::MovieMetadata
{
    pub async fn from_query(query: String, video_path: PathBuf) -> Option<MovieMetadata>
    {
        let apikey :String = match omdb_key::OmdbKey::get() {
            Some(x) =>match x {
                omdb_key::OmdbKey::Key(x) => x,
            },
            None => return None,
        };

        let movies = omdb::search(query)
        .apikey(apikey.clone())
        .get()
        .await
        .unwrap();
        let movie = match movies.results.get(0)
        {
            None => return None,
            Some(x) => x,
        };

        let movie_info = omdb::imdb_id(movie.imdb_id.clone()).apikey(apikey).get().await.unwrap();
        
        Some(MovieMetadata {
            id: movie_info.imdb_id.parse().unwrap(),
            title: movie_info.title,
            rating: movie_info.imdb_rating.parse().unwrap(),
            release_year: movie_info.year.parse().unwrap(),
            certification:movie_info.rated,
            poster_link: movie_info.poster,
            video: video_path,
        })
    }
}
