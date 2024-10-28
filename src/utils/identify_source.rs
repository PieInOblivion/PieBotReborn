use crate::utils::structs::SongFilterResult;
use regex::Regex;
use std::sync::LazyLock;

// Don't need to '\/' to escape '/' with rust regex
static YT_ID_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^.*((m\.)?youtu\.be/|e/|vi?/|u/\w/|embed/|\?vi?=|\&vi?=)([^#\&\?]{11}).*").unwrap()
});

static YT_LIST_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"list=([A-Za-z0-9_-]{34,41})").unwrap()
});

static SPOTIFY_TRACK_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?:spotify:track:|spotify\.com/track/)([A-Za-z0-9]{22})").unwrap()
});

static SPOTIFY_PLAYLIST_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?:spotify:playlist:|spotify\.com/playlist/)([A-Za-z0-9]{22})").unwrap()
});

static SPOTIFY_ALBUM_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?:spotify:album:|spotify\.com/album/)([A-Za-z0-9]{22})").unwrap()
});


pub fn parse_source(input: &str) -> SongFilterResult {
    let yt_id = yt_id_extract(input);
    let yt_list = yt_list_extract(input);
    let spot_track = spotify_track_extract(input);
    let spot_list = spotify_playlist_extract(input);
    let spot_album = spotify_album_extract(input);

    let search_needed = [&yt_id, &yt_list, &spot_track, &spot_list, &spot_album]
        .iter()
        .all(|o| o.is_none());

    SongFilterResult {
        yt_id,
        yt_list,
        spot_track,
        spot_list,
        spot_album,
        search_needed,
    }
}

fn yt_id_extract(input: &str) -> Option<String> {
    Some(YT_ID_REGEX.captures(input)?.get(3)?.as_str().to_string())
}

fn yt_list_extract(input: &str) -> Option<String> {
    Some(YT_LIST_REGEX.captures(input)?.get(1)?.as_str().to_string())
}

fn spotify_track_extract(input: &str) -> Option<String> {
    Some(SPOTIFY_TRACK_REGEX.captures(input)?.get(1)?.as_str().to_string())
}

fn spotify_playlist_extract(input: &str) -> Option<String> {
    Some(SPOTIFY_PLAYLIST_REGEX.captures(input)?.get(1)?.as_str().to_string())
}

fn spotify_album_extract(input: &str) -> Option<String> {
    Some(SPOTIFY_ALBUM_REGEX.captures(input)?.get(1)?.as_str().to_string())
}
