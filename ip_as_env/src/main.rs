use local_ip_address::local_ip;
use winreg::{enums::*, RegKey};
use std::env;

fn main() {
    let my_local_ip = local_ip().unwrap();

    let key = "LocalIp";
    // env::set_var(key, my_local_ip.to_string());
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (env, _) = hkcu.create_subkey("Environment").unwrap(); // create_subkey opens with write permissions
    env.set_value(key, &my_local_ip.to_string()).unwrap();
    // env::set_var(key, my_local_ip.to_string());

    // assert_eq!(env::var(key), Ok(my_local_ip.to_string()));
    // println!("This is my local IP address: {:?}", my_local_ip);
    //get env variable using RegKey
    let env = hkcu.open_subkey(key).unwrap();
    println!("{:?}", env);
    println!("The value {:?} on key: {:?}", env::var_os(key), key);
}
