# Plex Live Search
Search Plex's Live TV &amp; DVR Guide for Shows. Right now it only search for sports.

## Environment setup

Set these environment variables so you can talk to Plex
```
$ export PLEX_TOKEN=<Your Plex Token>
$ export PLEX_HOSTNAME=<Hostname of your Plex Server>
```

Optionally set the port for Plex if you do not want to use the default port `32400`
```
$ export PLEX_PORT=32400
```

## Build it

The project can be built with the [regular Rust tool chain](https://www.rust-lang.org/tools/install) on your dev box or with Docker.

On system:
```
$ cargo build --release

$ ./target/release/plex_live_search --ignore-case "the match"
Title: Capital One's The Match: Champions for Charity
      Parent Title: Season 2020
      Grand Parent Title: Capital One's The Match: Champions for Charity
      Summary: Tiger Woods and Peyton Manning face Phil Mickelson and Tom Brady in a charity team golf match from Medalist GC in Hobe Sound, Fla. COVID-19 beneficiaries include Direct Relief, the American Red Cross; Save Small Business; and the ALL IN Challenge.
      Channel: 741 HLNHD (HLN HDTV)
      Begins At: 2020-05-24 13:00:00 -06:00
      Ends At: 2020-05-24 18:00:00 -06:00
```

Docker:
```
./script/docker-build

./script/run --ignore-case "the match"
Title: Capital One's The Match: Champions for Charity
      Parent Title: Season 2020
      Grand Parent Title: Capital One's The Match: Champions for Charity
      Summary: Tiger Woods and Peyton Manning face Phil Mickelson and Tom Brady in a charity team golf match from Medalist GC in Hobe Sound, Fla. COVID-19 beneficiaries include Direct Relief, the American Red Cross; Save Small Business; and the ALL IN Challenge.
      Channel: 741 HLNHD (HLN HDTV)
      Begins At: 2020-05-24 13:00:00 -06:00
      Ends At: 2020-05-24 18:00:00 -06:00
```
