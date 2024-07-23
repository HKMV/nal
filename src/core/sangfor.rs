use crate::core::nal::{get_no_proxy_client, LoginConfig, Nal};
use crate::util::rc4::RC4;
use async_trait::async_trait;
use log::debug;
use reqwest::Error;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// 深信服
pub struct Sangfor {
    login_url: String,
}

/// 登录接口
const LOGIN_API: &str = "/ac_portal/login.php";

impl Sangfor {
    pub fn new(addr: &str) -> Sangfor {
        let mut string = addr.to_owned();
        string.push_str(LOGIN_API);
        Self {
            login_url: String::from(string),
        }
    }

    /// rc4编码
    fn rc4_encode(key: &str, pwd: &str) -> String {
        let mut data = pwd.as_bytes().to_vec();

        // let mut rc4 = Rc4::new(key.as_bytes().into());
        // rc4.apply_keystream(&mut data);

        let mut rc4 = RC4::new(key.as_bytes());
        rc4.apply_keystream(&mut data);

        data.iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>()
    }
}

#[async_trait]
impl Nal for Sangfor {
    async fn login_net(&self, config: &LoginConfig) -> Result<bool, Error> {
        let client = get_no_proxy_client();

        let mut params = HashMap::new();
        params.insert("opr", "pwdLogin");
        params.insert("userName", config.username.as_str());
        let timestamp = (SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64)
            .to_string();
        let auth_tag = timestamp.as_str();
        params.insert("auth_tag", auth_tag);
        let pwd = Self::rc4_encode(auth_tag, config.password.as_str());
        params.insert("pwd", pwd.as_str());
        params.insert("rememberPwd", "1");
        let lu = &self.login_url;
        let rsp = client
            .post(lu)
            .form(&params)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;
        debug!("login result: {rsp:#?}");
        // rsp.is_ok()
        Ok(rsp["success"] == true)
    }
}
