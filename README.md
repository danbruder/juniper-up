# Juniper/Rust + Apex Up

This is an example of running a [Juniper application](http://juniper.graphql.rs/) on [up](https://up.docs.apex.sh/).

![screenshot.jpg](https://github.com/danbruder/juniper-up/raw/master/screenshot.jpg)

## Requirements

- up
- docker
- rust toolchain

## Getting Started

```
git clone git@github.com:danbruder/juniperup.git MY_APP_NAME
cd MY_APP_NAME 
```

Edit `up.json` replacing MY_APP_NAME with your app name. Adjust profile if necessary as well as other Up [options](https://up.docs.apex.sh/#configuration).

```json
{
  "name": "MY_APP_NAME",
  "profile": "up",
  "hooks": {
    "build": [
      "docker run -v ~/.cargo:/cargo_data -e CARGO_HOME=/cargo_data -v $PWD:/volume -w /volume -t clux/muslrust cargo build --release",
      "cp target/x86_64-unknown-linux-musl/release/MY_APP_NAME server"
    ],
    "clean": "rm server"
  }
}
```

Edit `Cargo.toml` name and authors. 

```toml
[package]
name = "MY_APP_NAME"
version = "0.1.0"
authors = ["MY NAME AND EMAIL"]

... OTHER CONFIG HERE
```

## Deployment

```
up deploy
```
