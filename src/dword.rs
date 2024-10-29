use std::collections::HashMap;
use std::convert::Into;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};

use lazy_static::lazy_static;
use lers_windows_macro::{FromInto, self_attr};
use windows::Win32::Foundation;
use windows::Win32::Foundation::WIN32_ERROR;
use windows::Win32::System::Services;
use windows::Win32::System::Services::{ENUM_SERVICE_TYPE,
                                       SERVICE_ERROR, SERVICE_START_TYPE,
                                       SERVICE_STATUS_CURRENT_STATE};

#[derive(Debug, FromInto)]
pub struct ServiceError(WIN32_ERROR);

impl Hash for ServiceError {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.0.hash(state)
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
            write!(f, "错误({}):{}", self.0.0, SERVICE_ERRORS.get(self).unwrap())
        } else {
            write!(f, "未知错误({}),请查看官方文档", self.0.0)
        }
    }
}

#[self_attr(
    Foundation::ERROR_ACCESS_DENIED,
    Foundation::ERROR_CIRCULAR_DEPENDENCY,
    Foundation::ERROR_DUPLICATE_SERVICE_NAME,
    Foundation::ERROR_INVALID_HANDLE,
    Foundation::ERROR_INVALID_NAME,
    Foundation::ERROR_INVALID_PARAMETER,
    Foundation::ERROR_INVALID_SERVICE_ACCOUNT,
    Foundation::ERROR_SERVICE_EXISTS,
    Foundation::ERROR_SERVICE_MARKED_FOR_DELETE,
    Foundation::ERROR_PATH_NOT_FOUND,
    Foundation::ERROR_SERVICE_ALREADY_RUNNING,
    Foundation::ERROR_SERVICE_DATABASE_LOCKED,
    Foundation::ERROR_SERVICE_DEPENDENCY_DELETED,
    Foundation::ERROR_SERVICE_DEPENDENCY_FAIL,
    Foundation::ERROR_SERVICE_DISABLED,
    Foundation::ERROR_SERVICE_LOGON_FAILED,
    Foundation::ERROR_SERVICE_NO_THREAD,
    Foundation::ERROR_SERVICE_REQUEST_TIMEOUT
)]
impl ServiceError {}

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
            (
                ServiceError::ERROR_PATH_NOT_FOUND,
                "找不到服务二进制文件。",
            ),
            (
                ServiceError::ERROR_SERVICE_ALREADY_RUNNING,
                "服务的实例已在运行。"
            ),
            (
                ServiceError::ERROR_SERVICE_DATABASE_LOCKED,
                "数据库已锁定。"
            ),
            (
                ServiceError::ERROR_SERVICE_DEPENDENCY_DELETED,
                "该服务依赖于不存在或已标记为删除的服务。"
            ),
            (
                ServiceError::ERROR_SERVICE_DEPENDENCY_FAIL,
                "该服务依赖于另一个无法启动的服务。"
            ),
            (
                ServiceError::ERROR_SERVICE_DISABLED,
                "服务已被禁用。"
            ),
            (
                ServiceError::ERROR_SERVICE_LOGON_FAILED,
                "由于登录失败而无法启动服务。 如果将服务配置为在没有“作为服务登录”权限的帐户下运行，则会发生此错误。"
            ),
            (
                ServiceError::ERROR_SERVICE_NO_THREAD,
                "无法为服务创建线程。"
            ),
            (
                ServiceError::ERROR_SERVICE_REQUEST_TIMEOUT,
                "服务的进程已启动，但它未调用 StartServiceCtrlDispatcher，或者调用 StartServiceCtrlDispatcher 的线程可能在控制处理程序函数中被阻止。"
            )
        ]);
        map
    };
}

#[derive(Debug, FromInto)]
pub struct ServiceStatus(SERVICE_STATUS_CURRENT_STATE);

impl Hash for ServiceStatus {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.0.hash(state)
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
                self.0.0,
                SERVICE_STATUS.get(self).unwrap()
            )
        } else {
            write!(f, "未知服务状态({}),请查看官方文档", self.0.0)
        }
    }
}

lazy_static! {
    static ref SERVICE_STATUS: HashMap<ServiceStatus, &'static str> = {
        let result = HashMap::from([
            (ServiceStatus::SERVICE_CONTINUE_PENDING, "服务即将继续。"),
            (ServiceStatus::SERVICE_PAUSE_PENDING, "服务即将暂停。"),
            (ServiceStatus::SERVICE_PAUSED, "服务已暂停。"),
            (ServiceStatus::SERVICE_RUNNING, "服务正在运行。"),
            (ServiceStatus::SERVICE_START_PENDING, "服务正在启动。"),
            (ServiceStatus::SERVICE_STOP_PENDING, "服务正在停止。"),
            (ServiceStatus::SERVICE_STOPPED, "服务未运行。"),
        ]);
        result
    };
}

#[self_attr(
    Services::SERVICE_CONTINUE_PENDING,
    Services::SERVICE_PAUSE_PENDING,
    Services::SERVICE_PAUSED,
    Services::SERVICE_RUNNING,
    Services::SERVICE_START_PENDING,
    Services::SERVICE_STOP_PENDING,
    Services::SERVICE_STOPPED
)]
impl ServiceStatus {}

