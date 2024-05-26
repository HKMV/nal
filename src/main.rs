use std::ffi::OsString;
use std::{env, fs};
use std::fs::{File, OpenOptions};
use std::str::FromStr;
use tokio::time::{Duration, sleep};
use log::{debug, error, info, LevelFilter, warn};
use serde_json::Value::Bool;
use crate::core::nal;
use crate::core::nal::{login, LoginConfig, NalConfig, NetStatusCheck, NetType};
use crate::core::sangfor::Sangfor;
use std::io::{Error, Write};
use std::path::PathBuf;
use std::time::SystemTime;
use clap::{Command, Parser};
use serde::{Deserialize, Serialize};
use service_manager::{ServiceInstallCtx, ServiceLabel, ServiceManager, ServiceStartCtx, ServiceStopCtx, ServiceUninstallCtx};
use util::service::Service;
use crate::util::cmd::Cli;

mod core;
mod util;
mod test;

#[tokio::main]
async fn main() {
    let config = nal::init_config();
    debug!("config: {config:#?}");
    let level_filter = LevelFilter::from_str(config.log.level.as_str()).unwrap_or(LevelFilter::Info);
    util::logs::init("nal.log", level_filter).expect("初始化日志出错");

    let cli = util::cmd::Cli::parse();
    let service = Service::new("net-auto-login");
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
    if cli.run {
        handler(config).await;
        return;
    }
    if cfg!(debug_assertions) {
        handler(config).await
    }
}

async fn handler(config: NalConfig) {
    loop {
        //检测网络是否正常
        let is_ok = nal::check_net().await;
        if !is_ok {
            warn!("网络异常");
            //登录
            let sangfor = Sangfor::new("http://1.1.1.4");
            let login_ok = nal::login(&sangfor, &config.login).await;
            if login_ok.unwrap_or_else(|e| { false }) {
                info!("登录成功");
            } else {
                error!("登录失败");
            }
        } else if config.log.normal {
            info!("网络正常");
        };

        // 延迟指定秒后再次执行
        sleep(Duration::from_secs(config.check.interval as u64)).await;
    }
}