mod fvs;
mod notify;
use std::env;
use tokio::time;

#[tokio::main]
async fn main() {
  dotenv::dotenv().ok();
  env_logger::init();
  let wechat_bot_url = env::var("BOT_URL").unwrap();
  let publish_message = fvs::fetch_fvs_publish_message().await.unwrap();
  let related_plugins = publish_message.result.related_plugins.get(0).unwrap();
  let mut version = related_plugins.pluginversion.clone();
  log::info!("server is start, version: {}", version);

  let mut interval = time::interval(time::Duration::from_secs(2 * 60));
  loop {
    interval.tick().await;
    if let Ok(publish_message) = fvs::fetch_fvs_publish_message().await {
      if let Some(related_plugins) = publish_message.result.related_plugins.get(0) {
        if related_plugins.pluginversion != version {
          version = related_plugins.pluginversion.clone();
          log::info!("notify is start, version: {}", version);
          let notify_message = notify::make_notify_message(related_plugins);
          let bot_message = notify::BotMessage::from_notify_message(notify_message);
          let _ = notify::send_notify_message(&wechat_bot_url, &bot_message).await;
        } else {
          log::info!("version is not change, version: {}", version);
        }
      }
    }
  }
}
