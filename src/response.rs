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


#[derive(Debug, Clone, Serialize, Deserialize)]
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

    pub fn set_code(&mut self, code: i32) {
        self.code = code;
    }
    pub fn get_code(&self) -> i32 {
        self.code
    }
    pub fn get_sub_code(&self) -> i32 {
        self.sub_code
    }
    pub fn set_sub_code(&mut self, sub_code: i32) {
        self.sub_code = sub_code;
    }
    pub fn get_msg(&self) -> &'a str {
        self.msg
    }
    pub fn set_msg(&mut self, msg: &'a str) {
        self.msg = msg;
    }
    pub fn get_desc(&self) -> &'a str {
        self.desc
    }
    pub fn set_desc(&mut self, desc: &'a str) {
        self.desc = desc;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

    pub fn get_state(&self) -> &BaseState<'a> {
        &self.state
    }

    pub fn set_state(&mut self, state: BaseState<'a>) {
        self.state = state;
    }

    pub fn set_data(&mut self, data_map: HashMap<String, Value>) {
        self.data = data_map;
    }

    pub fn get_data(&self) -> &HashMap<String, Value> {
        &self.data
    }

    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    pub fn get_id(&self) -> &String {
        &self.id
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

