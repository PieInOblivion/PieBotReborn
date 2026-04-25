use std::collections::VecDeque;
use std::sync::Arc;

use serenity::prelude::Context;

use crate::utils::structs::{BotData, Song};

pub async fn yt_search(ctx: &Context, q: &str) -> Option<Song> {
    let data = ctx.data::<BotData>();
    let key = &data.youtube_key;

    let url = "https://www.googleapis.com/youtube/v3/search";
    let query = [
        ("part", "snippet"),
        ("q", q),
        ("maxResults", "1"),
        ("type", "video"),
        ("key", key),
    ];

    let response = yt_https_request(ctx, url, &query).await?;
    let video_id = response["items"][0]["id"]["videoId"].as_str()?;
    let video_title = response["items"][0]["snippet"]["title"].as_str()?;

    Some(Song::WithId {
        id: Arc::from(video_id),
        title: Arc::from(video_title),
    })
}

pub async fn yt_id_to_name(ctx: &Context, id: &str) -> Option<Song> {
    let data = ctx.data::<BotData>();
    let key = &data.youtube_key;

    let url = "https://www.googleapis.com/youtube/v3/videos";
    let query = [("part", "snippet"), ("id", id), ("key", key)];

    let response = yt_https_request(ctx, url, &query).await?;
    let video_title = response["items"][0]["snippet"]["title"].as_str()?;

    Some(Song::WithId {
        id: Arc::from(id),
        title: Arc::from(video_title),
    })
}

pub async fn yt_list_id_to_vec(ctx: &Context, id: &str) -> Option<VecDeque<Song>> {
    let data = ctx.data::<BotData>();
    let key = &data.youtube_key;

    let mut next_page_token = String::new();

    let mut list: VecDeque<Song> = VecDeque::new();

    loop {
        let url = "https://www.googleapis.com/youtube/v3/playlistItems";
        let query = [
            ("part", "snippet"),
            ("maxResults", "50"),
            ("pageToken", &next_page_token),
            ("playlistId", id),
            ("key", key),
        ];

        let response = yt_https_request(ctx, url, &query).await?;

        if let Some(res) = response["items"].as_array() {
            for item in res.iter() {
                let video_id = item["snippet"]["resourceId"]["videoId"].as_str()?;
                let video_title = item["snippet"]["title"].as_str()?;

                list.push_back(Song::WithId {
                    id: Arc::from(video_id),
                    title: Arc::from(video_title),
                });
            }
        }

        if let Some(token) = response["nextPageToken"].as_str() {
            next_page_token = token.to_string();
        } else {
            break;
        }
    }

    Some(list)
}

async fn yt_https_request(
    ctx: &Context,
    url: &str,
    query: &[(&str, &str)],
) -> Option<serde_json::Value> {
    let data = ctx.data::<BotData>();

    let response = data.http.get(url).query(query).send().await.ok()?;

    response.json().await.ok()
}
