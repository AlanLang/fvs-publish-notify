use serde::{Deserialize, Serialize};

use crate::fvs::FvsPublishMessageRelatedPlugin;

pub fn make_notify_message(publish_message: &FvsPublishMessageRelatedPlugin) -> NotifyMessage {
  NotifyMessage::new(
    publish_message.pluginversion.clone(),
    publish_message
      .upload_time
      .format("%Y年%-m月%-d日")
      .to_string(),
    publish_message.jartime.format("%Y年%-m月%-d日").to_string(),
    get_feature(&publish_message.changenotes),
    publish_message.pic.clone(),
  )
}

pub async fn send_notify_message(url: &str, message: &BotMessage) -> anyhow::Result<()> {
  let _ = reqwest::Client::new()
    .post(url)
    .json(&message)
    .send()
    .await?
    .text()
    .await?;
  Ok(())
}

fn get_feature(changenotes: &str) -> String {
  let reuslt = find_first_a_tag_content(changenotes);
  match reuslt {
    Some(feature) => feature.to_string(),
    None => "".to_string(),
  }
}

fn find_first_a_tag_content(text: &str) -> Option<&str> {
  if let Some(start) = text.find("<a>") {
    if let Some(end) = text[start..].find("</a>") {
      return Some(&text[start + 3..start + end]);
    }
  }
  None
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BotMessage {
  pub msgtype: String,
  pub news: BotMessageNews,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BotMessageNews {
  pub articles: Vec<BotMessageArticle>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BotMessageArticle {
  pub title: String,
  pub description: String,
  pub url: String,
  pub picurl: String,
}

impl BotMessage {
  pub fn from_notify_message(message: NotifyMessage) -> BotMessage {
    BotMessage {
      msgtype: "news".to_string(),
      news: BotMessageNews {
        articles: vec![BotMessageArticle {
          title: message.title,
          description: format!(
            "{}\n🎉🎉🎉🎉🎉🎉\nJar包时间: {}\n更新内容：\n{}",
            message.publish_time, message.jar_time, message.feature
          ),
          url: "https://market.fanruan.com/plugin/2b55753a-3d27-45cc-997b-e450b6c33fbc".to_string(),
          picurl: message.pic,
        }],
      },
    }
  }
}

pub struct NotifyMessage {
  pub title: String,
  pub version: String,
  pub publish_time: String,
  pub jar_time: String,
  pub feature: String,
  pub pic: String,
}

impl NotifyMessage {
  pub fn new(
    version: String,
    publish_time: String,
    jar_time: String,
    feature: String,
    pic: String,
  ) -> NotifyMessage {
    let title = format!("FineVis数据可视化\nv{} 发布成功", version);
    NotifyMessage {
      title,
      version,
      publish_time,
      jar_time,
      feature,
      pic,
    }
  }
}

#[cfg(test)]
mod tests {

  #[test]
  fn get_feature_works() {
    let message = "<p>[2024-03-04]<a>三维组件抗锯齿的msaa默认开启，视角设置绑定模型交互优化；新增模板页、tab、表格的页相关JSAPI</a></p>\n    <p>[2024-02-04]<a>悬浮暂停逻辑修改；修复一些已知问题</a></p>\n    <p>[2024-01-30]<a>修复外部参数传参不生效的问题</a></p>\n    <p>[2024-01-22]<a>FVS支持定时调度，轮播器改造为tab组件，增加查询按钮，三维glb使用indexedDB缓存；优化三维自定义视角、漫游动画；部分功能重构</a></p>\n    <p>[2024-01-09]<a>修复三维背景问题和三维空场景无法保存问题</a></p>\n    <p>[2024-01-02]<a>新增组件显隐事件，新增控件查询面板；优化三维组件标签和预警效果，优化编辑器列表交互，图片、视频素材支持预览；修复模板损坏问题</a></p>\n    <p>[2023-12-18]<a>新增绝对画布组件；我的资源标签支持保留；扩展日期&单复选下拉框控件样式</a></p>\n    <p>[2023-12-04]<a>新增时间组件；自动播放/旋转类功能支持暂停；画布中组件支持按图层层级选中；三维场景支持添加相机视角；三维组件内支持设置相机的垂直角度和缩放限制</a></p>\n    <p>[2023-11-21]<a>支持分页设置隐藏/显示、画布全屏模式；新增桌面端移动端布局切换；三维模型的移动、旋转、缩放操作支持撤销还原；三维模型操作能力增强；修复一些已知问题</a></p>\n    <p>[2023-11-06]<a>三维城市建筑生成优化；本地视频检测优化；showDialogAPI支持宽高百分比设置；字体组件支持搜索；支持双击尺寸热区创建可视化看板；部分性能优化；修复一些已知问题</a></p>\n    <p>[2023-10-23]<a>FVS图表支持单独导出，组件控件支持边框线、控件支持字符样式，移动端适配视图树和下拉树，showDialog接口支持配置牵引线，增加获取组件位置API，增加自定义模型跟随弹窗API；优化三维标签性能、三维城市瓦片地图，优化分页复制粘贴；解决跑马灯已知BUG</a></p>\n    <p>[2023-09-27]<a>支持三维背景；支持根据数据源变动刷新组件；多选类控件支持设置不允许全选；模板预览支持导航；页面加载支持背景色；图表组件支持绑定单元格数据；表格支持导出jsapi；支持FVS模板URL后参数导出和JSAPI导出pdf、pptx、png；优化三维组件功能结构；优化模型上传管理；优化弹出框样式；优化FVS轮播器tab样式</a></p>\n    <p>[2023-09-15]<a>修复场景地图跳转页面接口不生效等问题</a></p>\n    <p>[2023-09-08]<a>完善轮播三维组合地图十段线绘制,修复一些已知问题</a></p>\n    <p>[2023-08-28]<a>修复轮播器按钮位置问题</a></p>\n    <p>[2023-08-25]<a>支持跨模板复制组件或分页时复制各种资源、支持三维场景内自定义漫游动画、支持图表组件绑定单元格数据、支持导出扩展图表组件、支持模型批量上传、轮播组件支持懒加载；合并配置项到FVS模板设置中</a></p>\n    <p>[2023-08-14]<a>三维城市geojson上传限制优化；新增模板exitFullscreen接口；部分组件支持开启降级渲染；跑马灯支持单元格图表</a></p>\n    <p>[2023-08-04]<a>修复安全问题</a></p>\n    <p>[2023-07-31]<a>新增模板数据预警、表格组件放大、下拉树和视图树控件等功能；模板预览支持框选放大；资源中心在线zip支持下载到本地；优化组件角标逻辑</a></p>\n    <p>[2023-07-14]<a>支持移动端预览、三维组件（除Unity）新增帧率检测，流畅渲染模式、监控视频支持RTSP和RTMP协议、模板新增水印设置、表格支持setValue、网页框组件支持和页面进行通信；优化closeDialog接口</a></p>\n    <p>[2023-07-03]<a>优化geojson检测规则；新增数据图层相关接口：subscribeMarkerData、updateMarkerData；网页链接、弹出框，监控视频支持插入公式；模板支持导出</a></p>\n    <p>[2023-06-16]<a>优化三维组件的数据标签渲染，新增表格排序功能，FVS分页跳转支持传递参数、标题组件支持getValueJSAPI，三维城市和自定义模型组件新增仅刷新数据API</a></p>\n    <p>[2023-06-02]<a>优化场景地图自动简化逻辑，支持图表组件部分JSAPI, 分页支持跨模板复制粘贴，开放FRM应用版本管理入口，优化FTP预览机制，修复一些已知问题</a></p>\n    <p>[2023-05-22]<a>修复tomcat预览模板报错</a></p>\n    <p>[2023-05-19]<a>设计器支持打开多个FVS模板；支持remoteEvaluate API；优化三维模型运动API、聚焦API；三维模型环境反射默认关闭；控件数据字典弹窗改为前端实现</a></p>\n    <p>[2023-05-05]<a>修复jquery兼容问题</a></p>\n    <p>[2023-04-27]<a>新增fvs超链在平台内新页签打开、网页框传参支持动态值、组件复制保留组件之间的动态关系、默认翻页效果JSAPI；优化Unity组件资源不合理提示</a></p>\n    <p>[2023-04-14]<a>新增三维组件模型控制JSAPI、跑马灯产品化；FVS组件支持alt快速复制；删除组件监控刷新loading</a></p>\n    <p>[2023-03-31]<a>新增画布辅助线；新增表格组分翻页JSAPI；快捷键粘贴优化；修复一些已知问题</a></p>\n    <p>[2023-03-17]<a>新增三维时间体系、双向铺满自适应提示；优化三维模型发光设计、组件复制粘贴逻辑；支持83版本chrome浏览器</a></p>\n    <p>[2023-03-10]<a>新增离屏控制功能</a></p>\n    <p>[2023-03-03]<a>修复一些已知问题</a></p>\n    <p>[2023-02-17]<a>新增资源复用、支持模型和光源选中、支持FRM的扩展图表复制粘贴到FVS</a></p>\n    <p>[2023-01-13]<a>新增三维滤镜、支持FRM的表格和图表组件复制粘贴到FVS；优化色彩区间元件</a></p>\n    <p>[2023-01-09]<a>修改三维城市简约风格效果的BUG</a></p>\n    <p>[2022-12-29]<a>自定义模型组件新增动态地面效果，表格组件支持分页和导出，控件筛选对三维组件生效；优化自定义模型环境、光源阴影效果</a></p>\n    <p>[2022-12-12]<a>新增商用免费字体、支持tab块场景</a></p>\n    <p>[2022-11-28]<a>优化文件选中和hover样式，新增版本兼容提示</a></p>\n    <p>[2022-11-14]<a>优化组件未编辑时的默认效果，refresh接口支持传参</a></p>\n    <p>[2022-11-1]<a>新增模板关闭按钮，新增导出内置数据集模板功能，新增模板加载动画，优化自定义模型场景列表结构</a></p>\n    <p>[2022-10-17]<a>Unity组件支持多场景</a></p>\n    <p>[2022-9-28]<a>修改lic注册功能</a></p>\n    <p>[2022-9-27]<a>新增预览时支持组件隐藏、对齐了新前端FRM的JSAPI、双向铺满自适应等功能</a></p>\n    <p>[2022-9-23]<a>修复偶发重启后编辑器中展示空白的bug</a></p>\n    <p>[2022-9-13]<a>新增fvs表格支持当前决策报表对象、fvs模板支持定时刷新等功能</a></p>\n    <p>[2022-8-31]<a>修复轮播图表轮播间隔不准确和兼容问题 </a></p>\n    <p>[2022-8-29]<a>三维组件新增线框风格动画、优化了组件面板和input_number控件、富文本支持垂直居中等功能</a></p>\n    <p>[2022-8-15]<a>优化了组件lic报错、三维组件复制逻辑、三维组件位移工具等功能</a></p>\n    <p>[2022-8-1]<a>新增unity组件、优化geojson和glb模型限制等功能</a></p>\n    <p>[2022-7-20]<a>修复表格中图表字体自适应问题</a></p>\n    <p>[2022-7-15]<a>丰富组件、标题等样式，优化了三维组件的交互</a></p>\n    <p>[2022-7-8]<a>新增控件、网页链接/弹出框支持选择当前目录模板等功能，优化了三维组件的性能</a></p>\n    <p>[2022-6-1]<a>修复快速保存导致内容丢失、自适应兼容、多模型渲染等问题</a></p>\n    <p>[2022-5-24]<a>新增三维城市标签的富文本自定义功能和标签预警功能</a></p>\n    <p>[2022-4-22]<a>修复场景地图标签显示不全和图表默认配色不对的问题</a></p>\n    <p>[2022-4-15]<a>优化了fvs模版读写的逻辑，升级了使用的chrome内核版本</a></p>\n\t <p>[2022-2-28]<a>国际化更新</a></p>\n    <p>[2022-3-14]<a>修复自定义模型组件数据图层只配置一个字段引起的问题</a></p>\n    <p>[2022-3-8]<a>修复地图边界无法选择模板参数，高分辨率下无法将组件置于最左侧等问题</a></p>\n    <p>[2022-1-20]<a>修复初次启动时表格组件报错没有权限，jdk11图表配置界面错位等问题</a></p>\n    <p>[2022-1-12]<a>新增自定义模型组件，fvs组件支持跨模板复用</a></p>\n    <p>[2021-12-9]<a>修复网页链接全屏问题、优化三维城市建筑</a></p>\n    <p>[2021-12-6]<a>修复cpt组件概率性内容丢失的问题</a></p>\n    <p>[2021-12-3]<a>修复弹出框全屏、会话超时无请求时模板报错等问题</a></p>\n    <p>[2021-11-19]<a>新增弹出框、js控制显示隐藏三维城市图层等功能,平台预览适配</a></p>\n    <p>[2021-11-03]<a>修复组件动画设置下拉框无法弹出的问题</a></p>\n    <p>[2021-11-02]<a>修复点击撤销会导致cpt内容丢失等问题</a></p>\n    <p>[2021-10-22]<a>修复在没有网络连接的情况下，不显示编辑器icon的问题</a></p>\n    <p>[2021-10-16]<a>适配10.0，发布大屏编辑器模板插件</a></p>\n    <p>[2021-11-19]<a>新增弹出框、js控制三维城市图层显示隐藏等功能</a></p>\n    <p>[2021-11-15]<a>FVS插件适配11.0</a></p>";
    let result = super::get_feature(message);
    assert_eq!(
      result,
      "三维组件抗锯齿的msaa默认开启，视角设置绑定模型交互优化；新增模板页、tab、表格的页相关JSAPI"
    );
  }
}
