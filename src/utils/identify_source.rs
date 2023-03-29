use crate::utils::structs::SongFilterResult;
use regex::Regex;

pub fn parse_source(input: &String) -> SongFilterResult {
    let yt_id = yt_id_extract(input);
    let yt_list = yt_list_extract(input);
    let spot_track = spotify_track_extract(input);
    let spot_list = spotify_playlist_extract(input);
    let spot_album = spotify_album_extract(input);
    let search_needed = vec![&yt_id, &yt_list, &spot_track, &spot_list, &spot_album]
        .iter()
        .all(|o| o.is_none());

    return SongFilterResult {
        yt_id,
        yt_list,
        spot_track,
        spot_list,
        spot_album,
        search_needed,
    };
}

fn yt_id_extract(input: &String) -> Option<String> {
    // Don't need to '\/' to escape '/' with rust regex
    let re_id =
        Regex::new(r"^.*((m\.)?youtu\.be/|e/|vi?/|u/\w/|embed/|\?vi?=|\&vi?=)([^#\&\?]{11}).*")
            .unwrap();
    match re_id.captures(input) {
        Some(m) => Some(m.get(3).unwrap().as_str().to_string()),
        None => None,
    }
}

fn yt_list_extract(input: &String) -> Option<String> {
    let start = input.find("list=");
    if let Some(i) = start {
        return Some(input[i + 5..i + 39].to_string());
    }

    None
}

fn spotify_track_extract(input: &String) -> Option<String> {
    if input.contains("spotify:track:") {
        return Some(input[input.len() - 22..].to_string());
    }

    if let Some(start) = input.find("spotify.com/track/") {
        return Some(input[start + 18..start + 40].to_string());
    }

    None
}

fn spotify_playlist_extract(input: &String) -> Option<String> {
    if input.contains("spotify:playlist:") {
        return Some(input[input.len() - 22..].to_string());
    }

    if let Some(start) = input.find("spotify.com/playlist/") {
        return Some(input[start + 21..start + 43].to_string());
    }

    None
}

fn spotify_album_extract(input: &String) -> Option<String> {
    if input.contains("spotify:album:") {
        return Some(input[input.len() - 22..].to_string());
    }

    if let Some(start) = input.find("spotify.com/album/") {
        return Some(input[start + 18..start + 40].to_string());
    }

    None
}
