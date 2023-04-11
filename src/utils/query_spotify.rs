use rspotify::{
    model::PlaylistId,
    model::{AlbumId, Market},
    prelude::BaseClient,
    ClientCredsSpotify, Credentials,
};

use serenity::client::Context;

use crate::utils::structs::Spotify;

pub async fn spotify_auth(id: &str, secret: &str) -> ClientCredsSpotify {
    let creds = Credentials {
        id: id.to_string(),
        secret: Some(secret.to_string()),
    };

    let spotify = ClientCredsSpotify::new(creds);
    spotify.request_token().await.unwrap();

    spotify
}

pub async fn spotify_album(ctx: &Context) {
    let spotify = {
        let data_read = ctx.data.read().await;
        data_read.get::<Spotify>().unwrap().clone()
    };
    let uri = AlbumId::from_id("1LHSxBpDSoNUrOezwOcLKU").unwrap();

    let _ = spotify.auto_reauth().await;

    let a = spotify.album(uri).await.unwrap();

    let b = a.tracks;

    dbg!(&b.total);

    let uri2 = PlaylistId::from_id("7sdFIvyZMf2PfmbUxUaSzw").unwrap();
    let a2 = spotify.playlist(uri2, None, None).await.unwrap();
    let b2 = a2.tracks;
    dbg!(&b2);
}
