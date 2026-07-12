# insop-media

## About

A playerctl wrapper daemon for waybar. Instead of using ignored-players like the MPRIS module, we explicitly list our players into "players" into the config.json file. You can also do special-commands for per players. Note: some things are WIP as I've made it for myself

## Install

Requires `rust` and `playerctl`.

```bash
git clone https://github.com/Insop1/insop-media
cd insop-media
cargo install --path .
```

## Config

Insop-Media requires you to have a config file in `~/.config/insop-media`. You can overwrite default commands using special-commands. Default commands are `playerctl -p <players> <command>`.

```json
{
  "players": ["spotatui", "spotify"],
  "images": true,  
  "dynamic": [
    "artist", "title"
  ],
  "special-commands": {
    "play-pause": {
      "spotify": "playerctl -p spotify play-pause"
    },
    "next": {

    },
    "previous": {

    }
    "volume": {
      "spotify": "playerctl -p spotify volume"
    }
  }
}
```
