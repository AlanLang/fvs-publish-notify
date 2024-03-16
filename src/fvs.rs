use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

pub async fn fetch_fvs_publish_message() -> anyhow::Result<FvsPublishMessage> {
  let url =
    "https://market.fanruan.com/commodities?pg=plugin&id=2b55753a-3d27-45cc-997b-e450b6c33fbc";
  let result = reqwest::get(url).await?.json::<FvsPublishMessage>().await?;
  Ok(result)
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FvsPublishMessage {
  pub state: String,
  pub result: FvsPublishMessageResult,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FvsPublishMessageResult {
  pub id: String,
  pub name: String,
  pub related_plugins: Vec<FvsPublishMessageRelatedPlugin>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FvsPublishMessageRelatedPlugin {
  pub pluginid: String,
  pub name: String,
  pub upload_time: DateTime<Local>,
  pub pluginversion: String,
  pub jartime: DateTime<Local>,
  pub changenotes: String,
  pub pic: String,
}

#[cfg(test)]
mod tests {

  #[tokio::test]
  async fn it_works() {
    let reulst = super::fetch_fvs_publish_message().await.unwrap();
    assert_eq!(reulst.state, "ok");
  }
}
