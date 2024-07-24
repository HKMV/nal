#[test]
fn sangfor() {
    let nal: &dyn crate::core::nal::Nal =
        &crate::core::sangfor::Sangfor::new("http://1.1.1.4/ac_portal/login.php");
    let config = crate::core::nal::LoginConfig {
        username: 2336.to_string(),
        password: 13141.to_string(),
    };
    let is_ok = nal.login_net(&config).unwrap_or_else(|_| false);
    println!(" is_ok: {:?}", is_ok);
    assert_eq!(is_ok, true);
}

#[test]
fn check_net() {
    let is_ok = crate::core::nal::check_net();
    println!(" is_ok: {:?}", is_ok);
    assert_eq!(is_ok, true);
}
