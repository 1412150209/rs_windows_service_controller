use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use windows::Win32::Foundation;
use windows::Win32::Foundation::WIN32_ERROR;
use windows::Win32::System::Services;
use windows::Win32::System::Services::{
    ENUM_SERVICE_TYPE, SERVICE_ERROR, SERVICE_START_TYPE, SERVICE_STATUS_CURRENT_STATE,
};

#[derive(Hash, Debug)]
pub struct ServiceError(u32);

impl From<WIN32_ERROR> for ServiceError {
    fn from(value: WIN32_ERROR) -> Self {
        ServiceError(value.0)
    }
}

impl PartialEq<Self> for ServiceError {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for ServiceError {}

impl Display for ServiceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if SERVICE_ERRORS.contains_key(self) {
            write!(f, "错误({}):{}", self.0, SERVICE_ERRORS.get(self).unwrap())
        } else {
            write!(f, "未知错误({}),请查看官方文档", self.0)
        }
    }
}

impl ServiceError {
    pub const ERROR_ACCESS_DENIED: ServiceError = ServiceError {
        0: Foundation::ERROR_ACCESS_DENIED.0,
    };
    pub const ERROR_CIRCULAR_DEPENDENCY: ServiceError = ServiceError {
        0: Foundation::ERROR_CIRCULAR_DEPENDENCY.0,
    };
    pub const ERROR_DUPLICATE_SERVICE_NAME: ServiceError = ServiceError {
        0: Foundation::ERROR_DUPLICATE_SERVICE_NAME.0,
    };
    pub const ERROR_INVALID_HANDLE: ServiceError = ServiceError {
        0: Foundation::ERROR_INVALID_HANDLE.0,
    };
    pub const ERROR_INVALID_NAME: ServiceError = ServiceError {
        0: Foundation::ERROR_INVALID_NAME.0,
    };
    pub const ERROR_INVALID_PARAMETER: ServiceError = ServiceError {
        0: Foundation::ERROR_INVALID_PARAMETER.0,
    };
    pub const ERROR_INVALID_SERVICE_ACCOUNT: ServiceError = ServiceError {
        0: Foundation::ERROR_INVALID_SERVICE_ACCOUNT.0,
    };
    pub const ERROR_SERVICE_EXISTS: ServiceError = ServiceError {
        0: Foundation::ERROR_SERVICE_EXISTS.0,
    };
    pub const ERROR_SERVICE_MARKED_FOR_DELETE: ServiceError = ServiceError {
        0: Foundation::ERROR_SERVICE_MARKED_FOR_DELETE.0,
    };
}

lazy_static! {
    static ref SERVICE_ERRORS: HashMap<ServiceError, &'static str> = {
        let map = HashMap::from([
            (
                ServiceError::ERROR_ACCESS_DENIED,
                "SCM 数据库的句柄没有 SC_MANAGER_CREATE_SERVICE 访问权限。",
            ),
            (
                ServiceError::ERROR_CIRCULAR_DEPENDENCY,
                "指定了循环服务依赖项。",
            ),
            (
                ServiceError::ERROR_DUPLICATE_SERVICE_NAME,
                "显示名称已作为服务名称或其他显示名称存在于服务控制管理器数据库中。",
            ),
            (
                ServiceError::ERROR_INVALID_HANDLE,
                "指定服务控制管理器数据库的句柄无效。",
            ),
            (ServiceError::ERROR_INVALID_NAME, "指定的服务名称无效。"),
            (ServiceError::ERROR_INVALID_PARAMETER, "指定的参数无效。"),
            (
                ServiceError::ERROR_INVALID_SERVICE_ACCOUNT,
                "ServiceStartName 参数中指定的用户帐户名不存在。",
            ),
            (
                ServiceError::ERROR_SERVICE_EXISTS,
                "此数据库中已存在指定的服务。",
            ),
            (
                ServiceError::ERROR_SERVICE_MARKED_FOR_DELETE,
                "指定的服务已存在于此数据库中，并且已标记为要删除。",
            ),
        ]);
        map
    };
}

pub type ScManagerAccess = u32;
pub type ServiceAccess = u32;
pub type ServiceType = ENUM_SERVICE_TYPE;
#[derive(Hash, Debug)]
pub struct ServiceStatus(u32);

