use std::{env, fs};
use std::io::{Error};
use std::path::Path;
use chrono::Local;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::{Config};
use log4rs::config::{Appender, Logger, Root};
use log4rs::encode::pattern::PatternEncoder;
use log::{error, LevelFilter};
use log4rs::append::rolling_file::policy::compound::CompoundPolicy;
use log4rs::append::rolling_file::policy::compound::roll::fixed_window::FixedWindowRoller;
use log4rs::append::rolling_file::policy::compound::trigger::size::SizeTrigger;
use log4rs::append::rolling_file::policy::compound::trigger::time::{TimeTrigger, TimeTriggerConfig, TimeTriggerInterval};

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
    let log_file_path = logs_dir.clone().to_string() + log_file;

    init_log4rs(log_file_path, level);
    Ok(())
}

fn init_env_logger() {
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
}

fn init_fern() {
    /* //fern
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
        .apply()?;*/
}


/// 这是创建新文件的大小。32MB
const TRIGGER_FILE_SIZE: u64 = 1024 * 1024 * 32;
/// 日志存档将移动到的位置 有关模式信息，请参阅：
///     https://docs.rs/log4rs/*/log4rs/append/rolling_file/policy/compound/roll/fixed_window/struct.FixedWindowRollerBuilder.html#method.build
const ARCHIVE_PATTERN: &str = "./logs/nal.{}.log";
/// 要保留的存档日志文件数
const LOG_FILE_COUNT: u32 = 3;

/// 初始化log4rs
fn init_log4rs(log_file_path: String, level: LevelFilter) {
    // log4rs
    // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
    let pattern = Box::new(PatternEncoder::new("[{d(%Y-%m-%d %H:%M:%S)} {h({l}):<5.5} {M}] {f}:{L} {m}{n}"));

    // 创建用于文件日志记录的策略
    let trigger = SizeTrigger::new(TRIGGER_FILE_SIZE);
    // let time_trigger = TimeTrigger::new(TimeTriggerConfig::default());
    let roller = FixedWindowRoller::builder()
        .build(ARCHIVE_PATTERN, LOG_FILE_COUNT)
        .unwrap();
    let policy = CompoundPolicy::new(Box::new(trigger), Box::new(roller));

    // 日志记录到日志文件。（带滚动）
    let logfile = log4rs::append::rolling_file::RollingFileAppender::builder()
        .encoder(pattern)
        .build(log_file_path, Box::new(policy))
        .unwrap();
    let config = if cfg!(debug_assertions) {
        Config::builder()
            // .appender(
            //     Appender::builder()
            //         .filter(Box::new(ThresholdFilter::new(level)))
            //         .build("stderr", Box::new(stderr)),
            // )
            .appender(Appender::builder()
                .build("stdout", Box::new(ConsoleAppender::builder()
                    .encoder(pattern.clone())
                    .build())))
            .appender(Appender::builder().build("file", Box::new(logfile)))
            // .logger(Logger::builder()
            //     .appender("file")
            //     .additive(false)
            //     .build("*",level))
            .build(Root::builder().appender("stdout").appender("file").build(level))
            .unwrap()
    } else {
        Config::builder()
            .appender(Appender::builder().build("file", Box::new(logfile)))
            .build(Root::builder().appender("file").build(level))
            .unwrap()
    };
    log4rs::init_config(config).unwrap();
}