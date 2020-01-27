use std::collections::HashMap;
use serde_json::Value;
use std::string::String;

static SUCCESS_CODE: i32 = 2000000;
static SUCCESS_MSG: &'static str  = "操作成功";


#[derive(Clone)]
pub struct BaseState {

    code: i32,
    msg: String,
    desc: String,
    sub_code: i32,

}

impl BaseState{
    pub fn new_state() -> BaseState{
        BaseState{
            code: SUCCESS_CODE,
            msg: String::from(SUCCESS_MSG),
            desc: String::from(""),
            sub_code: SUCCESS_CODE,
        }
    }
    pub fn new_state_with_param(code: i32, msg: String, desc: String) -> BaseState{
        BaseState{
            code,
            msg,
            desc,
            sub_code: code,
        }
    }
    pub fn new_state_with_all_param(code: i32, msg: String, desc: String, sub_code: i32) -> BaseState{
        BaseState{
            code,
            msg,
            desc,
            sub_code,
        }
    }
}

#[derive(Clone)]
pub struct SimpleSdkResponse {

    id: String,
    state : BaseState,
    data: HashMap<String, Value>,

}

impl SimpleSdkResponse{
    pub fn new_repsonse(id: String) -> SimpleSdkResponse{
        SimpleSdkResponse{
            id,
            state: BaseState::new_state(),
            data: HashMap::new(),
        }
    }

    pub fn new_repsonse_with_state(id: String, state: BaseState) -> SimpleSdkResponse{
        SimpleSdkResponse{
            id,
            state,
            data: HashMap::new(),
        }
    }

    pub fn set_data(&mut self, data_map: HashMap<String, Value>) {
        self.data = data_map;
    }
}

pub fn response_invalid_param(service_name: String, start_time: i64, state: BaseState) -> String {
    return String::new()
}