impl From<SERVICE_STATUS_CURRENT_STATE> for ServiceStatus {
    fn from(value: SERVICE_STATUS_CURRENT_STATE) -> Self {
        ServiceStatus { 0: value.0 }
    }
}

impl PartialEq for ServiceStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for ServiceStatus {}

impl Display for ServiceStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if SERVICE_STATUS.contains_key(self) {
            write!(
                f,
                "服务状态({}):{}",
                self.0,
                SERVICE_STATUS.get(self).unwrap()
            )
        } else {
            write!(f, "未知服务状态({}),请查看官方文档", self.0)
        }
    }
}

lazy_static! {
    static ref SERVICE_STATUS: HashMap<ServiceStatus,&'static str> = {
        let result = HashMap::from([
            (ServiceStatus::SERVICE_CONTINUE_PENDING, "服务即将继续。"),
            (ServiceStatus::SERVICE_PAUSE_PENDING, "服务即将暂停。"),
            (ServiceStatus::SERVICE_PAUSED, "服务已暂停。"),
            (ServiceStatus::SERVICE_RUNNING, "服务正在运行。"),
            (ServiceStatus::SERVICE_START_PENDING, "服务正在启动。"),
            (ServiceStatus::SERVICE_STOP_PENDING, "服务正在停止。"),
            (ServiceStatus::SERVICE_STOPPED, "服务未运行。")
        ]);
        result
    };
}

impl ServiceStatus {
    pub const SERVICE_CONTINUE_PENDING: ServiceStatus = ServiceStatus {
        0: Services::SERVICE_CONTINUE_PENDING.0,
    };
    pub const SERVICE_PAUSE_PENDING: ServiceStatus = ServiceStatus {
        0: Services::SERVICE_PAUSE_PENDING.0,
    };
    pub const SERVICE_PAUSED: ServiceStatus = ServiceStatus {
        0: Services::SERVICE_PAUSED.0,
    };
    pub const SERVICE_RUNNING: ServiceStatus = ServiceStatus {
        0: Services::SERVICE_RUNNING.0,
    };
    pub const SERVICE_START_PENDING: ServiceStatus = ServiceStatus {
        0: Services::SERVICE_START_PENDING.0,
    };
    pub const SERVICE_STOP_PENDING: ServiceStatus = ServiceStatus {
        0: Services::SERVICE_STOP_PENDING.0,
    };
    pub const SERVICE_STOPPED: ServiceStatus = ServiceStatus {
        0: Services::SERVICE_STOPPED.0,
    };
}
pub type ServiceStartType = SERVICE_START_TYPE;
pub type ServiceErrorControl = SERVICE_ERROR;

pub mod sc_manager_access {
    use windows::Win32::System::Services;

    use crate::dword::ScManagerAccess;

    pub const SC_MANAGER_ALL_ACCESS: ScManagerAccess = Services::SC_MANAGER_ALL_ACCESS;
    pub const SC_MANAGER_CREATE_SERVICE: ScManagerAccess = Services::SC_MANAGER_CREATE_SERVICE;
    pub const SC_MANAGER_CONNECT: ScManagerAccess = Services::SC_MANAGER_CONNECT;
    pub const SC_MANAGER_ENUMERATE_SERVICE: ScManagerAccess =
        Services::SC_MANAGER_ENUMERATE_SERVICE;
    pub const SC_MANAGER_LOCK: ScManagerAccess = Services::SC_MANAGER_LOCK;
    pub const SC_MANAGER_MODIFY_BOOT_CONFIG: ScManagerAccess =
        Services::SC_MANAGER_MODIFY_BOOT_CONFIG;
    pub const SC_MANAGER_QUERY_LOCK_STATUS: ScManagerAccess =
        Services::SC_MANAGER_QUERY_LOCK_STATUS;

    pub const GENERIC_READ: ScManagerAccess =
        SC_MANAGER_ENUMERATE_SERVICE | SC_MANAGER_QUERY_LOCK_STATUS;
    pub const GENERIC_WRITE: ScManagerAccess =
        SC_MANAGER_CREATE_SERVICE | SC_MANAGER_MODIFY_BOOT_CONFIG;
    pub const GENERIC_EXECUTE: ScManagerAccess = SC_MANAGER_CONNECT | SC_MANAGER_LOCK;
    pub const GENERIC_ALL: ScManagerAccess = SC_MANAGER_ALL_ACCESS;
}

pub mod service_access {
    use windows::Win32::System::Services;

