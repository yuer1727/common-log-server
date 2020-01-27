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
    client_url: Option<Form<ClientUrl>>
) -> Result<String, String> {

    let client_url = match client_url {
        None => return Err(format!("url param none")),
        Some(client_url) => client_url,
    };

    //url校验
    let client_url_params =  match client_url.validate(service_name) {
        None => return Err(format!("url error, {:?} ", client_url)),
        Some(client_url) => client_url,
    };

    //body处理
    let request: SimpleSdkRequest = match serde_json::from_str(body.as_str()) {
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


fn main() {
    rocket::ignite().mount("/", routes![client_dispatcher]).launch();
}

