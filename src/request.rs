use std::collections::HashMap;
use serde_json::Value;
use std::string::String;
use serde::{Deserialize, Serialize};

use crate::common_util::json_common::{default_str_option, default_str, default_u64};


/**
doc: https://serde.rs/
https://stackoverflow.com/questions/19650265/is-there-a-faster-shorter-way-to-initialize-variables-in-a-rust-struct
*/

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Client {
    // 客户端版本
    ve: Option<String>,
    // 客户端操作系统
    os: Option<String>,
    // 客户端操作系统框架
    #[serde(default = "default_str_option")]
    fr: Option<String>,
    // 游戏编号
    #[serde(rename = "gameId")]
    game_id: Option<String>,
    // 渠道编号
    #[serde(rename = "channelId")]
    channel_id: Option<String>,
    // 客户端序安装标识
    #[serde(default = "default_str_option")]
    si: Option<String>,
    // 客户端扩展参数
    ex: Option<HashMap<String, Value>>,
}


impl Default for Client {
    fn default() -> Client {
        Client {
            ve: Some("".to_string()),
            os: Some("".to_string()),
            fr: Some("".to_string()),
            game_id: Some("".to_string()),
            channel_id: Some("".to_string()),
            si: Some("".to_string()),
            ex: Some(HashMap::new()),
        }
    }
}

impl Client{
    pub fn get_ve(&self) -> &str {
        match self.ve.as_ref() {
            Some(ve_ref) => return ve_ref,
            None => return "",
        }
    }
    pub fn get_os(&self) -> &str {
        match self.os.as_ref() {
            Some(os_ref) => return os_ref,
            None => return "",
        }
    }
    pub fn get_fr(&self) -> &str {
        match self.fr.as_ref() {
            Some(fr_ref) => return fr_ref,
            None => return "",
        }
    }
    pub fn get_game_id(&self) -> &str {
        match self.game_id.as_ref() {
            Some(game_id_ref) => return game_id_ref,
            None => return "",
        }
    }
    pub fn get_channel_id(&self) -> &str {
        match self.channel_id.as_ref() {
            Some(channel_id_ref) => return channel_id_ref,
            None => return "",
        }
    }
    pub fn get_si(&self) -> &str {
        match self.si.as_ref() {
            Some(si_ref) => return si_ref,
            None => return "",
        }
    }
    pub fn get_ex(&self) -> Option<&HashMap<String, Value>> {
        return self.ex.as_ref()
    }


}



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleSdkRequest {

    //请求唯一标识
    id: Option<String>,
    // 调用的服务，对应是service的具体方法
    #[serde(default = "default_str")]
    service: String,
    // 客户端公共参数
    client: Option<Client>,
    // 数据
    data: Option<HashMap<String, Value>>,
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
            id: Some("".to_string()),
            service: "".to_string(),
            client: Some(Default::default()),
            data: Some(HashMap::new()),
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
            id: Some(format!("{}", start_time)),
            service: service.clone(),
            ver: "1.0".to_string(),
            df: "json".to_string(),
            start_time: start_time,
            client: Some(Client{
                ve: Some("1.0".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        }
    }



    pub fn get_id(&self) -> &str {
        match self.id.as_ref() {
            Some(id) => id.as_str(),
            None => "".as_ref()
        }
    }
    pub fn get_service(&self) -> &str {
        return self.service.as_str()
    }
    pub fn set_service(&mut self, service: String) {
        self.service = service
    }
    pub fn get_client(&self) -> Option<&Client> {
        self.client.as_ref()
    }
    pub fn get_data(&self) -> Option<&HashMap<String, Value>> {
        self.data.as_ref()
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