#[derive(FromInto)]
pub struct ScManagerAccess(u32);

#[self_attr(
    Services::SC_MANAGER_ALL_ACCESS,
    Services::SC_MANAGER_CREATE_SERVICE,
    Services::SC_MANAGER_CONNECT,
    Services::SC_MANAGER_ENUMERATE_SERVICE,
    Services::SC_MANAGER_LOCK,
    Services::SC_MANAGER_MODIFY_BOOT_CONFIG,
    Services::SC_MANAGER_QUERY_LOCK_STATUS
)]
impl ScManagerAccess {
    pub const GENERIC_READ: ScManagerAccess =
        ScManagerAccess(Services::SC_MANAGER_ENUMERATE_SERVICE | Services::SC_MANAGER_QUERY_LOCK_STATUS);
    pub const GENERIC_WRITE: ScManagerAccess =
        ScManagerAccess(Services::SC_MANAGER_CREATE_SERVICE | Services::SC_MANAGER_MODIFY_BOOT_CONFIG);
    pub const GENERIC_EXECUTE: ScManagerAccess = ScManagerAccess(Services::SC_MANAGER_CONNECT | Services::SC_MANAGER_LOCK);
    pub const GENERIC_ALL: ScManagerAccess = ScManagerAccess::SC_MANAGER_ALL_ACCESS;
}

#[derive(FromInto)]
pub struct ServiceAccess(u32);

#[self_attr(
    Services::SERVICE_ALL_ACCESS,
    Services::SERVICE_CHANGE_CONFIG,
    Services::SERVICE_ENUMERATE_DEPENDENTS,
    Services::SERVICE_INTERROGATE,
    Services::SERVICE_PAUSE_CONTINUE,
    Services::SERVICE_QUERY_CONFIG,
    Services::SERVICE_QUERY_STATUS,
    Services::SERVICE_START,
    Services::SERVICE_STOP,
    Services::SERVICE_USER_DEFINED_CONTROL
)]
impl ServiceAccess {
    pub const DELETE: ServiceAccess = ServiceAccess(0x10000u32);
    pub const READ_CONTROL: ServiceAccess = ServiceAccess(0x20000u32);
    pub const WRITE_DAC: ServiceAccess = ServiceAccess(0x40000u32);
    pub const WRITE_OWNER: ServiceAccess = ServiceAccess(0x80000u32);
    pub const GENERIC_READ: ServiceAccess = ServiceAccess(Services::SERVICE_QUERY_CONFIG
        | Services::SERVICE_QUERY_STATUS
        | Services::SERVICE_INTERROGATE
        | Services::SERVICE_ENUMERATE_DEPENDENTS);
    pub const GENERIC_WRITE: ServiceAccess = ServiceAccess::SERVICE_CHANGE_CONFIG;
    pub const GENERIC_EXECUTE: ServiceAccess =
        ServiceAccess(Services::SERVICE_START | Services::SERVICE_STOP | Services::SERVICE_PAUSE_CONTINUE | Services::SERVICE_USER_DEFINED_CONTROL);
}

#[derive(FromInto)]
pub struct ServiceType(ENUM_SERVICE_TYPE);

#[self_attr(
    Services::SERVICE_WIN32_OWN_PROCESS,
    Services::SERVICE_ADAPTER,
    Services::SERVICE_FILE_SYSTEM_DRIVER,
    Services::SERVICE_KERNEL_DRIVER,
    Services::SERVICE_RECOGNIZER_DRIVER,
    Services::SERVICE_WIN32_SHARE_PROCESS
)]
impl ServiceType {}

#[derive(FromInto)]
pub struct ServiceStartType(SERVICE_START_TYPE);

#[self_attr(
    Services::SERVICE_AUTO_START,
    Services::SERVICE_BOOT_START,
    Services::SERVICE_DEMAND_START,
    Services::SERVICE_DISABLED,
    Services::SERVICE_SYSTEM_START
)]
impl ServiceStartType {}


#[derive(FromInto)]
pub struct ServiceErrorControl(SERVICE_ERROR);

#[self_attr(
    Services::SERVICE_ERROR_CRITICAL,
    Services::SERVICE_ERROR_IGNORE,
    Services::SERVICE_ERROR_NORMAL,
    Services::SERVICE_ERROR_SEVERE
)]
impl ServiceErrorControl {}

#[derive(FromInto)]
pub struct ServiceControlCode(u32);

#[self_attr(
    Services::SERVICE_CONTROL_CONTINUE,
    Services::SERVICE_CONTROL_INTERROGATE,
    Services::SERVICE_CONTROL_NETBINDADD,
    Services::SERVICE_CONTROL_NETBINDDISABLE,
    Services::SERVICE_CONTROL_NETBINDENABLE,
    Services::SERVICE_CONTROL_NETBINDREMOVE,
    Services::SERVICE_CONTROL_PARAMCHANGE,
    Services::SERVICE_CONTROL_PAUSE,
    Services::SERVICE_CONTROL_STOP
)]
impl ServiceControlCode {}