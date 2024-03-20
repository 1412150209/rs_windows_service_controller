use std::ptr::null_mut;

use widestring::U16CString;
use winapi::{
    shared::minwindef::DWORD,
    um::{
        errhandlingapi::GetLastError,
        winsvc::{
            ControlService, OpenSCManagerW, OpenServiceW, QueryServiceStatus,
            SC_MANAGER_CONNECT, SERVICE_CONTROL_STOP, StartServiceW,
        },
    },
};
use winapi::um::winsvc::{SC_HANDLE, SERVICE_QUERY_STATUS, SERVICE_START, SERVICE_STOP};

use crate::dword::ServiceStatus;

pub mod dword {
    use std::collections::HashMap;
    use std::fmt::{Display, Formatter};

    use lazy_static::lazy_static;
    use winapi::shared::minwindef::DWORD;

    pub struct ServiceStatus {
        pub kind: DWORD,
    }

    lazy_static! {
        static ref SERVICE_STATUS: HashMap<u8, STATUS> = {
            let map: HashMap<u8, STATUS> = HashMap::from([
                (1u8, STATUS::SERVICE_STOPPED),
                (2u8, STATUS::SERVICE_START_PENDING),
                (3u8, STATUS::SERVICE_STOP_PENDING),
                (4u8, STATUS::SERVICE_RUNNING),
                (5u8, STATUS::SERVICE_CONTINUE_PENDING),
                (6u8, STATUS::SERVICE_PAUSE_PENDING),
                (7u8, STATUS::SERVICE_PAUSED),
            ]);
            map
        };
    }

    #[derive(Debug)]
    #[allow(non_camel_case_types)]
    #[derive(PartialEq)]
    pub enum STATUS {
        SERVICE_STOPPED,
        SERVICE_START_PENDING,
        SERVICE_STOP_PENDING,
        SERVICE_RUNNING,
        SERVICE_CONTINUE_PENDING,
        SERVICE_PAUSE_PENDING,
        SERVICE_PAUSED,
    }

    impl Display for ServiceStatus {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let num = self.kind as u8;
            if SERVICE_STATUS.contains_key(&num) {
                write!(
                    f,
                    "Service Status({}):{:?}",
                    &self.kind,
                    SERVICE_STATUS.get(&num).unwrap()
                )
            } else {
                write!(f, "UNKNOWN STATUS:{:?}", &self.kind)
            }
        }
    }

    impl ServiceStatus {
        pub fn eq(&self, other: &STATUS) -> bool {
            let value = SERVICE_STATUS.get(&(self.kind as u8)).unwrap();
            return value == other;
        }
    }
}
#[cfg(windows)]
pub struct WindowsService {
    service_handle: SC_HANDLE,
    pub service_name: &'static str,
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

    // /// 删除服务
    // fn delete_service(&self) -> Result<(), DWORD> {
    //     let result = unsafe { DeleteService(self.service_handle) };
    //     if result == 0 {
    //         Err(unsafe { GetLastError() })
    //     } else {
    //         Ok(())
    //     }
    // }
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
