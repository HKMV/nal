use std::process::exit;
use crate::core::nal;
use crate::core::nal::NalConfig;
use crate::core::sangfor::Sangfor;
use crate::util::service::Service;
use clap::Parser;
use log::{debug, error, info, warn, LevelFilter};
use std::str::FromStr;
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use util::{cmd, logs};
use crate::util::cmd::Cli;

mod core;
mod test;
mod util;

fn main() {
    let pkg_name = env!("CARGO_PKG_NAME");

    let config = nal::init_config();
    debug!("config: {config:#?}");
    let level_filter =
        LevelFilter::from_str(config.log.level.as_str()).unwrap_or(LevelFilter::Info);
    logs::init(pkg_name.to_owned(), level_filter).expect("初始化日志出错");

    let cli = cmd::Cli::parse();

    if !is_service_cmd(cli.clone()) {
        if cli.run {
            run_service()
        } else {
            //默认直接运行
            handler(config)
        };
        return;
    }

    let service_res = Service::new(pkg_name);
    let service = match service_res {
        Ok(ser) => { ser }
        Err(err) => {
            println!("创建服务出错：{}", err.to_string());
            if !cfg!(debug_assertions) {
                error!("创建服务出错：{}",err.to_string());
            }
            return;
        }
    };

    // service.install();
    // return;

    if cli.install {
        service.install();
        service.start();
        return;
    }
    if cli.uninstall {
        service.stop();
        service.uninstall();
        return;
    }
    if cli.start {
        service.start();
        return;
    }
    if cli.stop {
        service.stop();
        return;
    }
}

fn run_service() {
    if cfg!(windows) {
        windows_service_main();
    } else {
        let config = nal::init_config();
        handler(config)
    }
}

#[cfg(windows)]
fn windows_service_main() {
    use std::ffi::OsString;
    use windows_service::service_dispatcher;
    use windows_service::service::{ServiceControl, ServiceControlAccept, ServiceExitCode, ServiceState, ServiceStatus, ServiceType};
    use windows_service::service_control_handler;
    use windows_service::service_control_handler::ServiceControlHandlerResult;

    fn run_service(_arguments: Vec<OsString>) -> windows_service::Result<()> {
        // Register system service event handler
        let status_handle = service_control_handler::
        register(env!("CARGO_PKG_NAME"), move |control_event| -> ServiceControlHandlerResult {
            match control_event {
                ServiceControl::Stop => {
                    thread::spawn(|| {
                        sleep(Duration::from_millis(10));
                        info!("停止服务");
                        exit(0)
                    });
                    ServiceControlHandlerResult::NoError
                }
                ServiceControl::Interrogate => {
                    ServiceControlHandlerResult::NoError
                }
                _ => ServiceControlHandlerResult::NotImplemented,
            }
        })?;

        let next_status = ServiceStatus {
            // Should match the one from system service registry
            service_type: ServiceType::OWN_PROCESS,
            // The new state
            current_state: ServiceState::Running,
            // Accept stop events when running
            controls_accepted: ServiceControlAccept::STOP,
            // Used to report an error when starting or stopping only, otherwise must be zero
            exit_code: ServiceExitCode::Win32(0),
            // Only used for pending states, otherwise must be zero
            checkpoint: 0,
            // Only used for pending states, otherwise must be zero
            wait_hint: Duration::default(),
            process_id: None,
        };

        // Tell the system that the service is running now
        status_handle.set_service_status(next_status)?;

        // Do some work
        let config = nal::init_config();
        handler(config);
        Ok(())
    }

    extern "system" fn ffi_service_main(num_service_arguments: u32, service_arguments: *mut *mut u16) {
        let arguments = unsafe {
            service_dispatcher::parse_service_arguments(
                num_service_arguments,
                service_arguments,
            )
        };
        if let Err(_e) = run_service(arguments) {
            error!("服务运行出错：{}", _e.to_string())
        }
    }
    // Register generated `ffi_service_main` with the system and start the service, blocking
    // this thread until the service is stopped.
    let pkg_name = env!("CARGO_PKG_NAME");
    let result = service_dispatcher::start(pkg_name, ffi_service_main);
    if let Err(_e) = result {
        error!("服务运行出错：{}", _e.to_string())
    }
}

fn handler(config: NalConfig) {
    println!("开始检测。。。");
    if !cfg!(debug_assertions) {
        info!("开始检测。。。");
    }
    loop {
        //检测网络是否正常
        let is_ok = nal::check_net();
        if !is_ok {
            warn!("网络异常");
            //登录
            let sangfor = Sangfor::new("http://1.1.1.4");
            let login_ok = nal::login(&sangfor, &config.login);
            if login_ok.unwrap_or_else(|_| false) {
                info!("登录成功");
            } else {
                error!("登录失败");
            }
        } else if config.log.normal {
            info!("网络正常");
        };

        // 延迟指定秒后再次执行
        sleep(Duration::from_secs(config.check.interval as u64));
    }
}

/// 校验是否为服务指令
fn is_service_cmd(cli: Cli) -> bool {
    cli.install || cli.uninstall || cli.start || cli.stop
}