use std::env;
use std::ffi::OsString;
use std::path::PathBuf;
use log::{debug, info};
use service_manager::{ServiceInstallCtx, ServiceLabel, ServiceManager, ServiceStartCtx, ServiceStopCtx, ServiceUninstallCtx};

/// 系统服务
pub struct Service {
    name: ServiceLabel,
    path: PathBuf,
    service_manage: Box<dyn ServiceManager>,
}

impl Service {
    /// 创建服务对象
    ///
    /// # 参数列表
    ///
    /// * `name`: 服务名
    ///
    /// returns: Service
    ///
    /// # 示例
    ///
    /// ```
    /// let service = Service::new("nal");
    /// ```
    pub fn new(name: &str) -> Self {
        Self {
            name: name.parse().unwrap(),
            path: env::current_exe().unwrap(),
            // 通过检测平台上可用的内容来获得通用服务
            service_manage: <dyn ServiceManager>::native()
                .expect("Failed to detect management platform"),
        }
    }

    /// 安装到系统服务
    pub fn install(&self) {
        // 使用底层服务管理平台安装我们的服务
        self.service_manage.install(ServiceInstallCtx {
            label: self.name.clone(),
            program: self.path.clone(),
            args: vec![OsString::from("--run")],
            contents: None, // 特定于系统的服务内容的可选字符串。
            username: None, // 可选字符串，供备用用户运行服务。
            working_directory: None, // 服务进程的工作目录的可选字符串。
            environment: None, // 用于提供服务进程的环境变量的可选列表。
        }).expect("Failed to install");
        info!("{:}服务安装完成。",self.name.clone().to_string())
    }

    /// 从系统服务卸载
    pub fn uninstall(&self) {
        // 使用底层服务管理平台卸载我们的服务
        self.service_manage.uninstall(ServiceUninstallCtx {
            label: self.name.clone()
        }).expect("Failed to uninstall");
        info!("{:}服务卸载完成。",self.name.clone().to_string())
    }

    /// 启动这个服务
    pub fn start(&self) {
        // 使用底层服务管理平台启动我们的服务
        self.service_manage.start(ServiceStartCtx {
            label: self.name.clone()
        }).expect("Failed to start");
        info!("{:}服务启动完成。",self.name.clone().to_string())
    }

    /// 停止这个服务
    pub fn stop(&self) {
        // 使用底层服务管理平台停止我们的服务
        self.service_manage.stop(ServiceStopCtx {
            label: self.name.clone()
        }).expect("Failed to stop");
        info!("{:}服务停止完成。",self.name.clone().to_string())
    }
}