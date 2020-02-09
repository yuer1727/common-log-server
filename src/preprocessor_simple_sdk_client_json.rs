use crate::request::SimpleSdkRequest;
use crate::RocketRequest;
use crate::response::SimpleSdkResponse;
use crate::simple_sdk_validator::validate;
use crate::response;
use crate::response::BaseState;
use crate::simplelog::access_log;


pub fn process_request(server_name: &String, body: &String, start_time: u64, http_request: &RocketRequest) -> Result<SimpleSdkRequest, String> {

    //body反序列化
    let mut sdk_request: SimpleSdkRequest = match serde_json::from_str(body.as_str()) {
        Ok(request) => {
            request
        },
        Err(_error) => {
            return Err(format!("decode error, {} ", body));
        },
    };

    sdk_request.set_service(server_name.clone());
    sdk_request.set_ver(String::from("1.0"));
    sdk_request.set_from(String::from("cs"));
    sdk_request.set_df(String::from("json"));
    sdk_request.set_start_time(start_time);
    sdk_request.set_url_param(String::from(""));
    sdk_request.set_ip(http_request.remote_ip.to_string());
    sdk_request.set_port(format!("{}",http_request.remote_port));
    return Ok(sdk_request.clone())

}

pub fn processing(request: &SimpleSdkRequest, response: &mut SimpleSdkResponse) {

    validate(request, response);
}


pub fn convert_response(request: &SimpleSdkRequest, response: &SimpleSdkResponse<'static>) -> Result<String, String> {

    //TODO 实现插件调用

    //write accesslog
    access_log::write_access_log(request.clone(), response.clone());

    //响应结果
    return Ok(response::response_invalid_param(&String::from(request.get_service()),
                                     request.get_start_time(),
                                     Some(request),
                                     BaseState::new_state_with_all_param(response.get_state().get_code(), response.get_state().get_msg(), response.get_state().get_desc(), response.get_state().get_sub_code())));

}






