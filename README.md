# Discord Rich Presence

<!-- [![crates.io](https://img.shields.io/crates/v/rpcdiscord.svg)](https://crates.io/crates/discord-rich-presence) -->

Fork of [rpc-discord](https://github.com/vionya/discord-rich-presence) crate<br>
Changes:
* Client intialization with u64
* Refactored examples

A simple, cross-platform crate to connect and send data to Discord's IPC. Special attention is given to sending rich presence data.

## Example

```rust
use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ipc_client = DiscordIpcClient::new(0000000000000000000u64)?;

    client.connect()?;
    client.set_activity(activity::Activity::new()
        .state("foo")
        .details("bar")
    )?;
    client.close()?;

    Ok(())
}
```

### Running example in the CLI

The repository comes with an example you can run with cargo to see how this library works.
You can provide an argument or it will default to a working application/client id.

```
cargo run --example presence -- <CLIENT_ID>
```

## Using this crate

This crate is not (and probably wont be) uploaded to crates.io so this is how you use it:

```toml
[depencencies]
rpcdiscord = { git = "https://github.com/Froststrap/rpcdiscord.git" }
```

## Licensing

This project is licensed under [AGPLv3](./LICENSE).
