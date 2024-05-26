#[tokio::test]
async fn sangfor() {
    let nal: &dyn crate::core::nal::Nal = &crate::core::sangfor::Sangfor::new("http://1.1.1.4/ac_portal/login.php");
    let config = crate::core::nal::LoginConfig {
        username: 2336.to_string(),
        password: 13141.to_string(),
    };
    let is_ok = nal.login_net(&config).await;
    println!(" is_ok: {:?}", is_ok);
}

#[tokio::test]
async fn check_net() {
    let is_ok = crate::core::nal::check_net().await;
    println!(" is_ok: {:?}", is_ok);
}