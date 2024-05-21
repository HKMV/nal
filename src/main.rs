use std::fs;
use std::fs::{File, OpenOptions};
use std::str::FromStr;
use tokio::time::{Duration, sleep};
use chrono::{Datelike, DateTime, Local, Timelike, Utc};
use cron::Schedule;
use log::{debug, info, warn};
use serde_json::Value::Bool;
use crate::core::nal;
use crate::core::nal::{login, LoginConfig, NetStatusCheck, NetType};
use crate::core::sangfor::Sangfor;
use std::io::{Error, Write};
use std::time::SystemTime;
use serde::{Deserialize, Serialize};

mod core;
mod test;
mod util;

#[tokio::main]
async fn main() {
    util::logs::init("nal.log", log::LevelFilter::Debug).expect("初始化日志出错");

    let config = nal::init_config();
    info!("config: {config:#?}");

    /*let expression = "* 1 * * * * *";
    let schedule = Schedule::from_str(expression).unwrap();
    println!("All stars: Upcoming fire times for '{}':", expression);
    for datetime in schedule.upcoming(Utc).take(10) {
        let is_ok = nal::check_net().await;
        println!("net isOk: -> {is_ok:}");
    }*/

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
                info!("登录失败");
            }
        } else {
            info!("网络正常");
        };

        // 延迟指定秒后再次执行
        sleep(Duration::from_secs(config.check.interval as u64)).await;
    }
}