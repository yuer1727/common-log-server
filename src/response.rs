use std::collections::HashMap;
use serde_json::Value;
use std::string::String;
use crate::request::SimpleSdkRequest;
use serde::{Deserialize, Serialize};


/**
doc:
多层结构体的生命周期问题： https://serde.rs/lifetimes.html#borrowing-data-in-a-derived-impl
*/

static SUCCESS_CODE: i32 = 2000000;
static SUCCESS_MSG: &'static str = "操作成功";
static INTERNAL_ERROR: &'static str = r#"{
        "id": "default",
        "state": {
            "code": 5000000,
            "msg": "内部错误",
            "desc": "内部错误",
            "sub_code": 5000000,
        },

        "data": {

        }
    })"#;


#[derive(Clone, Serialize, Deserialize)]
pub struct BaseState<'a> {
    code: i32,
    msg: &'a str,
    desc: &'a str,
    sub_code: i32,

}

impl<'a> BaseState<'a> {
    pub fn new_state() -> BaseState<'a> {
        BaseState {
            code: SUCCESS_CODE,
            msg: SUCCESS_MSG,
            desc: "",
            sub_code: SUCCESS_CODE,
        }
    }
    pub fn new_state_with_param(code: i32, msg: &'a str, desc: &'a str) -> BaseState<'a> {
        BaseState {
            code,
            msg,
            desc,
            sub_code: code,
        }
    }
    pub fn new_state_with_all_param(code: i32, msg: &'a str, desc: &'a str, sub_code: i32) -> BaseState<'a> {
        BaseState {
            code,
            msg,
            desc,
            sub_code,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SimpleSdkResponse<'a> {
    id: String,
    #[serde(borrow)]
    state: BaseState<'a>,
    data: HashMap<String, Value>,

}

impl<'a> SimpleSdkResponse<'a> {
    pub fn new_repsonse(id: String) -> SimpleSdkResponse<'a> {
        SimpleSdkResponse {
            id,
            state: BaseState::new_state(),
            data: HashMap::new(),
        }
    }

    pub fn new_repsonse_with_state(id: String, state: BaseState<'a>) -> SimpleSdkResponse<'a> {
        SimpleSdkResponse {
            id,
            state,
            data: HashMap::new(),
        }
    }

    pub fn set_data(&mut self, data_map: HashMap<String, Value>) {
        self.data = data_map;
    }
}

pub fn response_invalid_param(service_name: &String, start_time: u64, simple_sdk_request: Option<&SimpleSdkRequest>, state: BaseState) -> String {
    let new_simple_sdk_request = &SimpleSdkRequest::new(start_time, service_name);
    let simple_sdk_request = simple_sdk_request.unwrap_or(new_simple_sdk_request);

    let response = SimpleSdkResponse {
        id: String::from(simple_sdk_request.get_id()),
        state,
        data: HashMap::new(),
    };


    let result = match serde_json::to_string(&response) {
        Ok(result) => result,
        Err(_error) => INTERNAL_ERROR.to_string(),
    };
    return result;
}

