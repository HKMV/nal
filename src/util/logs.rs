use std::{env, fs};
use std::io::{Error};
use std::path::Path;
use chrono::Local;
use log::LevelFilter;
use log::{error, LevelFilter};

/// 初始化日志
pub fn init(log_file: &str, level: LevelFilter) -> Result<(), Error> {
    let current_dir = if cfg!(target_os = "windows") {
        // env::current_dir()?.to_str().unwrap_or(".").to_string()
        env::current_exe()?
            .parent().unwrap_or(Path::new("."))
            .to_str().unwrap_or(".").to_string()
    } else { ".".to_string() };
    let logs_dir = current_dir + "/logs/";
    fs::create_dir_all(logs_dir.clone()).unwrap();  // 如果需要，创建日志目录
    let log_file_path = logs_dir.to_string() + log_file;
    /* //env_logger
        /// 定义一个函数用于将DateTime对象转换为指定格式的字符串
        fn format_timestamp(dt: Timestamp) -> String {
            let dt: DateTime<Utc> = DateTime::from_str(&*dt.to_string()).unwrap();
            format!("{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
                    dt.year(), dt.month(), dt.day(),
                    dt.hour(), dt.minute(), dt.second())
        }
        // 设置全局日志级别，比如这里设置为info级别
        std::env::set_var("RUST_LOG", "info");
        // env_logger::init(); // 初始化一个默认的logger，如env_logger
        env_logger::Builder::from_default_env()
            .format(|buf, record| {
                // let warn_style = buf.default_level_style(log::Level::Warn);
                let timestamp = buf.timestamp();
                let time = format_timestamp(timestamp);
                let level = record.level();
                let file = record.file().unwrap();
                let line = record.line().unwrap();
                let module_path = record.module_path().unwrap();
                writeln!(
                    buf,
                    // "My formatted log ({time}): {warn_style}{}{warn_style:#}",
                    "{time} [{level:05}] {file}|{module_path}|{line} : {}",
                    record.args()
                )
            }) // 自定义格式化逻辑
            // .filter(Some("crate"), log::LevelFilter::Info) // 只记录指定crate的日志
            .target(log_file_path)
            .init();*/

    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}:{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.target(),
                record.line().unwrap(),
                message
            ))
        })
        .level(level)
        .chain(std::io::stdout())

        .chain(fern::log_file(log_file_path)?)
        .apply()?;
    Ok(())
}