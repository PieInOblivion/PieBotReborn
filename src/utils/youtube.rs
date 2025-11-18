use std::collections::VecDeque;
use std::sync::Arc;

use serenity::prelude::Context;

use crate::utils::structs::{BotData, Song};

pub async fn yt_search(ctx: &Context, q: &str) -> Option<Song> {
    let key = include_str!("../../secret/youtube");

    let encoded_q = q.replace(' ', "%20");

    let url = format!(
        "https://www.googleapis.com/youtube/v3/search?part=snippet&q={}&maxResults=1&type=video&key={}",
        encoded_q, key
    );

    let response = yt_https_request(ctx, &url).await?;
    let video_id = response["items"][0]["id"]["videoId"].as_str()?;
    let video_title = response["items"][0]["snippet"]["title"].as_str()?;

    Some(Song {
        id: Some(Arc::from(video_id)),
        title: Arc::from(video_title),
    })
}

pub async fn yt_id_to_name(ctx: &Context, id: &str) -> Option<Song> {
    let key = include_str!("../../secret/youtube");

    let url = format!(
        "https://www.googleapis.com/youtube/v3/videos?part=snippet&id={}&key={}",
        id, key
    );

    let response = yt_https_request(ctx, &url).await?;
    let video_title = response["items"][0]["snippet"]["title"].as_str()?;

    Some(Song {
        id: Some(Arc::from(id)),
        title: Arc::from(video_title),
    })
}

pub async fn yt_list_id_to_vec(ctx: &Context, id: &str) -> Option<VecDeque<Song>> {
    let key = include_str!("../../secret/youtube");

    let mut next_page_token: String = String::new();

    let mut list: VecDeque<Song> = VecDeque::new();

    loop {
        let url = format!(
            "https://www.googleapis.com/youtube/v3/playlistItems?part=snippet&maxResults=50&pageToken={}&playlistId={}&key={}",
            next_page_token, id, key
        );

        let response = yt_https_request(ctx, &url).await?;

        next_page_token = response["nextPageToken"].as_str().unwrap_or("").to_string();

        if let Some(res) = response["items"].as_array() {
            for item in res.iter() {
                let video_id = item["snippet"]["resourceId"]["videoId"].as_str()?;
                let video_title = item["snippet"]["title"].as_str()?;

                list.push_back(Song {
                    id: Some(Arc::from(video_id)),
                    title: Arc::from(video_title),
                });
            }
        }

        if next_page_token.is_empty() {
            break;
        }
    }

    Some(list)
}

async fn yt_https_request(ctx: &Context, url: &str) -> Option<serde_json::Value> {
    let data = ctx.data::<BotData>();

    let response = data.http.get(url).send().await.ok()?;

    response.json().await.ok()
}
