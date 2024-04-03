windows_service_controller[![windows_service_controller on crates.io](https://img.shields.io/crates/v/windows_service_controller)](https://crates.io/crates/windows_service_controller)[![windows_service_controller on docs.rs](https://docs.rs/winreg/badge.svg)](https://docs.rs/windows_service_controller)
======

Make it easier to control windows service.

Current features:

* Basic operations:

  * Open an existing service

  * Create a new service

  * Delete a service

  * Query the status of service

  * Edit config of service

## Usage

```toml
# Cargo.toml
[dependencies]
windows_service_controller = "0.1.2"
```

### Open an existing service


```rust
use windows_service_controller::WindowsService;
use windows_service_controller::dword::service_access;

fn open_service() {
        let service = WindowsService::open("WSearch",
            Some(service_access::GENERIC_READ), None);
        match service {
            Ok(s) => {
                println!("{:?}", s.config)
            }
            Err(e) => {
                println!("{}", e)
            }
        }
    }
```

### Create a new service

```rust
use windows_service_controller::WindowsService;
use windows_service_controller::dword::{sc_manager_access,
    service_access,service_type,service_start_type,service_error_control};

fn create_service() {
        let service = WindowsService::new(
            "Lers",
            None,
            Some(sc_manager_access::GENERIC_WRITE),
            Some(service_access::GENERIC_WRITE),
            service_type::SERVICE_WIN32_OWN_PROCESS,
            service_start_type::SERVICE_DEMAND_START,
            service_error_control::SERVICE_ERROR_NORMAL,
            "Path to Binary",
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
```

### Delete a service

```rust
use windows_service_controller::WindowsService;

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
```

### Edit service config

```rust
use windows_service_controller::{WindowsService,PWSTR};

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
```

**BUG: "lpServiceStartName" can't be edit.**

​    
