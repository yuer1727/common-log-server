use std::collections::HashMap;
use serde_json::Value;
use std::string::String;
use serde::{Deserialize, Serialize};

use crate::common_util::json_common::{default_str, default_u64};
use crate::common_util::time_common::get_timestamp_millis;


/**
doc: https://serde.rs/
https://stackoverflow.com/questions/19650265/is-there-a-faster-shorter-way-to-initialize-variables-in-a-rust-struct
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
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


impl Default for Client {
    fn default() -> Client {
        Client {
            ve: "".to_string(),
            os: "".to_string(),
            fr: "".to_string(),
            game_id: "".to_string(),
            channel_id: "".to_string(),
            si: "".to_string(),
            ex: HashMap::new(),
        }
    }
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



#[derive(Debug, Clone, Serialize, Deserialize)]
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
    #[serde(default = "default_u64")]
    start_time: u64,
    //请求urlParam
    #[serde(default = "default_str")]
    url_param: String,

}


impl Default for SimpleSdkRequest {
    fn default() -> SimpleSdkRequest {
        SimpleSdkRequest {
            id: "".to_string(),
            service: "".to_string(),
            client: Default::default(),
            data: HashMap::new(),
            ver: "".to_string(),
            ip: "".to_string(),
            port: "".to_string(),
            from: "".to_string(),
            df: "".to_string(),
            start_time: 0,
            url_param: "".to_string(),
        }
    }
}

impl SimpleSdkRequest{

    pub fn new(start_time: u64, service: &String) -> SimpleSdkRequest{
        return SimpleSdkRequest{
            id: format!("{}", start_time),
            service: service.clone(),
            ver: "1.0".to_string(),
            df: "json".to_string(),
            start_time: start_time,
            client: Client{
                ve: "1.0".to_string(),
                ..Default::default()
            },
            ..Default::default()
        }
    }



    pub fn get_id(&self) -> &str {
        return self.id.as_str()
    }
    pub fn get_service(&self) -> &str {
        return self.service.as_str()
    }
    pub fn set_service(&mut self, service: String) {
        self.service = service
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
    pub fn set_ip(&mut self, ip: String) {
        self.ip = ip
    }
    pub fn get_port(&self) -> &str {
        return self.port.as_str()
    }
    pub fn set_port(&mut self, port: String) {
        self.port = port
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
    pub fn get_start_time(&self) -> &u64 {
        return &(self.start_time)
    }
    pub fn set_start_time(&mut self, start_time: u64) {
        self.start_time = start_time
    }
    pub fn get_url_param(&self) -> &str {
        return self.url_param.as_str()
    }
    pub fn set_url_param(&mut self, url_param: String) {
        self.url_param = url_param
    }

}


