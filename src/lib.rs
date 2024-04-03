use windows::core::PCWSTR;
use windows::Win32::Foundation::GetLastError;
use windows::Win32::Security::SC_HANDLE;
use windows::Win32::System::Services::{
    ChangeServiceConfigW, CloseServiceHandle, CreateServiceW, DeleteService, OpenSCManagerW,
    OpenServiceW, QueryServiceConfigW, QueryServiceStatus, QUERY_SERVICE_CONFIGW, SERVICE_STATUS,
};

use crate::dword::{
    ScManagerAccess, ServiceAccess, ServiceError,
    ServiceErrorControl, ServiceStartType, ServiceStatus, ServiceType,
};
use windows_macro::PCWSTR;

pub mod dword;

/// windows服务类
pub struct WindowsService {
    sc_manager_handle: SC_HANDLE,
    service_handle: SC_HANDLE,
    pub config: ServiceConfig,
}

type ServiceConfig = QUERY_SERVICE_CONFIGW;

impl Drop for WindowsService {
    fn drop(&mut self) {
        unsafe {
            CloseServiceHandle(self.service_handle).expect("关闭服务对象句柄失败");
            CloseServiceHandle(self.sc_manager_handle).expect("关闭服务管理器句柄失败");
        }
    }
}

impl WindowsService {
    /// # 通过服务名打开一个服务实例
    /// ## 参数
    /// ### input:
    /// - name: 服务名称(不是显示名称)
    /// - service_access: 默认为SERVICE_ALL_ACCESS
    /// - sc_manager_access: 默认为SC_MANAGER_CONNECT
    /// ### output:
    /// - Result<WindowsService,ServiceError>
    /// ## 例子
    /// ```
    /// use windows_service_controller::dword::ServiceAccess;
    /// use windows_service_controller::WindowsService;
    /// let service = WindowsService::open("Lers", Some(ServiceAccess::GENERIC_READ),None);
    /// ```
    pub fn open(
        name: &str,
        service_access: Option<ServiceAccess>,
        sc_manager_access: Option<ScManagerAccess>,
    ) -> Result<WindowsService, ServiceError> {
        let sc_manager_handle = Self::open_sc_manager(
            sc_manager_access.unwrap_or_else(|| ScManagerAccess::SC_MANAGER_CONNECT),
        )?;
        let service_handle = Self::open_service(
            sc_manager_handle,
            name,
            service_access.unwrap_or_else(|| ServiceAccess::SERVICE_ALL_ACCESS),
        )?;
        Ok(WindowsService {
            sc_manager_handle,
            service_handle,
            config: Self::get_config(service_handle)?,
        })
    }

    /// # 请求当前服务状态
    pub fn query_service_status(&self) -> Result<ServiceStatus, ServiceError> {
        let mut status = SERVICE_STATUS::default();
        let result = unsafe { QueryServiceStatus(self.service_handle, &mut status) };
        if result.is_ok() {
            Ok(status.dwCurrentState.into())
        } else {
            unsafe { Err(GetLastError().into()) }
        }
    }

    /// # 新建一个服务
    /// ## 参数
    /// ### input:
    /// - name: 服务名称(最长256字符,斜杠无效)
    /// - display_name: 服务显示名称,不写与name一致
    /// - sc_manager_access: SCM的访问权限,默认SC_MANAGER_ALL_ACCESS
    /// - service_access: 对服务的访问权限,默认SERVICE_ALL_ACCESS
    /// - service_type: 服务类型,常量在 service_type::
    /// - service_start_type: 服务启动选项
    /// - error_control: 错误控制
    /// - binary_path: 需要启动的文件路径,路径可以包含启动的参数
    /// - dependencies: 服务的依赖项
    /// ### output:
    /// - Result<WindowsService,ServiceError>
    /// ## 例子
    /// ```
    /// use windows_service_controller::dword::{ServiceError, ServiceStartType, ServiceType};
    /// use windows_service_controller::WindowsService;
    /// let service = WindowsService::new(
    ///     "Lers",
    ///     None,
    ///     None,
    ///     None,
    ///     ServiceType::SERVICE_WIN32_OWN_PROCESS,
    ///     ServiceStartType::SERVICE_DEMAND_START,
    ///     ServiceError::SERVICE_ERROR_NORMAL,
    ///     "D:\\ENGLISH\\Rust\\hot_update\\target\\debug\\hot_update.exe",
    ///     None,
    ///  );
    ///

