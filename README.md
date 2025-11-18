# PieBotReborn
[PieInOblivion's JS Discord bot](https://github.com/PieInOblivion/PieBot) written in Rust this time.

## Features
- Multi-server compatibility
- Slash command
- A queue system in which individual song requests take priority above playlists and albums
- Automatically leaves the voice channel when it's alone (Has the side effect of being able to stop it by moving it into an empty voice channel)
- Youtube text searching, urls and playlist urls
- Spotify songs, albums and user playlists
- Rock, Paper, Scissors vs the bot

## Commands
### Play or Search Music
`/play (Youtube search, YouTube song url, YouTube playlist url, Spotify song url/uri, Spotify album url/uri, Spotify playlist url/uri)`
### Now Playing Title and URL
`/np`
### Skip Current Song
`/skip`
### Pause Current Song
`/pause`
### Resume Current Song
`/resume`
### Stop Current Playback and Reset Song Queues
`/stop`
### Show Both Queue Lengths
`/queue`
### Remove Last Song added in the User Priority Queue. (Useful for removing search mismatches, etc.)
`/remove`
### Play Rock, Paper or Scissors vs the PieBot
`/rps rock`
`/rps paper`
`/rps scissors`

## Requirements
1. [Rust](https://www.rust-lang.org/) (Not required after it's compiled)
2. [yt-dlp](https://github.com/yt-dlp/yt-dlp) in PATH

## Setup
1. Clone this repo: `git clone https://github.com/PieInOblivion/PieBotReborn.git`

2. Secrets and config

    - Keep a `secret/guilds` file in the project root with the `Server ID` of each server the bot should listen to (one per line). The `rps` file is for Rock Paper Scissors score keeping

3. Required environment variables

    - `DISCORD_TOKEN` - your Discord bot token
    - `SPOTIFY_ID` - Spotify Client ID
    - `SPOTIFY_SECRET` - Spotify Client Secret
    - `YOUTUBE_KEY` - YouTube API key

5. Build

    - Run `cargo build --release` to compile. Because the bot reads secrets from environment variables at runtime, the compiled binary will not contain your API keys if you do not bake them into build-time environment variables. Keep your runtime environment secure.

6. Run

    - Run `cargo run --release` (ensure required env vars are set and `secret/guilds` exists).