use std::mem::size_of;

use widestring::U16CString;
use windows::core::PCWSTR;
use windows::Win32::Foundation::GetLastError;
use windows::Win32::Security::SC_HANDLE;
use windows::Win32::System::Services::{
    CreateServiceW, OpenSCManagerW, OpenServiceW, QueryServiceConfigW, QueryServiceStatus,
    QUERY_SERVICE_CONFIGW, SERVICE_STATUS,
};

use crate::dword::{
    sc_manager_access, service_access, ScManagerAccess, ServiceAccess, ServiceError,
    ServiceErrorControl, ServiceStartType, ServiceStatus, ServiceType,
};

mod dword;

/// windows服务类
pub struct WindowsService {
    sc_manager_handle: SC_HANDLE,
    service_handle: SC_HANDLE,
    pub config: ServiceConfig,
}

type ServiceConfig = QUERY_SERVICE_CONFIGW;

impl WindowsService {
    /// 通过服务名打开一个服务实例
    pub fn open(name: &str) -> Result<WindowsService, ServiceError> {
        let sc_manager_handle = Self::open_sc_manager(
            sc_manager_access::SC_MANAGER_CONNECT | sc_manager_access::GENERIC_WRITE,
        )?;
        let service_handle = Self::open_service(
            sc_manager_handle,
            name,
            service_access::GENERIC_READ | service_access::GENERIC_EXECUTE,
        )?;
        Ok(WindowsService {
            sc_manager_handle,
            service_handle,
            config: Self::get_config(service_handle)?,
        })
    }

    /// 请求当前服务状态
    pub fn query_service_status(&self) -> ServiceStatus {
        let mut status = SERVICE_STATUS::default();
        unsafe {
            QueryServiceStatus(self.service_handle, &mut status).unwrap();
        };
        status.dwCurrentState.into()
    }

    pub fn new(
        name: &str,
        display_name: Option<&str>,
        service_type: ServiceType,
        service_start_type: ServiceStartType,
        error_control: ServiceErrorControl,
        binary_path: Option<&str>,
        dependencies: Option<Vec<&str>>,
    ) -> Result<WindowsService, ServiceError> {
        let sc_manager_handle = Self::open_sc_manager(
            sc_manager_access::SC_MANAGER_CONNECT | sc_manager_access::GENERIC_WRITE,
        )?;
        let display_name = display_name.unwrap_or_else(|| name);
        let binary_path = binary_path.unwrap_or_else(|| "");
        let dependencies = match dependencies {
            None => Vec::<u16>::default(),
            Some(v) => {
                let mut result: Vec<u16> = Vec::new();
                for str in v {
                    result.push(str.parse::<u16>().unwrap())
                }
                result
            }
        };
        let service_handle = unsafe {
            CreateServiceW(
                sc_manager_handle,
                PCWSTR(U16CString::from_str(name).unwrap().as_ptr()),
                PCWSTR(U16CString::from_str(display_name).unwrap().as_ptr()),
                service_access::GENERIC_READ | service_access::GENERIC_EXECUTE,
                service_type,
                service_start_type,
                error_control,
                PCWSTR(U16CString::from_str(binary_path).unwrap().as_ptr()),
                PCWSTR::null(),
                None,
                PCWSTR(U16CString::from_vec(dependencies).unwrap().as_ptr()),
                PCWSTR::null(),
                PCWSTR::null(),
            )
        };
        match service_handle {
            Ok(handle) => Ok(WindowsService {
                sc_manager_handle,
                service_handle: handle,
                config: Self::get_config(handle)?,
            }),
            Err(_) => unsafe { Err(GetLastError().into()) },
        }
    }

    fn open_service(
        sc_manager_handle: SC_HANDLE,
        name: &str,
        access: ServiceAccess,
    ) -> Result<SC_HANDLE, ServiceError> {
        let service_handle = unsafe {
            OpenServiceW(
                sc_manager_handle,
                PCWSTR(U16CString::from_str(name).unwrap().as_ptr()),
                access,
            )
        };
        match service_handle {
            Ok(handle) => Ok(handle),
            Err(_) => unsafe { Err(GetLastError().into()) },
        }
    }

    fn open_sc_manager(access: ScManagerAccess) -> Result<SC_HANDLE, ServiceError> {
        let sc_manager_handle = unsafe { OpenSCManagerW(PCWSTR::null(), PCWSTR::null(), access) };
        match sc_manager_handle {
            Ok(handle) => Ok(handle),
            Err(_) => unsafe { Err(GetLastError().into()) },
        }
    }

    fn get_config(service_handle: SC_HANDLE) -> Result<ServiceConfig, ServiceError> {
        let mut config = ServiceConfig::default();
        let mut cap: u32 = Default::default();
        match unsafe { QueryServiceConfigW(service_handle, Some(&mut config), 370, &mut cap) } {
            Ok(_) => Ok(config),
            Err(_) => {
                println!("{},{}", size_of::<ServiceConfig>() as u32, cap);
                unsafe { Err(GetLastError().into()) }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::WindowsService;

    #[test]
    fn open_service() {
        let service = WindowsService::open("WSearch");
        match service {
            Ok(s) => {
                println!("{:?}", s.config)
            }
            Err(e) => {
                println!("{}", e)
            }
        }
    }
}