    pub fn new(
        name: &str,
        display_name: Option<&str>,
        sc_manager_access: Option<ScManagerAccess>,
        service_access: Option<ServiceAccess>,
        service_type: ServiceType,
        service_start_type: ServiceStartType,
        error_control: ServiceErrorControl,
        binary_path: &str,
        dependencies: Option<Vec<&str>>,
    ) -> Result<WindowsService, ServiceError> {
        let sc_manager_handle = Self::open_sc_manager(
            sc_manager_access.unwrap_or_else(|| ScManagerAccess::SC_MANAGER_ALL_ACCESS),
        )?;
        let display_name = display_name.unwrap_or_else(|| name);
        let service_handle = unsafe {
            CreateServiceW(
                sc_manager_handle,
                PCWSTR!(name),
                PCWSTR!(display_name),
                service_access.unwrap_or_else(|| ServiceAccess::SERVICE_ALL_ACCESS).into(),
                service_type.into(),
                service_start_type.into(),
                error_control.into(),
                PCWSTR!(binary_path),
                PCWSTR::null(),
                None,
                match dependencies {
                    None => PCWSTR::null(),
                    Some(v) => {
                        let mut result: Vec<u16> = Vec::new();
                        for str in v {
                            result.push(str.parse::<u16>().unwrap())
                        }
                        PCWSTR!(vec result)
                    }
                },
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

    /// # 删除该服务
    /// ## 参数
    /// ### output:
    /// - Result<(),ServiceError>
    pub fn delete_service(&self) -> Result<(), ServiceError> {
        let result = unsafe { DeleteService(self.service_handle) };
        if result.is_ok() {
            Ok(())
        } else {
            unsafe { Err(GetLastError().into()) }
        }
    }

    /// # 更新服务配置
    /// ## 参数
    /// ### input:
    /// - passwd: 修改服务密码,不修改请传入None
    /// ### output:
    /// - Result<(),ServiceError>
    /// ## 例子
    /// ```
    /// use windows_service_controller::WindowsService;
    /// let mut service = WindowsService::open("Lers", None, None).unwrap();
    /// use windows_macro::PWSTR;
    ///
    /// service.config.lpDisplayName = PWSTR!("lers233");
    /// service.update_service_config(None).unwrap()
    ///```
    /// ## BUG
    /// 似乎无法修改lpServiceStartName字段
    pub fn update_service_config(&self, passwd: Option<&str>) -> Result<(), ServiceError> {
        match unsafe {
            ChangeServiceConfigW(
                self.service_handle,
                self.config.dwServiceType,
                self.config.dwStartType,
                self.config.dwErrorControl,
                PCWSTR(self.config.lpBinaryPathName.as_ptr()),
                PCWSTR(self.config.lpLoadOrderGroup.as_ptr()),
                None,
                PCWSTR(self.config.lpDependencies.as_ptr()),
                PCWSTR::null(),
                match passwd {
                    None => PCWSTR::null(),
                    Some(s) => PCWSTR!(s),
                },
                PCWSTR(self.config.lpDisplayName.as_ptr()),
            )
        } {
            Ok(_) => Ok(()),
            Err(_) => unsafe { Err(GetLastError().into()) },
        }
    }

    fn open_service(
        sc_manager_handle: SC_HANDLE,
        name: &str,
        access: ServiceAccess,
    ) -> Result<SC_HANDLE, ServiceError> {
        let service_handle = unsafe { OpenServiceW(sc_manager_handle, PCWSTR!(name), access.into()) };
        match service_handle {
            Ok(handle) => Ok(handle),
            Err(_) => unsafe { Err(GetLastError().into()) },
        }
    }

    fn open_sc_manager(access: ScManagerAccess) -> Result<SC_HANDLE, ServiceError> {
        let sc_manager_handle = unsafe { OpenSCManagerW(PCWSTR::null(), PCWSTR::null(), access.into()) };
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
                match unsafe {
                    QueryServiceConfigW(service_handle, Some(&mut config), cap, &mut cap)
                } {
                    Ok(_) => Ok(config),
                    Err(_) => unsafe { Err(GetLastError().into()) },
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use windows_macro::PWSTR;
    use crate::dword::{ScManagerAccess, ServiceAccess, ServiceErrorControl, ServiceStartType, ServiceType};
    use crate::WindowsService;

    #[test]
    fn open_service() {
        let service = WindowsService::open("WSearch", Some(ServiceAccess::GENERIC_READ), None);
        match service {
            Ok(s) => {
                println!("{:?}", s.config)
            }
            Err(e) => {
                println!("{}", e)
            }
        }
    }

    #[test]
    fn create_service() {
        let service = WindowsService::new(
            "Lers",
            None,
            Some(ScManagerAccess::GENERIC_WRITE),
            Some(ServiceAccess::GENERIC_WRITE),
            ServiceType::SERVICE_WIN32_OWN_PROCESS,
            ServiceStartType::SERVICE_DEMAND_START,
            ServiceErrorControl::SERVICE_ERROR_NORMAL,
            "D:\\example\\some.exe",
            None,
        );
        match service {
            Ok(s) => {
                println!("{:?}", s.config)
            }
            Err(e) => {
                println!("{}", e)
            }
        }
    }

    #[test]
    fn delete_service() {
        let service = WindowsService::open("Lers", None, None);
        match service {
            Ok(s) => match s.delete_service() {
                Ok(_) => {
                    println!("succeed")
                }
                Err(e) => {
                    println!("{}", e);
                }
            },
            Err(e) => {
                println!("{}", e);
            }
        }
    }

    #[test]
    fn update_service_config() {
        let service = WindowsService::open("Lers", None, None);
        match service {
            Ok(mut s) => {
                s.config.lpDisplayName = PWSTR!("lers test");
                match s.update_service_config(None) {
                    Ok(_) => {
                        println!("succeed")
                    }
                    Err(e) => {
                        println!("{}", e);
                    }
                }
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    }
}
