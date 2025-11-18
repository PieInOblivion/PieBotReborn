use crate::utils::structs::PlayRequest;
use regex_lite::Regex;
use std::sync::LazyLock;

// Don't need to '\/' to escape '/' with rust regex
static YT_ID_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^.*((m\.)?youtu\.be/|e/|vi?/|u/\w/|embed/|\?vi?=|\&vi?=)([^#\&\?]{11}).*").unwrap()
});

static YT_LIST_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"list=([A-Za-z0-9_-]{34,41})").unwrap());

static SPOTIFY_TRACK_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?:spotify:track:|spotify\.com/track/)([A-Za-z0-9]{22})").unwrap()
});

static SPOTIFY_PLAYLIST_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?:spotify:playlist:|spotify\.com/playlist/)([A-Za-z0-9]{22})").unwrap()
});

static SPOTIFY_ALBUM_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?:spotify:album:|spotify\.com/album/)([A-Za-z0-9]{22})").unwrap()
});

pub fn parse_source(input: &str) -> PlayRequest {
    if let Some(video) = yt_id_extract(input) {
        if let Some(playlist) = yt_list_extract(input) {
            return PlayRequest::YouTubeVideoAndPlaylist { video, playlist };
        }
        return PlayRequest::YouTubeVideo(video);
    }

    if let Some(playlist) = yt_list_extract(input) {
        return PlayRequest::YouTubePlaylist(playlist);
    }

    if let Some(track) = spotify_track_extract(input) {
        return PlayRequest::SpotifyTrack(track);
    }

    if let Some(playlist) = spotify_playlist_extract(input) {
        return PlayRequest::SpotifyPlaylist(playlist);
    }

    if let Some(album) = spotify_album_extract(input) {
        return PlayRequest::SpotifyAlbum(album);
    }

    PlayRequest::Search(input.to_string())
}

fn yt_id_extract(input: &str) -> Option<String> {
    Some(YT_ID_REGEX.captures(input)?.get(3)?.as_str().to_string())
}

fn yt_list_extract(input: &str) -> Option<String> {
    Some(YT_LIST_REGEX.captures(input)?.get(1)?.as_str().to_string())
}

fn spotify_track_extract(input: &str) -> Option<String> {
    Some(
        SPOTIFY_TRACK_REGEX
            .captures(input)?
            .get(1)?
            .as_str()
            .to_string(),
    )
}

fn spotify_playlist_extract(input: &str) -> Option<String> {
    Some(
        SPOTIFY_PLAYLIST_REGEX
            .captures(input)?
            .get(1)?
            .as_str()
            .to_string(),
    )
}

fn spotify_album_extract(input: &str) -> Option<String> {
    Some(
        SPOTIFY_ALBUM_REGEX
            .captures(input)?
            .get(1)?
            .as_str()
            .to_string(),
    )
}
