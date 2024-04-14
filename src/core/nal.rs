use std::collections::HashMap;
use std::time::Duration;
use async_trait::async_trait;
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