use std::collections::HashMap;
use serde_json::Value;
use std::string::String;



#[derive(Clone, Borrow)]
struct Client {
    // 客户端版本
    ve: String,
    // 客户端操作系统
    os: String,
    // 客户端操作系统框架
    fr: String,
    // 游戏编号
    gameId: String,
    // 渠道编号
    channelId: String,
    // 客户端序安装标识
    si: String,
    // 客户端扩展参数
    ex: HashMap<String, Value>,
}

impl Client{
    pub fn getVe(&self) -> &str {
        return self.ve
    }
    pub fn getOs(&self) -> &str {
        return self.os
    }
    pub fn getFr(&self) -> &str {
        return self.fr
    }
    pub fn getGameId(&self) -> &str {
        return self.gameId
    }
    pub fn getChannelId(&self) -> &str {
        return self.channelId
    }
    pub fn getSi(&self) -> &str {
        return self.si
    }
    pub fn getEx(&self) -> &HashMap {
        return self.ex
    }


}


#[derive(Clone)]
pub struct SimpleSdkRequest {

    //请求唯一标识
    id: String,
    // 调用的服务，对应是service的具体方法
    service: String,
    // 客户端公共参数
    client: Client,
    // 数据
    data: HashMap<String, Value>,
    //接口的版本号,默认1.0
    ver: String,
    // 请求客户端的ip （服务端补齐参数）
    ip: String,
    // 请求客户端的port (服务端补齐参数）
    port: String,
    // 请求来源{@link RequestFrom} （服务端补齐参数）
    from: String,
    // 数据格式
    df: String,
    // 服务器接收到请求的时间 （服务端补齐参数）
    startTime: i64,
    //请求urlParam
    urlParam: String,

}


impl SimpleSdkRequest{
    pub fn getId(&self) -> &str {
        return self.id
    }
    pub fn getService(&self) -> &str {
        return self.service
    }
    pub fn getClient(&self) -> &Client {
        return &(self.client)
    }
    pub fn getData(&self) -> &HashMap<String, Value> {
        return self.id
    }
    pub fn getVer(&self) -> &str {
        return self.ver
    }
    pub fn getIp(&self) -> &str {
        return self.ip
    }
    pub fn getPort(&self) -> &str {
        return self.port
    }
    pub fn getFrom(&self) -> &str {
        return self.from
    }
    pub fn setFrom(&mut self, from: String) {
        self.from = from
    }
    pub fn getDf(&self) -> &str {
        return self.df
    }
    pub fn setDf(&mut self, df: String) {
        self.df = df
    }
    pub fn getStartTime(&self) -> &i64 {
        return &(self.startTime)
    }
    pub fn setStartTime(&mut self, startTime: i64) {
        self.startTime = startTime
    }
    pub fn getUrlParam(&self) -> &str {
        return self.urlParam
    }
    pub fn setUrlParam(&mut self, urlParam: String) {
        self.urlParam = urlParam
    }
}