    use crate::dword::ServiceAccess;

    pub const SERVICE_ALL_ACCESS: ServiceAccess = Services::SERVICE_ALL_ACCESS;
    pub const SERVICE_CHANGE_CONFIG: ServiceAccess = Services::SERVICE_CHANGE_CONFIG;
    pub const SERVICE_ENUMERATE_DEPENDENTS: ServiceAccess = Services::SERVICE_ENUMERATE_DEPENDENTS;
    pub const SERVICE_INTERROGATE: ServiceAccess = Services::SERVICE_INTERROGATE;
    pub const SERVICE_PAUSE_CONTINUE: ServiceAccess = Services::SERVICE_PAUSE_CONTINUE;
    pub const SERVICE_QUERY_CONFIG: ServiceAccess = Services::SERVICE_QUERY_CONFIG;
    pub const SERVICE_QUERY_STATUS: ServiceAccess = Services::SERVICE_QUERY_STATUS;
    pub const SERVICE_START: ServiceAccess = Services::SERVICE_START;
    pub const SERVICE_STOP: ServiceAccess = Services::SERVICE_STOP;
    pub const SERVICE_USER_DEFINED_CONTROL: ServiceAccess = Services::SERVICE_USER_DEFINED_CONTROL;

    pub const DELETE: ServiceAccess = 0x10000u32;
    pub const READ_CONTROL: ServiceAccess = 0x20000u32;
    pub const WRITE_DAC: ServiceAccess = 0x40000u32;
    pub const WRITE_OWNER: ServiceAccess = 0x80000u32;

    pub const GENERIC_READ: ServiceAccess = SERVICE_QUERY_CONFIG
        | SERVICE_QUERY_STATUS
        | SERVICE_INTERROGATE
        | SERVICE_ENUMERATE_DEPENDENTS;
    pub const GENERIC_WRITE: ServiceAccess = SERVICE_CHANGE_CONFIG;
    pub const GENERIC_EXECUTE: ServiceAccess =
        SERVICE_START | SERVICE_STOP | SERVICE_PAUSE_CONTINUE | SERVICE_USER_DEFINED_CONTROL;
}

pub mod service_type {
    use windows::Win32::System::Services;

    use crate::dword::ServiceType;

    pub const SERVICE_WIN32_OWN_PROCESS: ServiceType = Services::SERVICE_WIN32_OWN_PROCESS;
    pub const SERVICE_ADAPTER: ServiceType = Services::SERVICE_ADAPTER;
    pub const SERVICE_FILE_SYSTEM_DRIVER: ServiceType = Services::SERVICE_FILE_SYSTEM_DRIVER;
    pub const SERVICE_KERNEL_DRIVER: ServiceType = Services::SERVICE_KERNEL_DRIVER;
    pub const SERVICE_RECOGNIZER_DRIVER: ServiceType = Services::SERVICE_RECOGNIZER_DRIVER;
    pub const SERVICE_WIN32_SHARE_PROCESS: ServiceType = Services::SERVICE_WIN32_SHARE_PROCESS;
}

pub mod service_start_type {
    use windows::Win32::System::Services;

    use crate::dword::ServiceStartType;

    pub const SERVICE_AUTO_START: ServiceStartType = Services::SERVICE_AUTO_START;
    pub const SERVICE_BOOT_START: ServiceStartType = Services::SERVICE_BOOT_START;
    pub const SERVICE_DEMAND_START: ServiceStartType = Services::SERVICE_DEMAND_START;
    pub const SERVICE_DISABLED: ServiceStartType = Services::SERVICE_DISABLED;
    pub const SERVICE_SYSTEM_START: ServiceStartType = Services::SERVICE_SYSTEM_START;
}

pub mod service_error_control {
    use windows::Win32::System::Services;

    use crate::dword::ServiceErrorControl;

    pub const SERVICE_ERROR_CRITICAL: ServiceErrorControl = Services::SERVICE_ERROR_CRITICAL;
    pub const SERVICE_ERROR_IGNORE: ServiceErrorControl = Services::SERVICE_ERROR_IGNORE;
    pub const SERVICE_ERROR_NORMAL: ServiceErrorControl = Services::SERVICE_ERROR_NORMAL;
    pub const SERVICE_ERROR_SEVERE: ServiceErrorControl = Services::SERVICE_ERROR_SEVERE;
}
