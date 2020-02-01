use crate::service::service_common::IService;
use crate::request::SimpleSdkRequest;
use crate::response::SimpleSdkResponse;
use crate::common_util::time_common::get_timestamp_millis;
use crate::response::BaseState;


impl IService for CommonLogService {
    fn execute(&self, request: &SimpleSdkRequest) -> SimpleSdkResponse{
        SimpleSdkResponse::new_repsonse_with_state(format!("{}", get_timestamp_millis()), BaseState::new_state())
    }
}


pub struct CommonLogService {

}

