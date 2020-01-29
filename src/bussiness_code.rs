


pub struct  BussinessCode {
    pub code: i32,
    pub msg: &'static str,
}

pub static SUCCESS: BussinessCode = BussinessCode{ code: 2000000, msg: "操作成功"};
pub static ERROR_REQUEST_PARAMETERS: BussinessCode = BussinessCode{ code: 4000000, msg: "无效的请求参数"};
pub static ERROR_BUZ_REQUEST_PARAMETERS: BussinessCode = BussinessCode{ code: 4000001, msg: "无效的业务请求参数"};
pub static INTERNAL_ERROR: BussinessCode = BussinessCode{ code: 5000000, msg: "服务器内部错误"};
pub static UNKNOWN_ERROR: BussinessCode = BussinessCode{ code: 5000001, msg: "未知错误"};
pub static INVALID_SYSTEM_PARAMETER: BussinessCode = BussinessCode{ code: 5000002, msg: "系统参数无效"};
pub static ERROR_SERVICE_DOWNGRADE: BussinessCode = BussinessCode{ code: 5000003, msg: "服务器服务降级错误"};

