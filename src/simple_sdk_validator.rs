use crate::request::SimpleSdkRequest;
use crate::response::SimpleSdkResponse;
use crate::common_util::time_common::get_timestamp_millis;
use std::collections::HashMap;
use crate::bussiness_code::ERROR_REQUEST_PARAMETERS;
use crate::response::BaseState;
use crate::request::Client;

pub fn validate(request: &SimpleSdkRequest, response: &mut SimpleSdkResponse) {

    if request.get_id().is_empty() {
        response.set_id(format!("{}", get_timestamp_millis()));
    } else {
        response.set_id(String::from(request.get_id()));
    }

    if check_length(request.get_id(), 32) {
        return response_error_parameter(response, "请求id参数为空或长度超长");
    }

    if check_length(request.get_client().unwrap_or(&Client::default()).get_ve(), 15) {
        return response_error_parameter(response, "请求ve参数为空或长度超长");
    }

    if check_length(request.get_client().unwrap_or(&Client::default()).get_os(), 10) {
        return response_error_parameter(response, "请求os参数为空或长度超长");
    }

    if check_length(request.get_client().unwrap_or(&Client::default()).get_fr(), 30) {
        return response_error_parameter(response, "请求fr参数为空或长度超长");
    }

    if check_length(request.get_client().unwrap_or(&Client::default()).get_game_id(), 8) {
        return response_error_parameter(response, "请求gameId参数为空或长度超长");
    }

    if check_length(request.get_client().unwrap_or(&Client::default()).get_channel_id(), 8) {
        return response_error_parameter(response, "请求channelId参数为空或长度超长");
    }

    if check_length(request.get_client().unwrap_or(&Client::default()).get_si(), 100) {
        return response_error_parameter(response, "请求si参数为空或长度超长");
    }
}

fn response_error_parameter(response: &mut SimpleSdkResponse, desc: &'static str) {
    response.set_state(BaseState::new_state_with_param(ERROR_REQUEST_PARAMETERS.code, ERROR_REQUEST_PARAMETERS.msg, desc));
    response.set_data(HashMap::new())
}

fn check_length(field_value: &str , length: usize) -> bool {

    if field_value.is_empty() || field_value.len() > length {

        return true;
    }

    return false;
}


