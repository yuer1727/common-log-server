use std::collections::HashMap;
use serde_json::Value;
use std::string::String;
use serde::{Deserialize, Serialize};

use crate::common_util::json_common::{default_str, default_i64};


/**
doc: https://serde.rs/
*/

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Client {
    // 客户端版本
    ve: String,
    // 客户端操作系统
    os: String,
    // 客户端操作系统框架
    #[serde(default = "default_str")]
    fr: String,
    // 游戏编号
    #[serde(rename = "gameId")]
    game_id: String,
    // 渠道编号
    #[serde(rename = "channelId")]
    channel_id: String,
    // 客户端序安装标识
    #[serde(default = "default_str")]
    si: String,
    // 客户端扩展参数
    ex: HashMap<String, Value>,
}

impl Client{
    pub fn get_ve(&self) -> &str {
        return self.ve.as_str()
    }
    pub fn get_os(&self) -> &str {
        return self.os.as_str()
    }
    pub fn get_fr(&self) -> &str {
        return self.fr.as_str()
    }
    pub fn get_game_id(&self) -> &str {
        return self.game_id.as_str()
    }
    pub fn get_channel_id(&self) -> &str {
        return self.channel_id.as_str()
    }
    pub fn get_si(&self) -> &str {
        return self.si.as_str()
    }
    pub fn get_ex(&self) -> &HashMap<String, Value> {
        return &self.ex;
    }


}



#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SimpleSdkRequest {

    //请求唯一标识
    id: String,
    // 调用的服务，对应是service的具体方法
    #[serde(default = "default_str")]
    service: String,
    // 客户端公共参数
    client: Client,
    // 数据
    data: HashMap<String, Value>,
    //接口的版本号,默认1.0
    #[serde(default = "default_str")]
    ver: String,
    // 请求客户端的ip （服务端补齐参数）
    #[serde(default = "default_str")]
    ip: String,
    // 请求客户端的port (服务端补齐参数）
    #[serde(default = "default_str")]
    port: String,
    // 请求来源{@link RequestFrom} （服务端补齐参数）
    #[serde(default = "default_str")]
    from: String,
    // 数据格式
    #[serde(default = "default_str")]
    df: String,
    // 服务器接收到请求的时间 （服务端补齐参数）
    #[serde(default = "default_i64")]
    start_time: i64,
    //请求urlParam
    #[serde(default = "default_str")]
    url_param: String,

}


impl SimpleSdkRequest{
    pub fn get_id(&self) -> &str {
        return self.id.as_str()
    }
    pub fn get_service(&self) -> &str {
        return self.service.as_str()
    }
    pub fn get_client(&self) -> &Client {
        return &self.client
    }
    pub fn get_data(&self) -> &HashMap<String, Value> {
        return &self.data
    }
    pub fn get_ver(&self) -> &str {
        return self.ver.as_str()
    }
    pub fn get_ip(&self) -> &str {
        return self.ip.as_str()
    }
    pub fn get_port(&self) -> &str {
        return self.port.as_str()
    }
    pub fn get_from(&self) -> &str {
        return self.from.as_str()
    }
    pub fn set_from(&mut self, from: String) {
        self.from = from
    }
    pub fn get_df(&self) -> &str {
        return self.df.as_str()
    }
    pub fn set_df(&mut self, df: String) {
        self.df = df
    }
    pub fn get_start_time(&self) -> &i64 {
        return &(self.start_time)
    }
    pub fn set_start_time(&mut self, start_time: i64) {
        self.start_time = start_time
    }
    pub fn get_url_param(&self) -> &str {
        return self.url_param.as_str()
    }
    pub fn set_url_param(&mut self, url_param: String) {
        self.url_param = url_param
    }

}


