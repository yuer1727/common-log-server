use crate::request::SimpleSdkRequest;
use rocket::Request;
use crate::RocketRequest;


pub fn processRequest(server_name: &String, body: &String, start_time: u64, http_request: &RocketRequest) -> Result<SimpleSdkRequest, String> {

    //body反序列化
    let mut sdk_request: SimpleSdkRequest = match serde_json::from_str(body.as_str()) {
        Ok(request) => {
            request
        },
        Err(error) => {
            return Err(format!("decode error, {} ", body));
        },
    };

    sdk_request.set_service(server_name.clone());
    sdk_request.set_from(String::from("cs"));
    sdk_request.set_df(String::from("json"));
    sdk_request.set_start_time(start_time);
    sdk_request.set_url_param(String::from(""));
    sdk_request.set_ip(http_request.remote_ip.to_string());
    sdk_request.set_port(format!("{}",http_request.remote_port));
    return Ok(sdk_request.clone())

}