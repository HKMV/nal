use std::string::String;
use std::str::{from_utf8, from_utf8_mut};
// use rc4::{consts::*, KeyInit, StreamCipher};
// use rc4::{Key, Rc4};

#[test]
fn custom_encode(){
    let key = b"secret_key";
    let mut rc4 = crate::util::rc4::RC4::new(key);

    let mut data = b"hello world".to_vec();
    println!("Original: {:?}", from_utf8(&*data));
    println!("Original: {:?}", data.iter().map(|b| format!("{:02x}", b)).collect::<String>());

    // let mut data = data.as_slice();
    rc4.apply_keystream(&mut data);
    let hex_string = data.iter().map(|b| format!("{:02x}", b)).collect::<String>();
    println!("Encrypted: {:?}", hex_string);

    // 每次使用需要重新创建，不然解密后的数据不对
    let mut rc4 = crate::util::rc4::RC4::new(key);
    let mut encode_data = b"\x2b\x5a\xf7\xcd\xaf\x75\x9c\x49\x3b\x8c\xd0".to_vec(); // 加密后的数据
    rc4.apply_keystream(&mut encode_data);
    println!("Decrypted: {:?}", encode_data.iter().map(|b| format!("{:02x}", b)).collect::<String>());
}

// #rc4 = "0.1.0"
// #[test]
// fn rc4_encode(){
//     let mut rc4 = Rc4::new(b"secret_key".into());
//     let mut data = b"hello world".to_vec();
//     println!(" Original: {:?}", data.iter().map(|b| format!("{:02x}", b)).collect::<String>());
//     rc4.apply_keystream(&mut data);
//     println!("Encrypted: {:?}", data.iter().map(|b| format!("{:02x}", b)).collect::<String>());
//
//     let mut rc4 = Rc4::new(b"secret_key".into());
//     rc4.apply_keystream(&mut data);
//     println!("Decrypted: {:?}", data.iter().map(|b| format!("{:02x}", b)).collect::<String>());
// }