use {
  rpcdiscord::{
    DiscordIpc,
    DiscordIpcClient,
    activity::{
      self,
      Button,
    },
  },
  std::time::Duration,
};

fn main() -> Result<(), ()> {
  let client_id = std::env::args()
    .nth(1)
    .and_then(|arg| arg.parse::<u64>().ok())
    .unwrap_or(1442965508059300030);

  let mut client = DiscordIpcClient::new(client_id).unwrap();

  client.connect().unwrap();

  client
    .set_activity(
      activity::Activity::new()
        .state("rpcdiscord")
        .details("Doing a backflip")
        .buttons(vec![
          Button::new(
            "A Good YouTube Video",
            "https://www.youtube.com/watch?v=SuMjNLAcPcU",
          ),
          Button::new(
            "Visit the repo!",
            "https://github.com/Froststrap/rpcdiscord",
          ),
        ]),
    )
    .unwrap();

  std::thread::sleep(Duration::new(10, 0));

  client.close().unwrap();
  Ok(())
}
