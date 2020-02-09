

use std::string::String;
use serde::{Deserialize, Serialize};
use crate::request::SimpleSdkRequest;
use crate::response::SimpleSdkResponse;
use crate::common_util::time_common::get_timestamp_millis;
use crate::simplethreadpool::simplethreadpool;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessLog<'a>{
    req_time: u64,
    service: String,
    request: SimpleSdkRequest,
    #[serde(borrow)]
    response: SimpleSdkResponse<'a>,
    exec_time: u64,
}


impl<'a> AccessLog<'a> {
    pub fn new(req_time: u64, service: &str, request: SimpleSdkRequest, response: SimpleSdkResponse<'a>) -> AccessLog<'a> {
        return AccessLog{
            req_time,
            service: String::from(service),
            request,
            response,
            exec_time: get_timestamp_millis() - req_time,
        }
    }
}


pub fn write_access_log(request: SimpleSdkRequest, response: SimpleSdkResponse<'static>) {
    simplethreadpool::getThreadPoolWithoutNone("log").execute(move || {
        let start_time = request.get_start_time();
        let service_name = String::from(request.get_service());
        let accessLog = AccessLog::new(start_time, service_name.as_str(), request, response);
        info!(target: "app::accesslog", "{}", serde_json::to_string(&accessLog).unwrap());
    });
}




