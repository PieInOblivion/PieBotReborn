# PieBotReborn
[PieInOblivion's JS Discord bot](https://github.com/PieInOblivion/PieBot) written in Rust this time.

## Features
- Multi-server compatibility
- Slash command
- A unique queue system in which individual song requests take priority above playlists and albums
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
3. [ffmpeg](https://ffmpeg.org/) in PATH

## Setup
1. Clone this repo: `git clone https://github.com/PieInOblivion/PieBotReborn.git`

2. Create a folder named `secret` in the projects root directory. You can also rename `secret_example` and edit the required files

3. Inside this `secret` folder create/edit these files:
    1. `discord` which stores your [Discord API Key](https://discordjs.guide/preparations/setting-up-a-bot-application.html#creating-your-bot)
    2. `guilds` which stores the  `Server ID` of each server you want the bot the listen to, each seperated by a new line
        - Note: Discord has built-in tools for limiting slash commands to certain roles and channels
    3. `youtube` which stores your [Youtube API Key](https://developers.google.com/youtube/v3/getting-started)
    4. `spotifyId` which stores your [Spotify Web API](https://developer.spotify.com/documentation/web-api) / [Client ID](https://developer.spotify.com/documentation/web-api/tutorials/client-credentials-flow)
    5. `spotifySecret` which stores your Spotify Web API Client Secret
    6. `rps` with `0 0`. For global tracking of our rock, paper, scissors scores

4. Run `cargo b --release` to compile. The compiled binary will contain your API keys, so keep it safe. The only file that must remain after compilation is the `rps` file

5. `cargo r --release` to run