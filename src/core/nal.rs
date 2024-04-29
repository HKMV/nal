use std::collections::HashMap;
use std::fs::File;
use std::time::Duration;
use async_trait::async_trait;
use log::warn;
use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};
use serde::__private::de::Content::I16;
use serde_json::Value;
use tokio::time::timeout;

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

impl LoginConfig {
    pub fn new(username: &str, password: &str) -> Self {
        Self {
            username: String::from(username),
            password: String::from(password),
        }
    }
}

/// 网络状态检测相关参数
#[derive(Debug, Serialize, Deserialize)]
pub struct NetStatusCheck {
    /// 检测间隔时间，单位秒
    pub interval: u16,
}

/// NAL配置参数
#[derive(Debug, Serialize, Deserialize)]
pub struct NalConfig {
    pub net_type: Option<NetType>,
    pub login: LoginConfig,
    pub check: NetStatusCheck,
}

impl NalConfig {
    pub fn default() -> Self {
        NalConfig {
            net_type: Option::from(NetType::Sangfor),
            login: LoginConfig { username: "".to_string(), password: "".to_string() },
            check: NetStatusCheck { interval: 0 },
        }
    }
}

/// 初始化配置
pub fn init_config() -> NalConfig {
    let result = File::open("./config.yml");
    if result.is_err() {
        //初始化配置
        return NalConfig::default();
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
        .timeout(Duration::from_secs(30))
        .build().unwrap()
}

/// 检测网络是否正常
pub async fn check_net() -> bool {
    get_no_proxy_client()
        .get("http://baidu.com")
        .send()
        .await
        .is_ok()
}

pub async fn login<T: Nal>(nal: &T, lc: &LoginConfig) -> Result<bool, Error> {
    nal.login_net(lc).await
}