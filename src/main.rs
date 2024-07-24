use crate::core::nal;
use crate::core::nal::NalConfig;
use crate::core::sangfor::Sangfor;
use crate::util::service::Service;
use clap::Parser;
use log::{debug, error, info, warn, LevelFilter};
use std::str::FromStr;
use tokio::time::{sleep, Duration};

mod core;
mod test;
mod util;

#[tokio::main]
async fn main() {
    let config = nal::init_config();
    debug!("config: {config:#?}");
    let level_filter =
        LevelFilter::from_str(config.log.level.as_str()).unwrap_or(LevelFilter::Info);
    util::logs::init("nal.log", level_filter).expect("初始化日志出错");

    let cli = util::cmd::Cli::parse();
    let service_res = Service::new("net-auto-login");
    let service = match service_res {
        Ok(ser) => { ser }
        Err(err) => {
            println!("创建服务出错！");
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
    if cli.run {
        handler(config).await;
        return;
    }
    //默认直接运行
    handler(config).await
}

async fn handler(config: NalConfig) {
    println!("开始检测。。。");
    if !cfg!(debug_assertions) {
        info!("开始检测。。。");
    }
    loop {
        //检测网络是否正常
        let is_ok = nal::check_net().await;
        if !is_ok {
            warn!("网络异常");
            //登录
            let sangfor = Sangfor::new("http://1.1.1.4");
            let login_ok = nal::login(&sangfor, &config.login).await;
            if login_ok.unwrap_or_else(|_| false) {
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
