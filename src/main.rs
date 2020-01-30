#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate lazy_static;

use std::string::String;
use rocket::request::Form;
use crate::client_url::ClientUrl;
use rocket::Request;
use rocket::request::FromRequest;
use rocket::request::Outcome;
use crate::response::response_invalid_param;
use crate::response::BaseState;
use bussiness_code::*;
use crate::response::SimpleSdkResponse;
mod preprocessor_simple_sdk_client_json;
mod client_url;
mod response;
mod request;
mod common_util;
mod bussiness_code;
mod simple_sdk_validator;


/**
ref doc:
https://codomatic.com/post/rust/rust_rocket/
https://stackoverflow.com/questions/54865824/return-json-with-an-http-status-other-than-200-in-rocket/54867136
*/

#[post("/client/<service_name>?<client_url..>", data ="<body>")]
fn client_dispatcher(
    service_name: String,
    body: String,
    client_url: Option<Form<ClientUrl>>,
    http_request: RocketRequest,
) -> Result<String, String> {

    let now: u64 = common_util::time_common::get_timestamp_millis();

    let client_url = match client_url {
        None => {
            return Ok(response_invalid_param(&service_name,
                                             now,
                                             None,
                                             BaseState::new_state_with_param(ERROR_REQUEST_PARAMETERS.code, ERROR_REQUEST_PARAMETERS.msg, "url参数缺失")));
        },
        Some(client_url) => client_url,
    };

    //url校验
    let client_url_params =  match client_url.validate(&service_name) {
        None => {
            return Ok(response_invalid_param(&service_name,
                                             now,
                                             None,
                                             BaseState::new_state_with_param(ERROR_REQUEST_PARAMETERS.code, ERROR_REQUEST_PARAMETERS.msg, "url校验错误")));
        },
        Some(client_url) => client_url,
    };

    //body处理
    let request = match preprocessor_simple_sdk_client_json::process_request(&service_name, &body, now, &http_request){
        Ok(request) => {
            request
        },
        Err(_error) => {
            return Err(format!("decode error, {} ", body));
        },
    };

    let mut response = SimpleSdkResponse::new_repsonse(format!("{}", now));
    //校验请求体
    preprocessor_simple_sdk_client_json::processing(&request, &mut response);
    if response.get_state().get_code() != SUCCESS.code {
        return Ok(response_invalid_param(&service_name,
                                         now,
                                         None,
                                         BaseState::new_state_with_all_param(response.get_state().get_code(), response.get_state().get_msg(), response.get_state().get_desc(), response.get_state().get_sub_code())));
    }



    //let request: SimpleSdkRequest = serde_json::from_str(body.as_str()).unwrap();

    //let response = SimpleSdkResponse::new_repsonse();
    //let result = serde_json::to_value(response)?;

    return Ok(format!("request, {:?} ", request));



}



pub struct RocketRequest{
    remote_ip: String,
    remote_port: u16,

}

impl<'a, 'r> FromRequest<'a, 'r> for RocketRequest {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {

        let rocket_request = RocketRequest{
            remote_ip: match request.real_ip(){
                Some(ip_addr) => ip_addr.to_string(),
                None => "".to_string(),
            },
            remote_port: match request.remote(){
                Some(socket_addr) => socket_addr.port(),
                None => 0,
            }
        };

        Outcome::Success(rocket_request)
    }
}



fn main() {
    rocket::ignite().mount("/", routes![client_dispatcher]).launch();
}

