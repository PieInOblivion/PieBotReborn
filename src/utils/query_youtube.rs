use std::collections::VecDeque;

use hyper::body::to_bytes;
use hyper::{Body, Client, Uri};
use hyper_rustls::HttpsConnectorBuilder;
use serde_json::from_slice;

use crate::utils::structs::Song;

pub async fn yt_search(q: &String) -> Option<Song> {
    let key = include_str!("../../secret/youtube");

    let encoded_q = q.replace(" ", "%20");

    let url = format!(
        "https://www.googleapis.com/youtube/v3/search?part=snippet&q={}&maxResults=1&type=video&key={}",
        encoded_q, key
    );

    let response = yt_https_request(url).await?;
    let video_id = response["items"][0]["id"]["videoId"].as_str()?;
    let video_title = response["items"][0]["snippet"]["title"].as_str()?;

    Some(Song {
        id: Some(video_id.to_string()),
        title: video_title.to_string(),
    })
}

pub async fn yt_id_to_name(id: &String) -> Option<Song> {
    let key = include_str!("../../secret/youtube");

    let url = format!(
        "https://www.googleapis.com/youtube/v3/videos?part=snippet&id={}&key={}",
        id, key
    );

    let response = yt_https_request(url).await?;
    let video_title = response["items"][0]["snippet"]["title"].as_str()?;

    Some(Song {
        id: Some(id.clone()),
        title: video_title.to_string(),
    })
}

pub async fn yt_list_id_to_vec(id: &String) -> Option<VecDeque<Song>> {
    let key = include_str!("../../secret/youtube");

    let mut next_page_token: String = "".to_string();

    let mut list: VecDeque<Song> = VecDeque::new();

    loop {
        let url = format!(
            "https://www.googleapis.com/youtube/v3/playlistItems?part=snippet&maxResults=50&pageToken={}&playlistId={}&key={}",
            next_page_token, id, key
        );

        let response = yt_https_request(url.clone()).await?;

        next_page_token = response["nextPageToken"]
            .as_str()
            .get_or_insert("")
            .to_string();

        if let Some(res) = response["items"].as_array() {
            for item in res.into_iter() {
                list.push_back(Song {
                    id: Some(
                        item["snippet"]["resourceId"]["videoId"]
                            .as_str()
                            .unwrap()
                            .to_string(),
                    ),
                    title: item["snippet"]["title"].as_str()?.to_string(),
                });
            }
        }

        if next_page_token == "" {
            break;
        }
    }

    Some(list)
}

async fn yt_https_request(url: String) -> Option<serde_json::Value> {
    let https = HttpsConnectorBuilder::new()
        .with_native_roots()
        .https_only()
        .enable_http2()
        .build();

    let client = Client::builder().build::<_, Body>(https);

    let uri = url.parse::<Uri>().ok()?;

    let mut res = client.get(uri).await.ok()?;

    let body = to_bytes(res.body_mut()).await.ok()?;

    let json: serde_json::Value = from_slice(&body).ok()?;

    Some(json)
}
