use crate::Metadata::metadata::{MovieRecord, MovieMetadata};

pub fn make_cards() -> String
{
    let movie_record = MovieRecord::load();
    let mut metadata_vector: Vec<MovieMetadata> = Vec::new();
    let mut html_vector: Vec<String> = Vec::new();
    for file in movie_record.file_vector.clone()
    {
        metadata_vector.push(MovieMetadata::load(file));
    }
    
    for metadata in metadata_vector
    {
        let string = format!("<div class=\"card\"><a href=\"\"><img src=\"{}\" alt=\"\"/><h2>{}</h2></a></div>", metadata.poster_link, metadata.title);
        html_vector.push(string);
    }

    html_vector.join("")
}
