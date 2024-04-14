use log::log;
use crate::core::nal;
use crate::core::nal::{LoginConfig, Nal};
use crate::core::sangfor::Sangfor;

// #[test]
#[tokio::test]
async fn sangfor() {
    let nal: &dyn Nal = &Sangfor::new("http://1.1.1.4/ac_portal/login.php");
    let config = LoginConfig {
        username: 2336.to_string(),
        password: 13141.to_string(),
    };
    let is_ok = nal.login_net(&config).await;
    println!(" is_ok: {:?}", is_ok);
}

#[tokio::test]
async fn check_net() {
    let is_ok = nal::check_net().await;
    println!(" is_ok: {:?}", is_ok);
}