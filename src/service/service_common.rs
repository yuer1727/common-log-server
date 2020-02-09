use crate::request::SimpleSdkRequest;
use crate::response::SimpleSdkResponse;
use std::collections::HashMap;
use crate::common_util::time_common::get_timestamp_millis;
use crate::response::BaseState;
use crate::service::commmon_log_service::CommonLogService;
use crate::service::commmon_log_service2::CommonLogService2;

pub trait IService: Sync {
    fn execute(&self, _request: &SimpleSdkRequest) -> SimpleSdkResponse {
        SimpleSdkResponse::new_repsonse_with_state(format!("{}", get_timestamp_millis()), BaseState::new_state())
    }
}


lazy_static! {
  pub static ref SERVICE_LIST: HashMap<&'static str, Box<dyn IService>> = {
        let mut m : HashMap<&'static str, Box<dyn IService>> = HashMap::new();
        m.insert("common.log.collect_1.0", Box::new(CommonLogService{}));
        m.insert("common.log.collect2_1.0", Box::new(CommonLogService2{}));
        m
  };
}

pub fn get_service<'a>(service_name: &'a str, ver: &'a str) -> Option<&'a Box<dyn IService>> {
    if !"0".eq_ignore_ascii_case(ver) {
        let mut full_key = String::from(service_name);
        full_key.push_str("_");
        full_key.push_str(ver);
        return SERVICE_LIST.get(full_key.as_str());
    }

    return SERVICE_LIST.get(service_name);
}