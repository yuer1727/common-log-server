#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate rocket_contrib;

use std::string::String;
use rocket::request::Form;
use crate::client_url::ClientUrl;
use rocket::response::Responder;
use rocket::Request;
use rocket::Response;
use response::SimpleSdkResponse;
use request::SimpleSdkRequest;
use serde_json::Value;
use crate::preprocessor_simple_sdk_client_json::processRequest;
use rocket::request::FromRequest;
use rocket::request::Outcome;
use std::net::IpAddr;
use crate::response::response_invalid_param;
use crate::response::BaseState;

mod preprocessor_simple_sdk_client_json;
mod client_url;
mod response;
mod request;
mod common_util;



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
                                             BaseState::new_state_with_param(4000000, "异常请求".to_string(), "url参数缺失".to_string())));
        },
        Some(client_url) => client_url,
    };

    //url校验
    let client_url_params =  match client_url.validate(&service_name) {
        None => {
            return Ok(response_invalid_param(&service_name,
                                             now,
                                             None,
                                             BaseState::new_state_with_param(4000000, "异常请求".to_string(), "url校验错误".to_string())));
        },
        Some(client_url) => client_url,
    };

    //body处理
    let request = match processRequest(&service_name, &body, now, &http_request){
        Ok(request) => {
            request
        },
        Err(error) => {
            return Err(format!("decode error, {} ", body));
        },
    };


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
                Some(ipAddr) => ipAddr.to_string(),
                None => "".to_string(),
            },
            remote_port: match request.remote(){
                Some(socketAddr) => socketAddr.port(),
                None => 0,
            }
        };

        Outcome::Success(rocket_request)
    }
}



fn main() {
    rocket::ignite().mount("/", routes![client_dispatcher]).launch();
}

