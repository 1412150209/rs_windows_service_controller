use std::ptr::null_mut;

use crate::dword::ServiceStatus;
use widestring::U16CString;
use winapi::um::winsvc::{SC_HANDLE, SERVICE_QUERY_STATUS, SERVICE_START, SERVICE_STOP};
use winapi::{
    shared::minwindef::DWORD,
    um::{
        errhandlingapi::GetLastError,
        winsvc::{
            ControlService, DeleteService, OpenSCManagerW, OpenServiceW, QueryServiceStatus,
            StartServiceW, SC_MANAGER_CONNECT, SERVICE_CONTROL_STOP,
        },
    },
};

pub mod dword {
    use std::collections::HashMap;
    use std::fmt::{Display, Formatter};
    use winapi::shared::minwindef::DWORD;

    pub struct ServiceStatus {
        pub kind: DWORD,
    }

    pub const STATUS: HashMap<DWORD, &str> = HashMap::from([
        (DWORD::from(1u8), "SERVICE_STOPPED"),
        (DWORD::from(2u8), "SERVICE_START_PENDING"),
        (DWORD::from(3u8), "SERVICE_STOP_PENDING"),
        (DWORD::from(4u8), "SERVICE_RUNNING"),
        (DWORD::from(5u8), "SERVICE_CONTINUE_PENDING"),
        (DWORD::from(6u8), "SERVICE_PAUSE_PENDING"),
        (DWORD::from(7u8), "SERVICE_PAUSED"),
    ]);

    impl Display for ServiceStatus {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            if STATUS.contains_key(&self.kind) {
                write!(
                    f,
                    "Service Status({}):{}",
                    &self.kind,
                    STATUS.get(&self.kind).unwrap()
                )
            } else {
                write!(f, "UNKNOWN STATUS:{:?}", &self.kind)
            }
        }
    }
}
#[cfg(windows)]
pub struct WindowsService {
    service_handle: SC_HANDLE,
    service_name: &'static str,
}

#[cfg(windows)]
impl WindowsService {
    pub fn new(name: &'static str) -> Result<WindowsService, DWORD> {
        let sc_manager_handle = Self::open_sc_manager(SC_MANAGER_CONNECT)?;
        let service_handle = Self::open_service(
            sc_manager_handle,
            name,
            SERVICE_QUERY_STATUS | SERVICE_START | SERVICE_STOP,
        )?;
        Ok(WindowsService {
            service_handle,
            service_name: name,
        })
    }

    fn open_service(
        sc_manager_handle: SC_HANDLE,
        service_name: &str,
        desired_access: DWORD,
    ) -> Result<SC_HANDLE, DWORD> {
        let service_name_wstr = U16CString::from_str(service_name).unwrap();
        let service_handle = unsafe {
            OpenServiceW(
                sc_manager_handle,
                service_name_wstr.as_ptr(),
                desired_access,
            )
        };
        if service_handle.is_null() {
            Err(unsafe { GetLastError() })
        } else {
            Ok(service_handle)
        }
    }

    fn open_sc_manager(desired_access: DWORD) -> Result<SC_HANDLE, DWORD> {
        let sc_manager_handle = unsafe { OpenSCManagerW(null_mut(), null_mut(), desired_access) };
        if sc_manager_handle.is_null() {
            Err(unsafe { GetLastError() })
        } else {
            Ok(sc_manager_handle)
        }
    }

    /// 请求服务状态
    pub fn query_service_status(&self) -> Result<ServiceStatus, DWORD> {
        let mut service_status = unsafe { std::mem::zeroed() };
        let result = unsafe { QueryServiceStatus(self.service_handle, &mut service_status) };
        if result == 0 {
            Err(unsafe { GetLastError() })
        } else {
            Ok(ServiceStatus {
                kind: service_status.dwCurrentState,
            })
        }
    }

    /// 启动服务
    pub fn start_service(&self) -> Result<(), DWORD> {
        let result = unsafe { StartServiceW(self.service_handle, 0, null_mut()) };
        if result == 0 {
            let dword = unsafe { GetLastError() };
            if dword.eq(&1056) {
                println!("WARNING: Service is already started.");
                return Ok(());
            }
            Err(dword)
        } else {
            Ok(())
        }
    }

    /// 关闭f服务
    pub fn stop_service(&self) -> Result<(), DWORD> {
        let mut service_status = unsafe { std::mem::zeroed() };
        let result = unsafe {
            ControlService(
                self.service_handle,
                SERVICE_CONTROL_STOP,
                &mut service_status,
            )
        };
        if result == 0 {
            Err(unsafe { GetLastError() })
        } else {
            Ok(())
        }
    }

    /// 删除服务
    fn delete_service(&self) -> Result<(), DWORD> {
        let result = unsafe { DeleteService(self.service_handle) };
        if result == 0 {
            Err(unsafe { GetLastError() })
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::WindowsService;

    #[test]
    fn it_works() {
        let service = match WindowsService::new("gupdatem") {
            Ok(s) => s,
            Err(e) => {
                eprintln!("{}", e);
                panic!()
            }
        };
        println!("{}", service.query_service_status().unwrap());
    }
}
