use {
  rpcdiscord::{
    DiscordIpc,
    DiscordIpcClient,
    activity,
  },
  std::error::Error,
};

#[test]
fn test_models() -> Result<(), Box<dyn Error>> {
  let mut client = DiscordIpcClient::new(771124766517755954)?;
  client.connect()?;

  let activity = activity::Activity::new()
    .state("A test")
    .details("A placeholder")
    .assets(
      activity::Assets::new()
        .large_image("large-image")
        .large_text("Large text")
        .small_image("https://picsum.photos/id/128/200")
        .small_text("Small image"),
    )
    .buttons(vec![activity::Button::new(
      "A button",
      "https://github.com",
    )]);
  client.set_activity(activity)?;

  client.close()?;
  Ok(())
}
