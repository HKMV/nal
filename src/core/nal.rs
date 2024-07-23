use async_trait::async_trait;
use log::{warn, LevelFilter};
use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};
use serde_yaml::Serializer;
use std::fs::File;
use std::time::Duration;

/// 网络自动登录trait
#[async_trait]
pub trait Nal {
    /// 登录网络
    async fn login_net(&self, config: &LoginConfig) -> Result<bool, Error>;
}

/// 网络类型
#[derive(Debug, Serialize, Deserialize)]
pub enum NetType {
    Sangfor,
    Other,
}

/// 登录配置
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginConfig {
    pub username: String,
    pub password: String,
}

/// 网络状态检测相关参数
#[derive(Debug, Serialize, Deserialize)]
pub struct NetStatusCheck {
    /// 检测间隔时间，单位秒
    pub interval: u16,
}

/// 日志相关配置
#[derive(Debug, Serialize, Deserialize)]
pub struct LogConfig {
    pub level: String,
    pub normal: bool,
}

/// NAL配置参数
#[derive(Debug, Serialize, Deserialize)]
pub struct NalConfig {
    pub net_type: Option<NetType>,
    pub login: LoginConfig,
    pub check: NetStatusCheck,
    pub log: LogConfig,
}

impl NalConfig {
    pub fn default() -> Self {
        NalConfig {
            net_type: Option::from(NetType::Sangfor),
            login: LoginConfig {
                username: "".to_string(),
                password: "".to_string(),
            },
            check: NetStatusCheck {
                // 默认3秒
                interval: 3,
            },
            log: LogConfig {
                //默认日志级别info
                level: LevelFilter::Info.to_string(),
                // 默认不显示正常日志
                normal: false,
            },
        }
    }
}

/// 初始化配置
pub fn init_config() -> NalConfig {
    let conf_file_path = "./config.yml";
    let result = File::open(conf_file_path);
    if result.is_err() {
        //初始化配置
        let config = NalConfig::default();
        let file_write = File::create(conf_file_path).unwrap();
        config
            .serialize(&mut Serializer::new(&file_write))
            .expect("序列化输出失败!");
        return config;
    }

    //缺少字段会导致序列化出错
    let result1 = serde_yaml::from_reader(result.unwrap());
    if result1.is_err() {
        let string = result1.err().unwrap().to_string();
        warn!("config serde_yaml error: {string}");
        NalConfig::default()
    } else {
        let mut yaml: NalConfig = result1.unwrap_or(NalConfig::default());
        if yaml.net_type.is_none() {
            yaml.net_type = Option::from(NetType::Sangfor)
        }
        yaml
    }
}

/// 获取没有代理的客户端
pub fn get_no_proxy_client() -> Client {
    Client::builder()
        .no_proxy() // 禁用代理
        .connect_timeout(Duration::from_secs(10))
        .timeout(Duration::from_secs(30))
        .build()
        .unwrap()
}

/// 检测网络是否正常
pub async fn check_net() -> bool {
    let mut fail_count = 0;
    for _ in 0..3 {
        if !get_no_proxy_client()
            .get("https://baidu.com")
            .send()
            .await
            .is_ok()
        {
            fail_count += 1;
        }
    }
    //失败次数小于3认为网络是正常的
    fail_count < 3
}

pub async fn login<T: Nal>(nal: &T, lc: &LoginConfig) -> Result<bool, Error> {
    nal.login_net(lc).await
}
