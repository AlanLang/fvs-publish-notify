mod fvs;
mod notify;
use chrono::{DateTime, Datelike, Local, Timelike, Weekday};
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

  let mut interval = time::interval(time::Duration::from_secs(5 * 60));
  let mut mon_interval = time::interval(time::Duration::from_secs(30));

  loop {
    let now = Local::now();

    if is_monday(&now) {
      mon_interval.tick().await;
    } else {
      interval.tick().await;
    }

    if !can_work(&now) {
      continue;
    }
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

fn can_work(day: &DateTime<Local>) -> bool {
  // 判断是否在周一到周五之间
  let weekday = day.weekday();
  let is_weekday = weekday != Weekday::Sat && weekday != Weekday::Sun;

  // 判断是否在早晨 9 点到晚上 12 点之间
  let is_between_9_and_12 = day.hour() >= 9 && day.hour() < 24;

  is_weekday && is_between_9_and_12
}

fn is_monday(day: &DateTime<Local>) -> bool {
  day.weekday() == Weekday::Mon
}

#[cfg(test)]
mod tests {
  use chrono::TimeZone;

  use super::*;

  #[test]
  fn test_weekday_morning() {
    let dt = Local
      .with_ymd_and_hms(2024, 3, 18, 8, 0, 0)
      .single()
      .unwrap(); // Saturday morning
    assert!(!can_work(&dt)); // Should be false
  }

  #[test]
  fn test_weekday_evening() {
    let dt = Local
      .with_ymd_and_hms(2024, 3, 18, 21, 0, 0)
      .single()
      .unwrap(); // Saturday morning
    assert!(can_work(&dt)); // Should be false
  }

  #[test]
  fn test_weekday_morning_boundaries() {
    let dt = Local
      .with_ymd_and_hms(2024, 3, 16, 10, 0, 0)
      .single()
      .unwrap(); // Saturday morning
    assert!(!can_work(&dt)); // Should be false
  }
}
