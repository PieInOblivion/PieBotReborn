use hyper::{Body, Client, Request};
use hyper_rustls::HttpsConnectorBuilder;

pub async fn spotify_track() {
    let https = HttpsConnectorBuilder::new()
        .with_native_roots()
        .https_only()
        .enable_http2()
        .build();

    let client = Client::builder().build::<_, Body>(https);

    let mut req = Request::builder()
        .method("GET")
        .uri("https://api.spotify.com/v1/albums/4aawyAB9vmqN3uQ7FjRGTy/tracks?offset=0&limit=50&market=au&locale=en-AU")
        .body(Body::empty())
        .unwrap();

    req.headers_mut()
        .insert("header-name", "header-value".parse().unwrap());

    let res = client.request(req).await;
    println!("{:#?}", res);
}
